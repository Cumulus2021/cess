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

//! Autogenerated weights for pallet_file_bank
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-initial-devnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// --chain
// cess-initial-devnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_file_bank
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/file-bank/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_file_bank.
pub trait WeightInfo {
	fn upload_filler(v: u32, ) -> Weight;
	fn buy_space() -> Weight;
	fn expansion_space() -> Weight;
	fn renewal_space() -> Weight;
	fn upload_declaration() -> Weight;
	fn upload(v: u32, ) -> Weight;
	fn delete_file() -> Weight;
	fn recover_file() -> Weight;
	fn clear_invalid_file() -> Weight;
	fn create_bucket() -> Weight;
	fn delete_bucket() -> Weight;
	fn ownership_transfer() -> Weight;
}

/// Weights for pallet_file_bank using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank FillerIndexCount (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:1)
	// Storage: FileBank FillerMap (r:1 w:1)
	// Storage: FileBank FillerKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFillerKeysMap (r:1 w:1)
	// Storage: SchedulerCredit CurrentCounters (r:1 w:1)
	fn upload_filler(v: u32, ) -> Weight {
		(11_120_000 as Weight)
			// Standard Error: 1_155_000
			.saturating_add((34_225_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: Sminer PurchasedSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:0)
	// Storage: Sminer TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn buy_space() -> Weight {
		(297_601_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: Sminer PurchasedSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:0)
	// Storage: Sminer TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn expansion_space() -> Weight {
		(3_972_139_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn renewal_space() -> Weight {
		(313_401_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank FileIndexCount (r:1 w:1)
	fn upload_declaration() -> Weight {
		(39_047_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	// Storage: SchedulerCredit CurrentCounters (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank FillerMap (r:3 w:1)
	// Storage: FileBank FillerKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFillerKeysMap (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:1)
	fn upload(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_634_000
			.saturating_add((182_972_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	fn delete_file() -> Weight {
		(376_051_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: FileBank FileRecovery (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	fn recover_file() -> Weight {
		(346_865_000 as Weight)
			// Standard Error: 118_000
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: FileBank InvalidFile (r:1 w:1)
	fn clear_invalid_file() -> Weight {
		(25_341_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn create_bucket() -> Weight {
		(31_818_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn delete_bucket() -> Weight {
		(36_686_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:2 w:2)
	// Storage: FileBank UserOwnedSpace (r:2 w:2)
	// Storage: FileBank UserHoldFileList (r:2 w:2)
	fn ownership_transfer() -> Weight {
		(322_561_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank FillerIndexCount (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:1)
	// Storage: FileBank FillerMap (r:1 w:1)
	// Storage: FileBank FillerKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFillerKeysMap (r:1 w:1)
	// Storage: SchedulerCredit CurrentCounters (r:1 w:1)
	fn upload_filler(v: u32, ) -> Weight {
		(11_120_000 as Weight)
			// Standard Error: 1_155_000
			.saturating_add((34_225_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: Sminer PurchasedSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:0)
	// Storage: Sminer TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	fn buy_space() -> Weight {
		(297_601_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: Sminer PurchasedSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:0)
	// Storage: Sminer TotalServiceSpace (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn expansion_space() -> Weight {
		(3_972_139_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UnitPrice (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn renewal_space() -> Weight {
		(313_401_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank FileIndexCount (r:1 w:1)
	fn upload_declaration() -> Weight {
		(39_047_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	// Storage: SchedulerCredit CurrentCounters (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: FileBank FillerMap (r:3 w:1)
	// Storage: FileBank FillerKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFillerKeysMap (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: Sminer TotalIdleSpace (r:1 w:1)
	fn upload(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_634_000
			.saturating_add((182_972_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	// Storage: FileBank UserHoldFileList (r:1 w:1)
	fn delete_file() -> Weight {
		(376_051_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	// Storage: FileBank FileRecovery (r:1 w:1)
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank UserOwnedSpace (r:1 w:1)
	// Storage: FileBank InvalidFile (r:1 w:1)
	// Storage: Sminer MinerItems (r:1 w:1)
	// Storage: Sminer TotalServiceSpace (r:1 w:1)
	// Storage: FileBank FileKeysMap (r:1 w:1)
	// Storage: FileBank CounterForFileKeysMap (r:1 w:1)
	fn recover_file() -> Weight {
		(346_865_000 as Weight)
			// Standard Error: 118_000
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: FileBank InvalidFile (r:1 w:1)
	fn clear_invalid_file() -> Weight {
		(25_341_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn create_bucket() -> Weight {
		(31_818_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank Bucket (r:1 w:1)
	// Storage: FileBank UserBucketList (r:1 w:1)
	fn delete_bucket() -> Weight {
		(36_686_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: FileBank File (r:1 w:1)
	// Storage: FileBank Bucket (r:2 w:2)
	// Storage: FileBank UserOwnedSpace (r:2 w:2)
	// Storage: FileBank UserHoldFileList (r:2 w:2)
	fn ownership_transfer() -> Weight {
		(322_561_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
}
