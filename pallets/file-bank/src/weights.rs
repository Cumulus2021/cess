
//! Autogenerated weights for `pallet_file_bank`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --output=./pallets/file-bank/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_file_bank`.
pub trait WeightInfo {
	fn cert_idle_space() -> Weight;
	fn upload_declaration(v: u32, ) -> Weight;
	fn transfer_report(v: u32, ) -> Weight;
	fn calculate_report() -> Weight;
	fn replace_idle_space() -> Weight;
	fn delete_file() -> Weight;
	fn create_bucket() -> Weight;
	fn delete_bucket() -> Weight;
	fn generate_restoral_order() -> Weight;
	fn claim_restoral_order() -> Weight;
	fn claim_restoral_noexist_order() -> Weight;
	fn restoral_order_complete() -> Weight;
	fn v03_migration_step() -> Weight;
	fn migration_noop() -> Weight;
	fn on_runtime_upgrade_noop() -> Weight;
	fn on_runtime_upgrade_in_progress() -> Weight;
	fn on_runtime_upgrade() -> Weight;
}

/// Weights for `pallet_file_bank` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalIdleSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalIdleSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn cert_idle_space() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4154`
		//  Estimated: `206644`
		// Minimum execution time: 79_859_000 picoseconds.
		Weight::from_parts(97_178_000, 206644)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `FileBank::File` (r:1 w:0)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::DealMap` (r:0 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833719), added: 836194, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `707`
		//  Estimated: `12492572`
		// Minimum execution time: 28_071_000 picoseconds.
		Weight::from_parts(30_658_375, 12492572)
			// Standard Error: 53_614
			.saturating_add(Weight::from_parts(1_036_572, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::DealMap` (r:1 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833719), added: 836194, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:12 w:12)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalIdleSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalIdleSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalServiceSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalServiceSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserHoldFileList` (r:1 w:1)
	/// Proof: `FileBank::UserHoldFileList` (`max_values`: None, `max_size`: Some(72000052), added: 72002527, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:0 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4996 + v * (833 ±0)`
		//  Estimated: `72003517`
		// Minimum execution time: 111_060_000 picoseconds.
		Weight::from_parts(125_282_482, 72003517)
			// Standard Error: 102_713
			.saturating_add(Weight::from_parts(2_403_862, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(20_u64))
			.saturating_add(T::DbWeight::get().writes(21_u64))
	}
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn calculate_report() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5896`
		//  Estimated: `12492572`
		// Minimum execution time: 111_739_000 picoseconds.
		Weight::from_parts(134_085_000, 12492572)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:1 w:1)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[8, 30]`.
	fn replace_idle_space() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4721`
		//  Estimated: `206644`
		// Minimum execution time: 93_598_000 picoseconds.
		Weight::from_parts(127_838_388, 206644)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:12 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::MinerItems` (r:12 w:12)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalServiceSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalServiceSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserHoldFileList` (r:1 w:1)
	/// Proof: `FileBank::UserHoldFileList` (`max_values`: None, `max_size`: Some(72000052), added: 72002527, mode: `MaxEncodedLen`)
	fn delete_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41192`
		//  Estimated: `72003517`
		// Minimum execution time: 313_412_000 picoseconds.
		Weight::from_parts(342_840_000, 72003517)
			.saturating_add(T::DbWeight::get().reads(29_u64))
			.saturating_add(T::DbWeight::get().writes(17_u64))
	}
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	fn create_bucket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `32036623`
		// Minimum execution time: 23_136_000 picoseconds.
		Weight::from_parts(23_895_000, 32036623)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	fn delete_bucket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398`
		//  Estimated: `32036623`
		// Minimum execution time: 19_279_000 picoseconds.
		Weight::from_parts(19_772_000, 32036623)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	fn generate_restoral_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1757`
		//  Estimated: `12492572`
		// Minimum execution time: 37_904_000 picoseconds.
		Weight::from_parts(43_099_000, 12492572)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:0)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	fn claim_restoral_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4371`
		//  Estimated: `206644`
		// Minimum execution time: 36_747_000 picoseconds.
		Weight::from_parts(40_101_000, 206644)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:0)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:1 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	fn claim_restoral_noexist_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5608`
		//  Estimated: `12492572`
		// Minimum execution time: 47_926_000 picoseconds.
		Weight::from_parts(60_855_000, 12492572)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:2 w:2)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:1 w:1)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:1 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn restoral_order_complete() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9247`
		//  Estimated: `12492572`
		// Minimum execution time: 83_862_000 picoseconds.
		Weight::from_parts(92_463_000, 12492572)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	
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
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalIdleSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalIdleSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn cert_idle_space() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4154`
		//  Estimated: `206644`
		// Minimum execution time: 79_859_000 picoseconds.
		Weight::from_parts(97_178_000, 206644)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `FileBank::File` (r:1 w:0)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::DealMap` (r:0 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833719), added: 836194, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 30]`.
	fn upload_declaration(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `707`
		//  Estimated: `12492572`
		// Minimum execution time: 28_071_000 picoseconds.
		Weight::from_parts(30_658_375, 12492572)
			// Standard Error: 53_614
			.saturating_add(Weight::from_parts(1_036_572, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::DealMap` (r:1 w:1)
	/// Proof: `FileBank::DealMap` (`max_values`: None, `max_size`: Some(833719), added: 836194, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:12 w:12)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalIdleSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalIdleSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalServiceSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalServiceSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserHoldFileList` (r:1 w:1)
	/// Proof: `FileBank::UserHoldFileList` (`max_values`: None, `max_size`: Some(72000052), added: 72002527, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:0 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 30]`.
	fn transfer_report(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4996 + v * (833 ±0)`
		//  Estimated: `72003517`
		// Minimum execution time: 111_060_000 picoseconds.
		Weight::from_parts(125_282_482, 72003517)
			// Standard Error: 102_713
			.saturating_add(Weight::from_parts(2_403_862, 0).saturating_mul(v.into()))
			.saturating_add(RocksDbWeight::get().reads(20_u64))
			.saturating_add(RocksDbWeight::get().writes(21_u64))
	}
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn calculate_report() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5896`
		//  Estimated: `12492572`
		// Minimum execution time: 111_739_000 picoseconds.
		Weight::from_parts(134_085_000, 12492572)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `TeeWorker::Workers` (r:1 w:0)
	/// Proof: `TeeWorker::Workers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::MasterPubkey` (r:1 w:0)
	/// Proof: `TeeWorker::MasterPubkey` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TeeWorker::LastWork` (r:1 w:1)
	/// Proof: `TeeWorker::LastWork` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Sminer::MinerItems` (r:1 w:1)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:1 w:1)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `SchedulerCredit::CurrentCounters` (r:1 w:1)
	/// Proof: `SchedulerCredit::CurrentCounters` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[8, 30]`.
	fn replace_idle_space() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4721`
		//  Estimated: `206644`
		// Minimum execution time: 93_598_000 picoseconds.
		Weight::from_parts(127_838_388, 206644)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:12 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::MinerItems` (r:12 w:12)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::Territory` (r:1 w:1)
	/// Proof: `StorageHandler::Territory` (`max_values`: None, `max_size`: Some(233), added: 2708, mode: `MaxEncodedLen`)
	/// Storage: `StorageHandler::TotalServiceSpace` (r:1 w:1)
	/// Proof: `StorageHandler::TotalServiceSpace` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserHoldFileList` (r:1 w:1)
	/// Proof: `FileBank::UserHoldFileList` (`max_values`: None, `max_size`: Some(72000052), added: 72002527, mode: `MaxEncodedLen`)
	fn delete_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41192`
		//  Estimated: `72003517`
		// Minimum execution time: 313_412_000 picoseconds.
		Weight::from_parts(342_840_000, 72003517)
			.saturating_add(RocksDbWeight::get().reads(29_u64))
			.saturating_add(RocksDbWeight::get().writes(17_u64))
	}
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	fn create_bucket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `32036623`
		// Minimum execution time: 23_136_000 picoseconds.
		Weight::from_parts(23_895_000, 32036623)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::Bucket` (r:1 w:1)
	/// Proof: `FileBank::Bucket` (`max_values`: None, `max_size`: Some(32033158), added: 32035633, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::UserBucketList` (r:1 w:1)
	/// Proof: `FileBank::UserBucketList` (`max_values`: None, `max_size`: Some(64050), added: 66525, mode: `MaxEncodedLen`)
	fn delete_bucket() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `398`
		//  Estimated: `32036623`
		// Minimum execution time: 19_279_000 picoseconds.
		Weight::from_parts(19_772_000, 32036623)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	fn generate_restoral_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1757`
		//  Estimated: `12492572`
		// Minimum execution time: 37_904_000 picoseconds.
		Weight::from_parts(43_099_000, 12492572)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:0)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	fn claim_restoral_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4371`
		//  Estimated: `206644`
		// Minimum execution time: 36_747_000 picoseconds.
		Weight::from_parts(40_101_000, 206644)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:1 w:0)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:1 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	fn claim_restoral_noexist_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5608`
		//  Estimated: `12492572`
		// Minimum execution time: 47_926_000 picoseconds.
		Weight::from_parts(60_855_000, 12492572)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Sminer::MinerItems` (r:2 w:2)
	/// Proof: `Sminer::MinerItems` (`max_values`: None, `max_size`: Some(203179), added: 205654, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::RestoralOrder` (r:1 w:1)
	/// Proof: `FileBank::RestoralOrder` (`max_values`: None, `max_size`: Some(284), added: 2759, mode: `MaxEncodedLen`)
	/// Storage: `FileBank::File` (r:1 w:1)
	/// Proof: `FileBank::File` (`max_values`: None, `max_size`: Some(12489107), added: 12491582, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::PendingReplacements` (r:1 w:1)
	/// Proof: `Sminer::PendingReplacements` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Sminer::RestoralTarget` (r:1 w:0)
	/// Proof: `Sminer::RestoralTarget` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn restoral_order_complete() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9247`
		//  Estimated: `12492572`
		// Minimum execution time: 83_862_000 picoseconds.
		Weight::from_parts(92_463_000, 12492572)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
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
