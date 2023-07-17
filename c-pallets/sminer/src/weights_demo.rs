// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_sminer
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-initial-devnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// pallet
// --chain
// cess-initial-devnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_sminer
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/sminer/src/weights_demo.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_sminer.
pub trait WeightInfo {
	fn regnstk() -> Weight;
}

/// Weights for pallet_sminer using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer CounterForMinerItems (r:1 w:1)
	// Storage: Sminer AllMiner (r:1 w:1)
	// Storage: Sminer RewardMap (r:0 w:1)
	// Storage: Sminer MinerPublicKey (r:0 w:1)
	fn regnstk() -> Weight {
		(1_506_193_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: TeeWorker TeePodr2Pk (r:1 w:0)
	// Storage: Sminer CounterForMinerItems (r:1 w:1)
	// Storage: Sminer AllMiner (r:1 w:1)
	// Storage: Sminer RewardMap (r:0 w:1)
	// Storage: Sminer MinerPublicKey (r:0 w:1)
	fn regnstk() -> Weight {
		(1_506_193_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
}