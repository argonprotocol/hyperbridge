use crate::abi::{
	GetRequest, GetResponseMessage, GetTimeoutMessage, IsmpHandler, PostRequest, PostRequestLeaf,
	PostRequestMessage, PostResponse, PostResponseLeaf, PostResponseMessage, PostTimeoutMessage,
	Proof,
};
use anyhow::{anyhow, Error};
use codec::Decode;
use ethers::{
	core::k256::ecdsa::SigningKey,
	middleware::SignerMiddleware,
	prelude::{Provider, Wallet, Ws},
	providers::PendingTransaction,
};
use ismp::{
	host::StateMachine,
	messaging::{Message, ResponseMessage, TimeoutMessage},
	router::{Request, Response},
	SubstrateStateProof,
};
use ismp_rpc::MmrProof;
use merkle_mountain_range_labs::mmr_position_to_k_index;
use pallet_ismp::NodesUtils;
use primitive_types::H160;
use sp_core::H256;
use std::sync::Arc;
use tesseract_primitives::queue::{start_pipeline, PipelineQueue};

/// Use this to initialize the transaction submit queue. This pipelines transaction submission
/// eliminating race conditions.
pub fn init_queue(
	signer: Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
	handler: H160,
	ismp_host: H160,
) -> anyhow::Result<PipelineQueue<Vec<Message>>> {
	let contract = IsmpHandler::new(handler, signer);

	let queue = start_pipeline(move |messages| {
		let contract = contract.clone();
		async move {
			for msg in messages {
				match msg {
					Message::Consensus(msg) => {
						match contract
							.handle_consensus(ismp_host, msg.consensus_proof.into())
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					Message::Request(msg) => {
						let membership_proof =
							match MmrProof::<H256>::decode(&mut msg.proof.proof.as_slice()) {
								Ok(proof) => proof,
								_ => {
									log::error!("Failed to decode membership proof");
									continue
								},
							};
						let mmr_size = NodesUtils::new(membership_proof.leaf_count).size();
						let k_and_leaf_indices = membership_proof
							.leaf_positions_and_indices
							.into_iter()
							.map(|(pos, leaf_index)| {
								let k_index = mmr_position_to_k_index(vec![pos], mmr_size)[0].1;
								(k_index, leaf_index)
							})
							.collect::<Vec<_>>();

						let mut leaves = msg
							.requests
							.into_iter()
							.zip(k_and_leaf_indices)
							.map(|(post, (k_index, leaf_index))| PostRequestLeaf {
								request: PostRequest {
									source: post.source.to_string().as_bytes().to_vec().into(),
									dest: post.dest.to_string().as_bytes().to_vec().into(),
									nonce: post.nonce,
									from: post.from.into(),
									to: post.to.into(),
									timeout_timestamp: post.timeout_timestamp,
									body: post.data.into(),
									gaslimit: post.gas_limit.into(),
								},
								index: leaf_index.into(),
								k_index: k_index.into(),
							})
							.collect::<Vec<_>>();
						leaves.sort_by_key(|leaf| leaf.index);

						let post_message = PostRequestMessage {
							proof: Proof {
								height: crate::abi::ismp_handler::StateMachineHeight {
									state_machine_id: {
										match msg.proof.height.id.state_id {
											StateMachine::Polkadot(id) |
											StateMachine::Kusama(id) => id.into(),
											_ => {
												panic!(
													"Expected polkadot or kusama state machines"
												);
											},
										}
									},
									height: msg.proof.height.height.into(),
								},
								multiproof: membership_proof
									.items
									.into_iter()
									.map(|node| node.0)
									.collect(),
								leaf_count: membership_proof.leaf_count.into(),
							},
							requests: leaves,
						};

						match contract
							.handle_post_requests(ismp_host, post_message)
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					Message::Response(ResponseMessage::Post { responses, proof }) => {
						let membership_proof =
							match MmrProof::<H256>::decode(&mut proof.proof.as_slice()) {
								Ok(proof) => proof,
								_ => {
									log::error!("Failed to decode membership proof");
									continue
								},
							};
						let mmr_size = NodesUtils::new(membership_proof.leaf_count).size();
						let k_and_leaf_indices = membership_proof
							.leaf_positions_and_indices
							.into_iter()
							.map(|(pos, leaf_index)| {
								let k_index = mmr_position_to_k_index(vec![pos], mmr_size)[0].1;
								(k_index, leaf_index)
							})
							.collect::<Vec<_>>();

						let mut leaves = responses
							.into_iter()
							.zip(k_and_leaf_indices)
							.filter_map(|(res, (k_index, leaf_index))| match res {
								Response::Post(res) => Some(PostResponseLeaf {
									response: PostResponse {
										request: PostRequest {
											source: res
												.post
												.source
												.to_string()
												.as_bytes()
												.to_vec()
												.into(),
											dest: res
												.post
												.dest
												.to_string()
												.as_bytes()
												.to_vec()
												.into(),
											nonce: res.post.nonce,
											from: res.post.from.into(),
											to: res.post.to.into(),
											timeout_timestamp: res.post.timeout_timestamp,
											body: res.post.data.into(),
											gaslimit: res.post.gas_limit.into(),
										},
										response: res.response.into(),
									},
									index: leaf_index.into(),
									k_index: k_index.into(),
								}),
								_ => None,
							})
							.collect::<Vec<_>>();
						leaves.sort_by_key(|leaf| leaf.index);

						let message = PostResponseMessage {
							proof: Proof {
								height: crate::abi::ismp_handler::StateMachineHeight {
									state_machine_id: {
										match proof.height.id.state_id {
											StateMachine::Polkadot(id) |
											StateMachine::Kusama(id) => id.into(),
											_ => {
												log::error!(
													"Expected polkadot or kusama state machines"
												);
												continue
											},
										}
									},
									height: proof.height.height.into(),
								},
								multiproof: membership_proof
									.items
									.into_iter()
									.map(|node| node.0)
									.collect(),
								leaf_count: membership_proof.leaf_count.into(),
							},
							responses: leaves,
						};
						match contract
							.handle_post_responses(ismp_host, message)
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					Message::Response(ResponseMessage::Get { requests, proof }) => {
						let requests = match requests
							.into_iter()
							.map(|req| {
								let get = req
									.get_request()
									.map_err(|_| anyhow!("Expected get request"))?;
								Ok(GetRequest {
									source: get.source.to_string().as_bytes().to_vec().into(),
									dest: get.dest.to_string().as_bytes().to_vec().into(),
									nonce: get.nonce,
									from: get.from.into(),
									keys: get.keys.into_iter().map(|key| key.into()).collect(),
									timeout_timestamp: get.timeout_timestamp,
									gaslimit: get.gas_limit.into(),
									height: get.height.into(),
								})
							})
							.collect::<Result<Vec<_>, Error>>()
						{
							Ok(reqs) => reqs,
							Err(err) => {
								log::error!("Failed to error {err:?}");
								continue
							},
						};

						let state_proof: SubstrateStateProof =
							match codec::Decode::decode(&mut proof.proof.as_slice()) {
								Ok(proof) => proof,
								_ => {
									log::error!("Failed to decode membership proof");
									continue
								},
							};
						let message = GetResponseMessage {
							proof: state_proof
								.storage_proof
								.into_iter()
								.map(|key| key.into())
								.collect(),
							height: crate::abi::ismp_handler::StateMachineHeight {
								state_machine_id: {
									match proof.height.id.state_id {
										StateMachine::Polkadot(id) | StateMachine::Kusama(id) =>
											id.into(),
										_ => {
											log::error!(
												"Expected polkadot or kusama state machines"
											);
											continue
										},
									}
								},
								height: proof.height.height.into(),
							},
							requests,
						};

						match contract
							.handle_get_responses(ismp_host, message)
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					Message::Timeout(TimeoutMessage::Post { timeout_proof, requests }) => {
						let post_requests = requests
							.into_iter()
							.filter_map(|req| match req {
								Request::Post(post) => Some(PostRequest {
									source: post.source.to_string().as_bytes().to_vec().into(),
									dest: post.dest.to_string().as_bytes().to_vec().into(),
									nonce: post.nonce,
									from: post.from.into(),
									to: post.to.into(),
									timeout_timestamp: post.timeout_timestamp,
									body: post.data.into(),
									gaslimit: post.gas_limit.into(),
								}),
								Request::Get(_) => None,
							})
							.collect();

						let state_proof: SubstrateStateProof =
							match codec::Decode::decode(&mut timeout_proof.proof.as_slice()) {
								Ok(proof) => proof,
								_ => {
									log::error!("Failed to decode membership proof");
									continue
								},
							};
						let message = PostTimeoutMessage {
							timeouts: post_requests,
							height: crate::abi::ismp_handler::StateMachineHeight {
								state_machine_id: {
									match timeout_proof.height.id.state_id {
										StateMachine::Polkadot(id) | StateMachine::Kusama(id) =>
											id.into(),
										_ => {
											log::error!(
												"Expected polkadot or kusama state machines"
											);
											continue
										},
									}
								},
								height: timeout_proof.height.height.into(),
							},
							proof: state_proof
								.storage_proof
								.into_iter()
								.map(|key| key.into())
								.collect(),
						};

						match contract
							.handle_post_timeouts(ismp_host, message)
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					Message::Timeout(TimeoutMessage::Get { requests }) => {
						let get_requests = requests
							.into_iter()
							.filter_map(|req| match req {
								Request::Get(get) => Some(GetRequest {
									source: get.source.to_string().as_bytes().to_vec().into(),
									dest: get.dest.to_string().as_bytes().to_vec().into(),
									nonce: get.nonce,
									from: get.from.into(),
									keys: get.keys.into_iter().map(|key| key.into()).collect(),
									timeout_timestamp: get.timeout_timestamp,
									gaslimit: get.gas_limit.into(),
									height: get.height.into(),
								}),
								_ => None,
							})
							.collect();

						let message = GetTimeoutMessage { timeouts: get_requests };

						match contract
							.handle_get_timeouts(ismp_host, message)
							.gas(10_000_000)
							.send()
							.await
						{
							Ok(progress) => wait_for_success(progress).await,
							Err(err) => {
								log::error!("Error broadcasting transaction for  {err:?}");
							},
						}
					},
					_ => {
						log::debug!(target: "tesseract", "Message handler not implemented in solidity abi")
					},
				}
			}
		}
	});

	Ok(queue)
}

async fn wait_for_success<'a>(tx: PendingTransaction<'a, Ws>) {
	if let Err(err) = tx.await {
		log::error!("Error broadcasting transaction for  {err:?}");
	}
}
