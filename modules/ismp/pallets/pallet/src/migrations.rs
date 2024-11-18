// Copyright (c) 2024 Polytope Labs.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// File for migrations for the ISMP pallet

use crate::{Config, Coprocessor};
use core::marker::PhantomData;
use frame_support::__private::Get;
use ismp::host::StateMachine;

/// Move the Coprocessor from the runtime config to storage during runtime upgrade
pub struct MoveCoprocessorToStorage<T: Config, P: Get<Option<StateMachine>>>(PhantomData<(T, P)>);
impl<T: Config, P: Get<Option<StateMachine>>> frame_support::traits::OnRuntimeUpgrade
	for MoveCoprocessorToStorage<T, P>
{
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// Set the new storage value for Coprocessor using the provided value
		if let Some(coprocessor) = P::get() {
			Coprocessor::<T>::set(Some(coprocessor));
		}
		T::DbWeight::get().writes(1) // Return the weight of one storage write
	}
}
