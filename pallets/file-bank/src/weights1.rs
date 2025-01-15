
//! Autogenerated weights for `pallet_file_bank`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2024-12-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ytqaljn-virtual-machine`, CPU: `12th Gen Intel(R) Core(TM) i5-12400`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("cess-initial-testnet")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/cess-node
// benchmark
// pallet
// --chain
// cess-initial-testnet
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
// --output=./pallets/file-bank/src/weights1.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_file_bank`.
pub trait WeightInfo {
	fn v03_migration_step() -> Weight;
	fn migration_noop() -> Weight;
	fn on_runtime_upgrade_noop() -> Weight;
	fn on_runtime_upgrade_in_progress() -> Weight;
	fn on_runtime_upgrade() -> Weight;
}

/// Weights for `pallet_file_bank` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `FileBank::File` (r:2 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(9289107), added: 9291582, mode: `Measured`)
	/// Storage: `FileBank::DealMap` (r:2 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833655), added: 836130, mode: `Measured`)
	fn v03_migration_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2666`
		//  Estimated: `8606`
		// Minimum execution time: 23_442_000 picoseconds.
		Weight::from_parts(31_331_000, 8606)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::MigrationInProgress` (r:1 w:1)
	/// Proof: `FileBank::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `MaxEncodedLen`)
	fn migration_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `2511`
		// Minimum execution time: 3_591_000 picoseconds.
		Weight::from_parts(3_973_000, 2511)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	fn on_runtime_upgrade_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3711`
		// Minimum execution time: 5_602_000 picoseconds.
		Weight::from_parts(6_747_000, 3711)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	fn on_runtime_upgrade_in_progress() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `3732`
		// Minimum execution time: 4_107_000 picoseconds.
		Weight::from_parts(4_577_000, 3732)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Storage: `FileBank::MigrationInProgress` (r:1 w:1)
	/// Proof: `FileBank::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	fn on_runtime_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3711`
		// Minimum execution time: 5_252_000 picoseconds.
		Weight::from_parts(5_961_000, 3711)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `FileBank::File` (r:2 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(9289107), added: 9291582, mode: `Measured`)
	/// Storage: `FileBank::DealMap` (r:2 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833655), added: 836130, mode: `Measured`)
	fn v03_migration_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2666`
		//  Estimated: `8606`
		// Minimum execution time: 23_442_000 picoseconds.
		Weight::from_parts(31_331_000, 8606)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::MigrationInProgress` (r:1 w:1)
	/// Proof: `FileBank::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `MaxEncodedLen`)
	fn migration_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `2511`
		// Minimum execution time: 3_591_000 picoseconds.
		Weight::from_parts(3_973_000, 2511)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	fn on_runtime_upgrade_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3711`
		// Minimum execution time: 5_602_000 picoseconds.
		Weight::from_parts(6_747_000, 3711)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	fn on_runtime_upgrade_in_progress() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `267`
		//  Estimated: `3732`
		// Minimum execution time: 4_107_000 picoseconds.
		Weight::from_parts(4_577_000, 3732)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xa3c62a50675125d74bc9f59e3f75adc94e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	/// Storage: `FileBank::MigrationInProgress` (r:1 w:1)
	/// Proof: `FileBank::MigrationInProgress` (`max_values`: Some(1), `max_size`: Some(1026), added: 1521, mode: `Measured`)
	fn on_runtime_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3711`
		// Minimum execution time: 5_252_000 picoseconds.
		Weight::from_parts(5_961_000, 3711)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}