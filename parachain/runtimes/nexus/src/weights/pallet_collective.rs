
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-08-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Lukambas-M2-MAX`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("nexus-2000")`, DB CACHE: 1024

// Executed Command:
// ./target/release/hyperbridge
// benchmark
// pallet
// --chain=nexus-2000
// --pallet
// pallet_collective
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// parachain/runtimes/nexus/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `TechnicalCollective::Members` (r:1 w:1)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Voting` (r:100 w:100)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Prime` (r:0 w:1)
	/// Proof: `TechnicalCollective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 10]`.
	/// The range of component `n` is `[0, 10]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (309 ±0)`
		//  Estimated: `7119 + m * (1848 ±25) + p * (2643 ±2)`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7119))
			// Standard Error: 314_941
			.saturating_add(Weight::from_parts(7_741_416, 0).saturating_mul(m.into()))
			// Standard Error: 32_192
			.saturating_add(Weight::from_parts(2_937_270, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1848).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2643).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `104 + m * (32 ±0)`
		//  Estimated: `1588 + m * (32 ±0)`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_574_628, 0)
			.saturating_add(Weight::from_parts(0, 1588))
			// Standard Error: 50
			.saturating_add(Weight::from_parts(1_679, 0).saturating_mul(b.into()))
			// Standard Error: 5_382
			.saturating_add(Weight::from_parts(21_171, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:1 w:0)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 10]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `104 + m * (32 ±0)`
		//  Estimated: `3568 + m * (32 ±0)`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(10_687_830, 0)
			.saturating_add(Weight::from_parts(0, 3568))
			// Standard Error: 46
			.saturating_add(Weight::from_parts(1_970, 0).saturating_mul(b.into()))
			// Standard Error: 4_994
			.saturating_add(Weight::from_parts(93_021, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalCount` (r:1 w:1)
	/// Proof: `TechnicalCollective::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Voting` (r:0 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 10]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + m * (32 ±0) + p * (35 ±0)`
		//  Estimated: `3797 + m * (40 ±0) + p * (36 ±0)`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_498_486, 0)
			.saturating_add(Weight::from_parts(0, 3797))
			// Standard Error: 78
			.saturating_add(Weight::from_parts(1_895, 0).saturating_mul(b.into()))
			// Standard Error: 9_271
			.saturating_add(Weight::from_parts(92_557, 0).saturating_mul(m.into()))
			// Standard Error: 807
			.saturating_add(Weight::from_parts(124_532, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Voting` (r:1 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 10]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `974 + m * (64 ±0)`
		//  Estimated: `4439 + m * (64 ±0)`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_920_613, 0)
			.saturating_add(Weight::from_parts(0, 4439))
			// Standard Error: 5_989
			.saturating_add(Weight::from_parts(8_744, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCollective::Voting` (r:1 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `397 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3824 + m * (77 ±1) + p * (37 ±0)`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(16_139_780, 0)
			.saturating_add(Weight::from_parts(0, 3824))
			// Standard Error: 12_758
			.saturating_add(Weight::from_parts(174_755, 0).saturating_mul(m.into()))
			// Standard Error: 848
			.saturating_add(Weight::from_parts(128_552, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 77).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Voting` (r:1 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `742 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4331 + b * (1 ±0) + m * (44 ±2) + p * (41 ±0)`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(24_612_069, 0)
			.saturating_add(Weight::from_parts(0, 4331))
			// Standard Error: 97
			.saturating_add(Weight::from_parts(1_010, 0).saturating_mul(b.into()))
			// Standard Error: 15_006
			.saturating_add(Weight::from_parts(41_820, 0).saturating_mul(m.into()))
			// Standard Error: 1_001
			.saturating_add(Weight::from_parts(143_107, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 44).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Voting` (r:1 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Prime` (r:1 w:0)
	/// Proof: `TechnicalCollective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `417 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `3844 + m * (77 ±1) + p * (37 ±0)`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_900_899, 0)
			.saturating_add(Weight::from_parts(0, 3844))
			// Standard Error: 11_269
			.saturating_add(Weight::from_parts(160_773, 0).saturating_mul(m.into()))
			// Standard Error: 749
			.saturating_add(Weight::from_parts(124_731, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 77).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Voting` (r:1 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Members` (r:1 w:0)
	/// Proof: `TechnicalCollective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Prime` (r:1 w:0)
	/// Proof: `TechnicalCollective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:1 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 10]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `762 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4351 + b * (1 ±0) + m * (44 ±2) + p * (41 ±0)`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_108_360, 0)
			.saturating_add(Weight::from_parts(0, 4351))
			// Standard Error: 108
			.saturating_add(Weight::from_parts(582, 0).saturating_mul(b.into()))
			// Standard Error: 16_765
			.saturating_add(Weight::from_parts(93_868, 0).saturating_mul(m.into()))
			// Standard Error: 1_118
			.saturating_add(Weight::from_parts(143_957, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 44).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(p.into()))
	}
	/// Storage: `TechnicalCollective::Proposals` (r:1 w:1)
	/// Proof: `TechnicalCollective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::Voting` (r:0 w:1)
	/// Proof: `TechnicalCollective::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCollective::ProposalOf` (r:0 w:1)
	/// Proof: `TechnicalCollective::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `260 + p * (32 ±0)`
		//  Estimated: `1745 + p * (32 ±0)`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(10_739_984, 0)
			.saturating_add(Weight::from_parts(0, 1745))
			// Standard Error: 719
			.saturating_add(Weight::from_parts(115_394, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}
