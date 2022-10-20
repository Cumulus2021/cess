#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::*,
	traits::ValidatorCredits,
	weights::Weight,
};
use log::{debug, warn};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{SaturatedConversion, Zero},
	Percent,
	RuntimeDebug,
};
use sp_std::{collections::btree_map::BTreeMap, prelude::*};

use cp_scheduler_credit::{SchedulerCreditCounter, SchedulerStashAccountFinder};

pub use pallet::*;

pub type EpochIndex = u64;
pub type CreditScore = u32;

pub const FULL_CREDIT_SCORE: u32 = 1000;
const LOG_TARGET: &str = "scheduler-credit";
/// The weight of credit value when figure credit score.
/// period n-1 to period n-5
const PERIOD_WEIGHT: [Percent; 5] = [
	Percent::from_percent(50),
	Percent::from_percent(20),
	Percent::from_percent(15),
	Percent::from_percent(10),
	Percent::from_percent(5),
];

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct SchedulerCounterEntry {
	pub proceed_block_size: u64,
	pub punishment_count: u32,
}

impl SchedulerCounterEntry {
	pub fn increase_block_size(&mut self, block_size: u64) {
		self.proceed_block_size += block_size;
	}

	pub fn increase_punishment_count(&mut self) {
		self.punishment_count += 1;
	}

	pub fn figure_credit_score(&self, total_block_size: u64) -> CreditScore {
		if total_block_size != 0 {
			let a = (FULL_CREDIT_SCORE as f64 *
				(self.proceed_block_size as f64 / total_block_size as f64)) as u32;
			return a.saturating_sub(self.punishment_part())
		}
		return 0
	}

	fn punishment_part(&self) -> u32 {
		if self.punishment_count != 0 {
			return (10 * self.punishment_count).pow(2)
		}
		return 0
	}
}

impl Default for SchedulerCounterEntry {
	fn default() -> Self {
		SchedulerCounterEntry { proceed_block_size: 0_u64, punishment_count: 0_u32 }
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(sp_std::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {

		#[pallet::constant]
		type PeriodDuration: Get<Self::BlockNumber>;

		/// Number of periods to keep in history.
		#[pallet::constant]
		type HistoryDepth: Get<u32>;

		type StashAccountFinder: SchedulerStashAccountFinder<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn current_scheduler_credits)]
	pub(super) type CurrentCounters<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, SchedulerCounterEntry, ValueQuery>;

	/// Stores the latest `HistoryDepth` preiods credit values.
	#[pallet::storage]
	pub(super) type HistoryCreditValues<T: Config> =
		StorageDoubleMap<_, Twox64Concat, u32, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			let period_duration = T::PeriodDuration::get();
			if now % period_duration == T::BlockNumber::zero() {
				let period: u32 = (now / period_duration).saturated_into();
				Self::figure_credit_values(period.saturating_sub(1));
			}
			0
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn record_proceed_block_size(scheduler_id: &T::AccountId, block_size: u64) {
		<CurrentCounters<T>>::mutate(scheduler_id, |scb| scb.increase_block_size(block_size));
	}

	pub fn record_punishment(scheduler_id: &T::AccountId) {
		<CurrentCounters<T>>::mutate(scheduler_id, |scb| scb.increase_punishment_count());
	}

	pub fn figure_credit_values(period: u32) {
		let mut total_size = 0_u64;
		for (_, counter_entry) in <CurrentCounters<T>>::iter() {
			total_size += counter_entry.proceed_block_size;
		}

		for (ctrl_account_id, counter_entry) in <CurrentCounters<T>>::iter() {
			let credit_score = counter_entry.figure_credit_score(total_size);
			debug!(
				target: LOG_TARGET,
				"scheduler stash account: {:?}, credit: {}",
				ctrl_account_id,
				credit_score
			);
			HistoryCreditValues::<T>::insert(&period, &ctrl_account_id, credit_score);
		}

		//Remove `period - HistoryDepth` credit values in history.
		let history_depth = T::HistoryDepth::get();
		if period > history_depth {
			HistoryCreditValues::<T>::remove_prefix(&period.saturating_sub(history_depth), None);
		}
	}

	pub fn figure_credit_scores() -> BTreeMap<T::AccountId, CreditScore> {
		let mut result = BTreeMap::new();
		let now = <frame_system::Pallet<T>>::block_number();
		let period_duration = T::PeriodDuration::get();
		let period: u32 = (now / period_duration).saturated_into();
		let last_period = period.saturating_sub(1);
		HistoryCreditValues::<T>::iter_key_prefix(&last_period)
			.for_each(|ctrl_account_id| {
				if let Some(stash_account_id) =
					T::StashAccountFinder::find_stash_account_id(&ctrl_account_id)
				{
					let mut credit_score = 0_u32;
					for (index, weight) in PERIOD_WEIGHT.into_iter().enumerate() {
						if last_period > index as u32 {
							let credit_value = HistoryCreditValues::<T>::try_get(&last_period.saturating_sub(index as u32), &ctrl_account_id)
								.unwrap_or(0);
							credit_score += weight * credit_value;
						}
					}
					result.insert(stash_account_id, credit_score);
				} else {
					warn!(
						target: LOG_TARGET,
						"can not find the scheduler stash account for the controller account: {:?}",
						ctrl_account_id
					);
				}
		});
		result
	}
}

impl<T: Config> SchedulerCreditCounter<T::AccountId> for Pallet<T> {
	fn record_proceed_block_size(scheduler_id: &T::AccountId, block_size: u64) {
		Pallet::<T>::record_proceed_block_size(scheduler_id, block_size);
	}

	fn record_punishment(scheduler_id: &T::AccountId) {
		Pallet::<T>::record_punishment(scheduler_id);
	}
}

impl<T: Config> ValidatorCredits<T::AccountId> for Pallet<T> {
	fn full_credit() -> u32 {
		FULL_CREDIT_SCORE
	}

	fn credits(epoch_index: u64) -> BTreeMap<T::AccountId, CreditScore> {
		debug!(target: LOG_TARGET, "begin fetch schedulers credit on epoch: {}", epoch_index);
		Pallet::<T>::figure_credit_scores()
	}
}

#[cfg(test)]
mod test {
	use crate::SchedulerCounterEntry;

	#[test]
	fn scheduler_counter_works() {
		let mut sce = SchedulerCounterEntry::default();
		sce.increase_block_size(100);
		assert_eq!(100, sce.proceed_block_size);
		sce.increase_block_size(100);
		assert_eq!(200, sce.proceed_block_size);
		assert_eq!(0, sce.punishment_part());
		assert_eq!(100, sce.figure_credit_score(2000));

		sce.increase_punishment_count();
		assert_eq!(1, sce.punishment_count);
		assert_eq!(100, sce.figure_credit_score(1000));
		sce.increase_punishment_count();
		assert_eq!(2, sce.punishment_count);
		assert_eq!(0, sce.figure_credit_score(1000));
	}
}
