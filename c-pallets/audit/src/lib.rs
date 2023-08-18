//! # Segemnt Book Module
//!
//!  This file is the exclusive pallet of cess and the proof of podr2 adaptation
//!
//! ## OverView
//!
//!  The job of this segment Book pallet is to process the proof of miner's service file and filling
//! file,  and generate random challenges. Call some traits of Smith pallet to punish miners.
//!  Call the trail of file bank pallet to obtain random files or files with problems in handling
//! challenges.
//!
//! ### Terminology
//!
//! * **random_challenge:** The random time trigger initiates a challenge to the random documents.
//!   The miners need to complete the challenge within a limited time and submit the certificates of
//!   the corresponding documents.
//!
//! * **deadline:** 		Expiration time of challenge, stored in challengeduration
//! * **mu:**				Miner generated challenge related information
//! * **sigma:**			Miner generated challenge related information
//!
//! ### Interface
//!
//! ### Dispatchable Functions
//!
//! * `submit_challange_prove`   Miner submits challenge certificate.
//! * `verify_proof`             Consensus submission verification challenge proof results.
//!
//! ### Scenarios
//!
//! #### Punishment
//!
//!   When the verification result of the miner's certificate is false,
//!   or the miner fails to complete the challenge on time, the miner
//!   will be punished in both cases. Decide whether to reduce power
//!   or space according to the file type of punishment

#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

mod types;
use types::*;

mod constants;
use constants::*;

// pub mod migrations;

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

use sp_runtime::{
	traits::{CheckedAdd, SaturatedConversion},
	RuntimeDebug, Permill,
	offchain::storage::{StorageValueRef, StorageRetrievalError},
};


use codec::{Decode, Encode};
use frame_support::{
	transactional,
	dispatch::DispatchResult,
	pallet_prelude::*,
	storage::bounded_vec::BoundedVec,
	traits::{
		FindAuthor, Randomness, ReservableCurrency, EstimateNextSessionRotation,
		ValidatorSetWithIdentification, ValidatorSet, OneSessionHandler, StorageVersion,
	},
	PalletId, WeakBoundedVec, BoundedSlice,
};
use sp_core::{
	crypto::KeyTypeId,
	offchain::OpaqueNetworkState,
};
use sp_runtime::{Saturating, app_crypto::RuntimeAppPublic};
use frame_system::offchain::{CreateSignedTransaction, SubmitTransaction};
use pallet_file_bank::RandomFileList;
use pallet_tee_worker::TeeWorkerHandler;
use pallet_sminer::MinerControl;
use pallet_storage_handler::StorageHandle;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::{ 
		convert:: { TryFrom, TryInto },
		prelude::*,
		collections::btree_map::BTreeMap,
	};
use cp_enclave_verify::verify_rsa;
use cp_cess_common::*;
pub mod weights;
pub use weights::WeightInfo;
use cp_bloom_filter::BloomFilter;

type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

pub const AUDIT: KeyTypeId = KeyTypeId(*b"cess");
// type FailureRate = u32;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

pub mod sr25519 {
	mod app_sr25519 {
		use crate::*;
		use sp_runtime::app_crypto::{app_crypto, sr25519};
		app_crypto!(sr25519, AUDIT);
	}

	sp_runtime::app_crypto::with_pair! {
		pub type AuthorityPair = app_sr25519::Pair;
	}

	pub type AuthoritySignature = app_sr25519::Signature;

	pub type AuthorityId = app_sr25519::Public;
}

enum OffchainErr {
	Ineligible,
	GenerateInfoError,
	NetworkState,
	FailedSigning,
	Overflow,
	Working,
	SubmitTransactionFailed,
}

impl sp_std::fmt::Debug for OffchainErr {
	fn fmt(&self, fmt: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		match *self {
			OffchainErr::Ineligible => write!(fmt, "The current node does not have the qualification to execute offline working machines"),
			OffchainErr::GenerateInfoError => write!(fmt, "Failed to generate random file meta information"),
			OffchainErr::NetworkState => write!(fmt, "Failed to obtain the network status of the offline working machine"),
			OffchainErr::FailedSigning => write!(fmt, "Signing summary information failed"),
			OffchainErr::Overflow => write!(fmt, "Calculation data, boundary overflow"),
			OffchainErr::Working => write!(fmt, "The offline working machine is currently executing work"),
			OffchainErr::SubmitTransactionFailed => write!(fmt, "Failed to submit transaction."),
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use frame_benchmarking::baseline::Config;
	use frame_support::{traits::Get};
	use frame_system::{ensure_signed, pallet_prelude::*};

	///18446744073709551615
	pub const LIMIT: u64 = u64::MAX;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + sp_std::fmt::Debug + CreateSignedTransaction<Call<Self>>
	{
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		//the weights
		type WeightInfo: WeightInfo;
		#[pallet::constant]
		/// The pallet id
		type MyPalletId: Get<PalletId>;

		#[pallet::constant]
		type SessionKeyMax: Get<u32> + Clone + Eq + PartialEq;

		#[pallet::constant]
		type ChallengeMinerMax: Get<u32> + Clone + Eq + PartialEq;

		#[pallet::constant]
		type VerifyMissionMax: Get<u32> + Clone + Eq + PartialEq;

		#[pallet::constant]
		type SigmaMax: Get<u32> + Clone + Eq + PartialEq;

		#[pallet::constant]
		type IdleTotalHashLength: Get<u32> + Clone + Eq + PartialEq;
		// one day block
		#[pallet::constant]
		type OneDay: Get<BlockNumberOf<Self>>;
		// one hours block
		#[pallet::constant]
		type OneHours: Get<BlockNumberOf<Self>>;
		// randomness for seeds.
		type MyRandomness: Randomness<Option<Self::Hash>, Self::BlockNumber>;
		//Find the consensus of the current block
		type FindAuthor: FindAuthor<Self::AccountId>;
		//Random files used to obtain this batch of challenges
		type File: RandomFileList<Self::AccountId>;
		//Judge whether it is the trait of the consensus node
		type TeeWorkerHandler: TeeWorkerHandler<Self::AccountId>;
		//It is used to increase or decrease the miners' computing power, space, and execute
		// punishment
		type MinerControl: MinerControl<Self::AccountId, Self::BlockNumber>;

		type StorageHandle: StorageHandle<Self::AccountId>;
		//Configuration to be used for offchain worker
		type AuthorityId: Member
			+ Parameter
			+ RuntimeAppPublic
			+ Ord
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;
		//Verifier of this round
		type ValidatorSet: ValidatorSetWithIdentification<Self::AccountId>;
		//Information for the next session
		type NextSessionRotation: EstimateNextSessionRotation<Self::BlockNumber>;
		/// A configuration for base priority of unsigned transactions.
		///
		/// This is exposed so that it can be tuned for particular runtime, when
		/// multiple pallets send unsigned transactions.
		#[pallet::constant]
		type UnsignedPriority: Get<TransactionPriority>;

		#[pallet::constant]
		type LockTime: Get<BlockNumberOf<Self>>;

		#[pallet::constant]
		type ReassignCeiling: Get<u8> + Clone + Eq + PartialEq;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GenerateChallenge,

		SubmitIdleProof { miner: AccountOf<T> },

		SubmitServiceProof { miner: AccountOf<T> },

		SubmitIdleVerifyResult { tee: AccountOf<T>, miner: AccountOf<T>, result: bool },

		SubmitServiceVerifyResult { tee: AccountOf<T>, miner: AccountOf<T>, result: bool },

		VerifyProof { tee_worker: AccountOf<T>, miner: AccountOf<T> },

	}

	/// Error for the audit pallet.
	#[pallet::error]
	pub enum Error<T> {
		//Vec to BoundedVec Error.
		BoundedVecError,
		//Error that the storage has reached the upper LIMIT.
		StorageLimitReached,

		Overflow,
		//The miner submits a certificate, but there is no error in the challenge list
		NoChallenge,
		//Not a consensus node or not registered
		ScheduleNonExistent,
		//The certificate does not exist or the certificate is not verified by this dispatcher
		NonProof,
		//filetype error
		FileTypeError,
		//The user does not have permission to call this method
		NotQualified,
		//Error recording time
		RecordTimeError,

		OffchainSignedTxError,

		NoLocalAcctForSigning,

		LengthExceedsLimit,

		Locked,

		SystemError,

		NonExistentMission,

		UnexpectedError,

		Expired,

		VerifyTeeSigFailed,

		BloomFilterError,

		Submitted,
	}

	//Relevant time nodes for storage challenges
	#[pallet::storage]
	#[pallet::getter(fn challenge_duration)]
	pub(super) type ChallengeDuration<T: Config> = StorageValue<_, BlockNumberOf<T>, ValueQuery>;

	//Relevant time nodes for storage challenges
	#[pallet::storage]
	#[pallet::getter(fn verify_duration)]
	pub(super) type VerifyDuration<T: Config> = StorageValue<_, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cur_authority_index)]
	pub(super) type CurAuthorityIndex<T: Config> = StorageValue<_, u16, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn keys)]
	pub(super) type Keys<T: Config> = StorageValue<_, WeakBoundedVec<T::AuthorityId, T::SessionKeyMax>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn challenge_proposal)]
	pub(super) type ChallengeProposal<T: Config> = CountedStorageMap<_, Blake2_128Concat, [u8; 32], (u32, ChallengeInfo<T>)>;

	#[pallet::storage]
	#[pallet::getter(fn challenge_snap_shot)]
	pub(super) type ChallengeSnapShot<T: Config> = StorageValue<_, ChallengeInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn counted_idle_failed)]
	pub(super) type CountedIdleFailed<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn counted_service_failed)]
	pub(super) type CountedServiceFailed<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn counted_clear)]
	pub(super) type CountedClear<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, u8, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn challenge_era)]
	pub(super) type ChallengeEra<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn unverify_idle_proof)]
	pub(super) type UnverifyIdleProof<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, BoundedVec<IdleProveInfo<T>, T::VerifyMissionMax>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn unverify_service_proof)]
	pub(super) type UnverifyServiceProof<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, BoundedVec<ServiceProveInfo<T>, T::VerifyMissionMax>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn verify_result)]
	pub(super) type VerifyResult<T: Config> = StorageMap<_, Blake2_128Concat, AccountOf<T>, (Option<bool>, Option<bool>)>;

	#[pallet::storage]
	#[pallet::getter(fn verify_reassign_count)]
	pub(super) type VerifyReassignCount<T: Config> = StorageValue<_, u8, ValueQuery>;

	// FOR TESTING
	#[pallet::storage]
	#[pallet::getter(fn lock)]
	pub(super) type Lock<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {
		fn on_initialize(now: BlockNumberOf<T>) -> Weight {
			let weight: Weight = Weight::from_ref_time(0);
			weight
				.saturating_add(Self::clear_challenge(now))
				.saturating_add(Self::clear_verify_mission(now))
		}

		fn offchain_worker(now: T::BlockNumber) {
			let deadline = Self::verify_duration();
			let lock = <Lock<T>>::get();
			if sp_io::offchain::is_validator() {
				if lock {
					if now > deadline {
						//Determine whether to trigger a challenge
						// if Self::trigger_challenge(now) {
							log::info!("offchain worker random challenge start");
							if let Err(e) = Self::offchain_work_start(now) {
								match e {
									OffchainErr::Working => log::info!("offchain working, Unable to perform a new round of work."),
									_ => log::info!("offchain worker generation challenge failed:{:?}", e),
								};
							}
							log::info!("offchain worker random challenge end");
						// }
					}
				}
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[transactional]
		#[pallet::weight(0)]
		pub fn save_challenge_info(
			origin: OriginFor<T>,
			challenge_info: ChallengeInfo<T>,
			_key: T::AuthorityId,
			_seg_digest: SegDigest<BlockNumberOf<T>>,
			_signature: <T::AuthorityId as RuntimeAppPublic>::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			let encode_info: Vec<u8> = challenge_info.encode();

			let hash = sp_io::hashing::sha2_256(&encode_info);

			let count: u32 = Keys::<T>::get().len() as u32;
			let limit = count
				.checked_mul(2).ok_or(Error::<T>::Overflow)?
				.checked_div(3).ok_or(Error::<T>::Overflow)?;

			if ChallengeProposal::<T>::contains_key(&hash) {
				let proposal = ChallengeProposal::<T>::get(&hash).unwrap();
				if proposal.0 + 1 >= limit {
					let cur_blcok = <ChallengeDuration<T>>::get();
					let now = <frame_system::Pallet<T>>::block_number();
					if now > cur_blcok {
						let duration = now.checked_add(&proposal.1.net_snap_shot.life).ok_or(Error::<T>::Overflow)?;
						<ChallengeDuration<T>>::put(duration);
						let one_hour = T::OneHours::get();
						let duration: u32 = (proposal.1.net_snap_shot.total_idle_space
							.checked_add(proposal.1.net_snap_shot.total_service_space).ok_or(Error::<T>::Overflow)?
							.checked_div(IDLE_VERIFY_RATE).ok_or(Error::<T>::Overflow)?) as u32;
						let v_duration = now
							.checked_add(&duration.saturated_into()).ok_or(Error::<T>::Overflow)?
							.checked_add(&one_hour).ok_or(Error::<T>::Overflow)?;
						<VerifyDuration<T>>::put(v_duration);
						<ChallengeSnapShot<T>>::put(proposal.1);
						let _ = ChallengeProposal::<T>::clear(ChallengeProposal::<T>::count(), None);
					}

					Self::deposit_event(Event::<T>::GenerateChallenge);
				}
			} else {
				if ChallengeProposal::<T>::count() > count {
					// Proposal Generally Less
					let _ = ChallengeProposal::<T>::clear(ChallengeProposal::<T>::count(), None);
				} else {
					ChallengeProposal::<T>::insert(
						&hash,
						(1, challenge_info),
					);
				}
			}

			Ok(())
		}

		#[pallet::call_index(1)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn submit_idle_proof(
			origin: OriginFor<T>,
			idle_prove: BoundedVec<u8, T::IdleTotalHashLength>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let miner_snapshot = <ChallengeSnapShot<T>>::try_mutate(|challenge_opt| -> Result<MinerSnapShot<AccountOf<T>, BlockNumberOf<T>>, DispatchError> {
				let challenge_info = challenge_opt.as_mut().ok_or(Error::<T>::NoChallenge)?;
				for (index, miner_snapshot) in challenge_info.miner_snapshot_list.iter_mut().enumerate() {
					if miner_snapshot.miner == sender {
						ensure!(!miner_snapshot.idle_submitted, Error::<T>::Submitted);
						let now = <frame_system::Pallet<T>>::block_number();
						let idle_life = challenge_info.net_snap_shot.start.checked_add(&miner_snapshot.idle_life).ok_or(Error::<T>::Overflow)?;
						ensure!(now < idle_life, Error::<T>::Expired);
						let temp_miner_snap = miner_snapshot.clone();

						miner_snapshot.idle_submitted = true;
						if miner_snapshot.idle_submitted && miner_snapshot.service_submitted {
							<CountedClear<T>>::insert(&sender, u8::MIN);
							challenge_info.miner_snapshot_list.remove(index);
						}
						
						return Ok(temp_miner_snap);
					}
				}

				Err(Error::<T>::NoChallenge)?
			})?;

			let tee_list = T::TeeWorkerHandler::get_controller_list();
			ensure!(tee_list.len() > 0, Error::<T>::SystemError);

			let seed: u32 = <frame_system::Pallet<T>>::block_number().saturated_into();
			let index = Self::random_number(seed) as u32;
			let index: u32 = index % (tee_list.len() as u32);
			let tee_acc = &tee_list[index as usize];

			let prove_info = IdleProveInfo::<T> {
				snap_shot: miner_snapshot,
				idle_prove,
			};

			UnverifyIdleProof::<T>::mutate(tee_acc, |unverify_list| -> DispatchResult {
				unverify_list.try_push(prove_info).map_err(|_| Error::<T>::Overflow)?;

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::SubmitIdleProof { miner: sender });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn submit_service_proof(
			origin: OriginFor<T>,
			service_prove: BoundedVec<u8, T::SigmaMax>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let miner_snapshot = <ChallengeSnapShot<T>>::try_mutate(|challenge_opt| -> Result<MinerSnapShot<AccountOf<T>, BlockNumberOf<T>>, DispatchError> {
				let challenge_info = challenge_opt.as_mut().ok_or(Error::<T>::NoChallenge)?;

				for (index, miner_snapshot) in challenge_info.miner_snapshot_list.iter_mut().enumerate() {
					if miner_snapshot.miner == sender {
						ensure!(!miner_snapshot.service_submitted, Error::<T>::Submitted);
						let now = <frame_system::Pallet<T>>::block_number();
						let service_life = challenge_info.net_snap_shot.start.checked_add(&miner_snapshot.service_life).ok_or(Error::<T>::Overflow)?;
						ensure!(now < service_life, Error::<T>::Expired);
						let temp_miner_snap = miner_snapshot.clone();

						miner_snapshot.service_submitted = true;
						if miner_snapshot.idle_submitted && miner_snapshot.service_submitted {
							<CountedClear<T>>::insert(&sender, u8::MIN);
							challenge_info.miner_snapshot_list.remove(index);
						}
						
						return Ok(temp_miner_snap);
					}
				}

				Err(Error::<T>::NoChallenge)?
			})?;

			let tee_list = T::TeeWorkerHandler::get_controller_list();
			ensure!(tee_list.len() > 0, Error::<T>::SystemError);

			let seed: u32 = <frame_system::Pallet<T>>::block_number().saturated_into();
			let index = Self::random_number(seed) as u32;
			let index: u32 = index % (tee_list.len() as u32);
			let tee_acc = &tee_list[index as usize];

			let prove_info = ServiceProveInfo::<T> {
				snap_shot: miner_snapshot,
				service_prove,
			};

			UnverifyServiceProof::<T>::mutate(tee_acc, |unverify_list| -> DispatchResult {
				unverify_list.try_push(prove_info).map_err(|_| Error::<T>::Overflow)?;

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::SubmitServiceProof { miner: sender });

			Ok(())
		}

		#[pallet::call_index(3)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn submit_verify_idle_result(
			origin: OriginFor<T>,
			total_prove_hash: BoundedVec<u8, T::IdleTotalHashLength>,
			front: u64,
			rear: u64,
			accumulator: Accumulator,
			idle_result: bool,
			signature: TeeRsaSignature,
			tee_acc: AccountOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			<UnverifyIdleProof<T>>::try_mutate(&tee_acc, |unverify_list| -> DispatchResult {
				for (index, miner_info) in unverify_list.iter().enumerate() {
					if &miner_info.snap_shot.miner == &sender {
						let snap_shot = <ChallengeSnapShot<T>>::try_get().map_err(|_| Error::<T>::UnexpectedError)?;

						let verify_idle_result = VerifyIdleResultInfo::<T>{
							miner: sender.clone(),
							miner_prove: total_prove_hash.clone(),
							front: miner_info.snap_shot.space_proof_info.front,
							rear: miner_info.snap_shot.space_proof_info.rear,
							accumulator: miner_info.snap_shot.space_proof_info.accumulator,
							space_challenge_param: snap_shot.net_snap_shot.space_challenge_param,
							result: idle_result,
							tee_acc: tee_acc.clone(),
						};

						let tee_puk = T::TeeWorkerHandler::get_tee_publickey()?;
						let encoding = verify_idle_result.encode();
						let hashing = sp_io::hashing::sha2_256(&encoding);
						ensure!(verify_rsa(&tee_puk, &hashing, &signature), Error::<T>::VerifyTeeSigFailed);

						let idle_result = Self::check_idle_verify_param(idle_result, front, rear, &total_prove_hash, &accumulator, &miner_info);

						if let Ok((_, service_result_opt)) = <VerifyResult<T>>::try_get(&sender).map_err(|_| Error::<T>::UnexpectedError) {
							let service_result = service_result_opt.ok_or(Error::<T>::UnexpectedError)?;
							if idle_result && service_result {
								T::MinerControl::calculate_miner_reward(
									&sender,
									snap_shot.net_snap_shot.total_reward,
									snap_shot.net_snap_shot.total_idle_space,
									snap_shot.net_snap_shot.total_service_space,
									miner_info.snap_shot.idle_space,
									miner_info.snap_shot.service_space,
								)?;
							}

							<VerifyResult<T>>::remove(&sender);
						} else {
							<VerifyResult<T>>::insert(
								&sender,
								(Option::Some(idle_result), Option::<bool>::None),
							);
						}

						if idle_result {
							<CountedIdleFailed<T>>::insert(&sender, u32::MIN);
						} else {
							let count = <CountedIdleFailed<T>>::get(&sender).checked_add(1).unwrap_or(IDLE_FAULT_TOLERANT as u32);
							if count >= IDLE_FAULT_TOLERANT as u32 {
								T::MinerControl::idle_punish(&sender, miner_info.snap_shot.idle_space, miner_info.snap_shot.service_space)?;
							}
							<CountedIdleFailed<T>>::insert(&sender, count);
						}

						unverify_list.remove(index);

						Self::deposit_event(Event::<T>::SubmitIdleVerifyResult { tee: tee_acc.clone(), miner: sender, result: idle_result });

						return Ok(())
					}
				}

				Err(Error::<T>::NonExistentMission)?
			})
		}

		#[pallet::call_index(4)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn submit_verify_service_result(
			origin: OriginFor<T>,
			service_result: bool,
			signature: TeeRsaSignature,
			service_bloom_filter: BloomFilter,
			tee_acc: AccountOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			<UnverifyServiceProof<T>>::try_mutate(&tee_acc, |unverify_list| -> DispatchResult {
				for (index, miner_info) in unverify_list.iter().enumerate() {
					if &miner_info.snap_shot.miner == &sender {
						let snap_shot = <ChallengeSnapShot<T>>::try_get().map_err(|_| Error::<T>::UnexpectedError)?;

						let verify_service_result = VerifyServiceResultInfo::<T>{
							miner: sender.clone(),
							tee_acc: tee_acc.clone(),
							miner_prove: miner_info.service_prove.clone(),
							result: service_result,
							chal: QElement { 
								random_index_list: snap_shot.net_snap_shot.random_index_list,
								random_list: snap_shot.net_snap_shot.random_list,
							},
							service_bloom_filter: service_bloom_filter,
						};

						let tee_puk = T::TeeWorkerHandler::get_tee_publickey()?;
						let encoding = verify_service_result.encode();
						let hashing = sp_io::hashing::sha2_256(&encoding);
						ensure!(verify_rsa(&tee_puk, &hashing, &signature), Error::<T>::VerifyTeeSigFailed);

						ensure!(
							service_bloom_filter == miner_info.snap_shot.service_bloom_filter,
							Error::<T>::BloomFilterError,
						); 

						// Determine whether both proofs have been verified.
						if let Ok((idle_result_opt, _)) = <VerifyResult<T>>::try_get(&sender).map_err(|_| Error::<T>::UnexpectedError) {
							let idle_result = idle_result_opt.ok_or(Error::<T>::UnexpectedError)?;
							// Determine whether to distribute rewards to miners.
							if idle_result && service_result {
								T::MinerControl::calculate_miner_reward(
									&sender,
									snap_shot.net_snap_shot.total_reward,
									snap_shot.net_snap_shot.total_idle_space,
									snap_shot.net_snap_shot.total_service_space,
									miner_info.snap_shot.idle_space,
									miner_info.snap_shot.service_space,
								)?;
							}
							<VerifyResult<T>>::remove(&sender);
						} else {
							<VerifyResult<T>>::insert(
								&sender,
								(Option::<bool>::None, Some(service_result)),
							);
						}

						if service_result {
							<CountedIdleFailed<T>>::insert(&sender, u32::MIN);
						} else {
							let count = <CountedServiceFailed<T>>::get(&sender).checked_add(1).unwrap_or(SERVICE_FAULT_TOLERANT.into());
							if count >= SERVICE_FAULT_TOLERANT as u32 {
								T::MinerControl::service_punish(&sender, miner_info.snap_shot.idle_space, miner_info.snap_shot.service_space)?;
							}
							<CountedServiceFailed<T>>::insert(&sender, count);
						}

						unverify_list.remove(index);

						Self::deposit_event(Event::<T>::SubmitServiceVerifyResult { tee: tee_acc.clone(), miner: sender, result: service_result });

						return Ok(())
					}
				}

				Err(Error::<T>::NonExistentMission)?
			})
		}
		// FOR TESTING
		#[pallet::call_index(5)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn update_lock(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_root(origin)?;

			Lock::<T>::mutate(|lock| *lock = !*lock );

			Ok(())

		}
		// FOR TESTING
		#[pallet::call_index(6)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn update_verify_duration(origin: OriginFor<T>, new: BlockNumberOf<T>) -> DispatchResult {
			let _ = ensure_root(origin)?;

			<VerifyDuration<T>>::put(new);

			Ok(())
		}
		// FOR TESTING
		#[pallet::call_index(7)]
		#[transactional]
		#[pallet::weight(100_000_000)]
		pub fn update_counted_clear(origin: OriginFor<T>, miner: AccountOf<T>) -> DispatchResult {
			let _ = ensure_root(origin)?;

			<CountedClear<T>>::insert(
				&miner, 
				0,
			);

			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::save_challenge_info {
				challenge_info: _,
				key,
				seg_digest,
				signature,			
			} = call {
				Self::check_unsign(key.clone(), &seg_digest, &signature)
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn clear_challenge(now: BlockNumberOf<T>) -> Weight {
			let mut weight: Weight = Weight::from_ref_time(0);
			let duration = <ChallengeDuration<T>>::get();
			weight = weight.saturating_add(T::DbWeight::get().reads(1));
			if now == duration {
				let snap_shot = match <ChallengeSnapShot<T>>::get() {
					Some(snap_shot) => snap_shot,
					None => return weight,
				};
				weight = weight.saturating_add(T::DbWeight::get().reads(1));
				for miner_snapshot in snap_shot.miner_snapshot_list.iter() {
					// unwrap_or(3) 3 Need to match the maximum number of consecutive penalties.
					let count = <CountedClear<T>>::get(&miner_snapshot.miner).checked_add(1).unwrap_or(3);
					weight = weight.saturating_add(T::DbWeight::get().reads(1));

					let _ = T::MinerControl::clear_punish(
						&miner_snapshot.miner, 
						count, 
						miner_snapshot.idle_space, 
						miner_snapshot.service_space
					);
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

					if count >= 3 {
						let result = T::MinerControl::force_miner_exit(&miner_snapshot.miner);
						if result.is_err() {
							log::info!("force clear miner: {:?} failed", miner_snapshot.miner);
						}
						<CountedClear<T>>::remove(&miner_snapshot.miner);
					} else {
						<CountedClear<T>>::insert(
							&miner_snapshot.miner, 
							count,
						);
					}
				}

				weight = weight.saturating_add(T::DbWeight::get().writes(1));
			}

			weight
		}

		fn clear_verify_mission(now: BlockNumberOf<T>) -> Weight {
			let mut weight: Weight = Weight::from_ref_time(0);
			let duration = <VerifyDuration<T>>::get();
			if now == duration {
				let mut reassign_count = <VerifyReassignCount<T>>::get();
				let ceiling = T::ReassignCeiling::get();
				if reassign_count >= ceiling {
					for (acc, unverify_list) in UnverifyIdleProof::<T>::iter() {
						weight = weight.saturating_add(T::DbWeight::get().reads(1));
						if unverify_list.len() > 0 {
							UnverifyIdleProof::<T>::remove(acc);
							weight = weight.saturating_add(T::DbWeight::get().writes(1));
						}
					}
					for (acc, unverify_list) in UnverifyServiceProof::<T>::iter() {
						weight = weight.saturating_add(T::DbWeight::get().reads(1));
						if unverify_list.len() > 0 {
							UnverifyServiceProof::<T>::remove(acc);
							weight = weight.saturating_add(T::DbWeight::get().writes(1));
						}
					}
					<ChallengeSnapShot<T>>::kill();
					weight = weight.saturating_add(T::DbWeight::get().writes(1));
					<VerifyReassignCount<T>>::put(0);
					weight = weight.saturating_add(T::DbWeight::get().writes(1));
					for (miner, _) in VerifyResult::<T>::iter() {
						<VerifyResult<T>>::remove(miner);
						weight = weight.saturating_add(T::DbWeight::get().writes(1));
					}
					return weight;
				} else {
					reassign_count = reassign_count.checked_add(1).unwrap_or(ceiling);
					<VerifyReassignCount<T>>::put(reassign_count);
				}

				let mut seed: u32 = 0;
				// Used to calculate the new validation period.
				let mut mission_count: u32 = 0;
				let mut max_count = 0;
				let tee_list = T::TeeWorkerHandler::get_controller_list();
				let mut reassign_list: BTreeMap<AccountOf<T>, BoundedVec<IdleProveInfo<T>, T::VerifyMissionMax>> = Default::default();

				for (acc, unverify_list) in UnverifyIdleProof::<T>::iter() {
					seed += 1;
					weight = weight.saturating_add(T::DbWeight::get().reads(1));
					if unverify_list.len() > 0 {
						// Count the number of verification tasks that need to be performed.
						mission_count = mission_count.saturating_add(unverify_list.len() as u32);

						let index = Self::random_number(seed) as u32;
						let mut index: u32 = index % (tee_list.len() as u32);
						let mut tee_acc = &tee_list[index as usize];

						if &acc == tee_acc {
							index += 1;
							index = index % (tee_list.len() as u32);
							tee_acc = &tee_list[index as usize];
						}

						if let Some(value) = reassign_list.get_mut(tee_acc) {
							let result = value.try_append(&mut unverify_list.to_vec());

							if result.is_err() {
								let new_block: BlockNumberOf<T> = now.saturating_add(10u32.saturated_into());
								<VerifyDuration<T>>::put(new_block);
								weight = weight.saturating_add(T::DbWeight::get().writes(1));
								return weight;
							}
						} else {
							reassign_list.insert(tee_acc.clone(), unverify_list);
						}

						weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

						UnverifyIdleProof::<T>::remove(acc);
					}
				}

				if mission_count != 0 {
					max_count = mission_count;
					for (acc, unverify_list) in reassign_list {
						let result = UnverifyIdleProof::<T>::mutate(acc, |tar_unverify_list| -> DispatchResult {
							tar_unverify_list.try_append(&mut unverify_list.to_vec()).map_err(|_| Error::<T>::Overflow)?;
							// tar_unverify_list.try_push(mission)
							Ok(())
						});

						if result.is_err() {
							let new_block: BlockNumberOf<T> = now.saturating_add(5u32.saturated_into());
							<VerifyDuration<T>>::put(new_block);
							weight = weight.saturating_add(T::DbWeight::get().writes(1));
							return weight;
						}

						weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
					}
				}

				let mut mission_count: u32 = 0;
				let mut reassign_list: BTreeMap<AccountOf<T>, BoundedVec<ServiceProveInfo<T>, T::VerifyMissionMax>> = Default::default();

				for (acc, unverify_list) in UnverifyServiceProof::<T>::iter() {
					seed += 1;
					weight = weight.saturating_add(T::DbWeight::get().reads(1));
					if unverify_list.len() > 0 {
						// Count the number of verification tasks that need to be performed.
						mission_count = mission_count.saturating_add(unverify_list.len() as u32);

						let index = Self::random_number(seed) as u32;
						let mut index: u32 = index % (tee_list.len() as u32);
						let mut tee_acc = &tee_list[index as usize];

						if &acc == tee_acc {
							index += 1;
							index = index % (tee_list.len() as u32);
							tee_acc = &tee_list[index as usize];
						}

						if let Some(value) = reassign_list.get_mut(tee_acc) {
							let result = value.try_append(&mut unverify_list.to_vec());

							if result.is_err() {
								let new_block: BlockNumberOf<T> = now.saturating_add(10u32.saturated_into());
								<VerifyDuration<T>>::put(new_block);
								weight = weight.saturating_add(T::DbWeight::get().writes(1));
								return weight;
							}
						} else {
							reassign_list.insert(tee_acc.clone(), unverify_list);
						}

						weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));

						UnverifyServiceProof::<T>::remove(acc);
					}
				}

				if mission_count != 0 {
					max_count = mission_count;
					for (acc, unverify_list) in reassign_list {
						let result = UnverifyServiceProof::<T>::mutate(acc, |tar_unverify_list| -> DispatchResult {
							tar_unverify_list.try_append(&mut unverify_list.to_vec()).map_err(|_| Error::<T>::Overflow)?;
							// tar_unverify_list.try_push(mission)
							Ok(())
						});

						if result.is_err() {
							let new_block: BlockNumberOf<T> = now.saturating_add(5u32.saturated_into());
							<VerifyDuration<T>>::put(new_block);
							weight = weight.saturating_add(T::DbWeight::get().writes(1));
							return weight;
						}

						weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
					}
				}

				if max_count == 0 {
					<ChallengeSnapShot<T>>::kill();
					weight = weight.saturating_add(T::DbWeight::get().writes(1));
					<VerifyReassignCount<T>>::put(0);
					weight = weight.saturating_add(T::DbWeight::get().writes(1));
				} else {
					let new_block = max_count.checked_mul(50u32).unwrap_or(u32::MAX.into());
					let new_block = now.checked_add(&new_block.saturated_into()).unwrap_or(u32::MAX.into());
					<VerifyDuration<T>>::put(new_block);
				}
			}

			weight
		}

		fn check_unsign(
			key: T::AuthorityId,
			seg_digest: &SegDigest<BlockNumberOf<T>>,
			signature: &<T::AuthorityId as RuntimeAppPublic>::Signature,
		) -> TransactionValidity {
			let current_session = T::ValidatorSet::session_index();
			let keys = Keys::<T>::get();

			if !keys.contains(&key) {
				return InvalidTransaction::Stale.into();
			} 

			let signature_valid = seg_digest.using_encoded(|encoded_seg_digest| {
				key.verify(&encoded_seg_digest, &signature)
			});

			if !signature_valid {
				log::error!("bad signature.");
				return InvalidTransaction::BadProof.into()
			}

			log::info!("build valid transaction");
			ValidTransaction::with_tag_prefix("Audit")
				.priority(T::UnsignedPriority::get())
				.and_provides((current_session, key, signature))
				.longevity(
					TryInto::<u64>::try_into(
						T::NextSessionRotation::average_session_length() / 2u32.into(),
					)
					.unwrap_or(64_u64),
				)
				.propagate(true)
				.build()
		}

		//Record challenge time
		fn _record_challenge_time(duration: BlockNumberOf<T>) -> DispatchResult {
			let now = <frame_system::Pallet<T>>::block_number();
			let verify_deadline = now
				.checked_add(&duration)
				.ok_or(Error::<T>::Overflow)?
				.checked_add(&2000u32.saturated_into())
				.ok_or(Error::<T>::Overflow)?;
			<VerifyDuration<T>>::try_mutate(|o| -> DispatchResult {
				*o = verify_deadline;
				Ok(())
			})?;
			<ChallengeDuration<T>>::try_mutate(|o| -> DispatchResult {
				*o = now.checked_add(&duration).ok_or(Error::<T>::Overflow)?;
				Ok(())
			})?;
			Ok(())
		}

		//Trigger: whether to trigger the challenge
		fn trigger_challenge(now: BlockNumberOf<T>) -> bool {
			const START_FINAL_PERIOD: Permill = Permill::from_percent(80);

			let time_point = Self::random_number(20220509);
			//The chance to trigger a challenge is once a day
			let probability: u32 = T::OneDay::get().saturated_into();
			// A fixed value that will not overflow.
			let range = LIMIT / probability as u64 * 10;
			if (time_point > 2190502) && (time_point < (range + 2190502)) {
				if let (Some(progress), _) =
				T::NextSessionRotation::estimate_current_session_progress(now) {
					if progress >= START_FINAL_PERIOD {
						log::error!("TooLate!");
						return false;
					}
				}
				return true;
			}
			false
		}

		fn offchain_work_start(now: BlockNumberOf<T>) -> Result<(), OffchainErr> {
			log::info!("get loacl authority...");
			let (authority_id, _validators_len) = Self::get_authority()?;
			log::info!("get loacl authority success!");
			if !Self::check_working(&now, &authority_id) {
				Self::unlock_offchain(&authority_id);
				return Err(OffchainErr::Working);
			}
			log::info!("get challenge data...");
			let challenge_info = Self::generation_challenge(now).map_err(|e| {
				Self::unlock_offchain(&authority_id);
				log::error!("generation challenge error:{:?}", e);
				OffchainErr::GenerateInfoError
			})?;
			log::info!("get challenge success!");
			log::info!("submit challenge to chain...");
			Self::offchain_call_extrinsic(now, authority_id.clone(), challenge_info)?;
			log::info!("submit challenge to chain!");
			Self::unlock_offchain(&authority_id);

			Ok(())
		}

		fn check_working(now: &BlockNumberOf<T>, authority_id: &T::AuthorityId) -> bool {
			let key = &authority_id.encode();
			let storage = StorageValueRef::persistent(key);

			let res = storage.mutate(|status: Result<Option<BlockNumberOf<T>>, StorageRetrievalError>| {
				match status {
					// we are still waiting for inclusion.
					Ok(Some(last_block)) => {
						let lock_time = T::LockTime::get();
						// Based on human time, there is no possibility of overflow here
						if last_block + lock_time > *now {
							log::info!("last_block: {:?}, lock_time: {:?}, now: {:?}", last_block, lock_time, now);
							Err(OffchainErr::Working)
						} else {
							Ok(*now)
						}
					},
					// attempt to set new status
					_ => Ok(*now),
				}
			});

			if res.is_err() {
				log::error!("offchain work: {:?}", OffchainErr::Working);
				return false
			}

			true
		}

		fn unlock_offchain(authority_id: &T::AuthorityId) {
			let key = &authority_id.encode();
			let mut storage = StorageValueRef::persistent(key);

			storage.clear();
		}

		fn get_authority() -> Result<(T::AuthorityId, usize), OffchainErr> {
			let validators = Keys::<T>::get();

			let mut local_keys = T::AuthorityId::all();

			if local_keys.len() == 0 {
				log::info!("no local_keys");
				return Err(OffchainErr::Ineligible);
			}

			local_keys.sort();
			// Find eligible keys locally.
			for key in validators.iter() {
				let res = local_keys.binary_search(key);

				let authority_id = match res {
					Ok(index) => local_keys.get(index),
					Err(_e) => continue,
				};

				if let Some(authority_id) = authority_id {
					return Ok((authority_id.clone(), validators.len()));
				}
			}

			Err(OffchainErr::Ineligible)
		}

		fn generation_challenge(now: BlockNumberOf<T>) 
			-> Result<ChallengeInfo<T>, OffchainErr> 
		{
			// let miner_count = T::MinerControl::get_miner_count();
			let allminer = T::MinerControl::get_all_miner().map_err(|_| OffchainErr::GenerateInfoError)?;
			let miner_count = allminer.len() as u32;
			if miner_count == 0 {
				Err(OffchainErr::GenerateInfoError)?;
			}

			// FOR TESTING
			let need_miner_count = miner_count;
			// let need_miner_count = miner_count / 10 + 1;

			let mut miner_list: BoundedVec<MinerSnapShot<AccountOf<T>, BlockNumberOf<T>>, T::ChallengeMinerMax> = Default::default();

			let mut valid_index_list: Vec<u32> = Default::default();

			let mut total_idle_space: u128 = u128::MIN;
			let mut total_service_space: u128 = u128::MIN;
			let mut max_life: u32 = 0;
			let mut max_space: u128 = 0;

			// TODO: need to set a maximum number of cycles
			let mut seed: u32 = 20230601;
			while ((miner_list.len() as u32) < need_miner_count) && (valid_index_list.len() as u32 != miner_count) {
				seed = seed.saturating_add(1); 
				let index_list = Self::random_select_miner(need_miner_count, miner_count, &valid_index_list, seed);

				for index in index_list {
					valid_index_list.push(index);
					let miner = allminer[index as usize].clone();
					let state = T::MinerControl::get_miner_state(&miner).map_err(|_| OffchainErr::GenerateInfoError)?;
					if state == "lock".as_bytes().to_vec() || state == "offline".as_bytes().to_vec() || state == "exit".as_bytes().to_vec() {
						continue;
					}

					let (idle_space, service_space, service_bloom_filter, space_proof_info, tee_signature) = T::MinerControl::get_miner_snapshot(&miner).map_err(|_| OffchainErr::GenerateInfoError)?;

					if (idle_space == 0) && (service_space == 0) {
						continue;
					}

					let idle_life: u32 = (idle_space
						.checked_div(IDLE_PROVE_RATE).ok_or(OffchainErr::Overflow)?
						.checked_add(50).ok_or(OffchainErr::Overflow)?
					) as u32;

					if idle_life > max_life {
						max_life = idle_life;
					}

					let service_life: u32 = (service_space
						.checked_div(SERVICE_PROVE_RATE).ok_or(OffchainErr::Overflow)?
						.checked_add(50).ok_or(OffchainErr::Overflow)?
					) as u32;

					if service_life > max_life {
						max_life = service_life;
					}

					total_idle_space = total_idle_space.checked_add(idle_space).ok_or(OffchainErr::Overflow)?;
					total_service_space = total_service_space.checked_add(service_space).ok_or(OffchainErr::Overflow)?;

					let miner_snapshot = MinerSnapShot::<AccountOf<T>, BlockNumberOf<T>> {
						miner,
						idle_life: idle_life.saturated_into(),
						service_life: service_life.saturated_into(),
						idle_space,
						service_space,
						idle_submitted: false,
						service_submitted: false,
						service_bloom_filter,
						space_proof_info,
						tee_signature,
					};

					if let Err(_e) = miner_list.try_push(miner_snapshot) {
						return Err(OffchainErr::GenerateInfoError)?;
					};

					if (miner_list.len() as u32) >= need_miner_count {
						break;
					}
				}
			}

			// generate service challenge param
			let mut random_index_list: Vec<u32> = Default::default();
			let need_count = CHUNK_COUNT * 46 / 1000;
			let mut seed: u32 = u32::MIN;
			while random_index_list.len() < need_count as usize {
				seed = seed.checked_add(1).ok_or(OffchainErr::Overflow)?;
				let random_index = (Self::random_number(seed) % CHUNK_COUNT as u64) as u32;
				if !random_index_list.contains(&random_index) {
					random_index_list.push(random_index);
				}
			}

			let mut random_list: Vec<[u8; 20]> = Default::default();
			let mut seed: u32 = now.saturated_into();
			while random_list.len() < need_count as usize {
				seed = seed.checked_add(1).ok_or(OffchainErr::Overflow)?;
				let random_number = Self::generate_challenge_random(seed);
				if !random_list.contains(&random_number) {
					random_list.push(random_number);
				}
			}

			// generate idle challenge param
			let (_, n, d) = T::MinerControl::get_expenders().map_err(|_| OffchainErr::GenerateInfoError)?;
			let mut space_challenge_param: SpaceChallengeParam = Default::default();
			let mut repeat_filter: Vec<u64> = Default::default();
			let mut seed_multi: u32 = 10000;
			let mut seed: u32 = 1;
			for elem in &mut space_challenge_param {
				loop {
					let random = Self::random_number(seed.checked_add(seed_multi).ok_or(OffchainErr::Overflow)?) % n;
					let random = n
						.checked_mul(d).ok_or(OffchainErr::Overflow)?
						.checked_add(random).ok_or(OffchainErr::Overflow)?;
					if repeat_filter.contains(&random) {
						continue;
					}
					repeat_filter.push(random);
					*elem = random;
					seed = seed.checked_add(1).ok_or(OffchainErr::Overflow)?;
					break;
				}
				seed_multi = seed_multi.checked_add(10000).ok_or(OffchainErr::Overflow)?;
			}

			let total_reward: u128 = T::MinerControl::get_reward() / 6;
			let snap_shot = NetSnapShot::<BlockNumberOf<T>>{
				start: now,
				life: max_life.into(),
				total_reward,
				total_idle_space,
				total_service_space,
				random_index_list: random_index_list.try_into().map_err(|_| OffchainErr::GenerateInfoError)?,
				random_list: random_list.try_into().map_err(|_| OffchainErr::GenerateInfoError)?,
				space_challenge_param,
			};

			Ok( ChallengeInfo::<T>{ net_snap_shot: snap_shot, miner_snapshot_list: miner_list } )
		}

		// Ensure that the length is not 0
		fn random_select_miner(need: u32, length: u32, valid_index_list: &Vec<u32>, seed: u32) -> Vec<u32> {
			let mut miner_index_list: Vec<u32> = Default::default();
			let mut seed: u32 = seed.saturating_mul(5000);
			// In theory, unless the number of registered miners reaches 400 million, there is no possibility of overflow.
			while (miner_index_list.len() as u32) < need && ((valid_index_list.len() + miner_index_list.len()) as u32 != length) {
				seed += 1;
				let index = Self::random_number(seed);
				let index: u32 = (index % length as u64) as u32;

				if valid_index_list.contains(&index) {
					continue;
				}

				if !miner_index_list.contains(&index) {
					miner_index_list.push(index);
				}
			}

			miner_index_list
		}

		fn offchain_call_extrinsic(
			now: BlockNumberOf<T>,
			authority_id: T::AuthorityId,
			challenge_info: ChallengeInfo<T>,
		) -> Result<(), OffchainErr> {

			let (signature, digest) = Self::offchain_sign_digest(now, &authority_id)?;

			let call = Call::save_challenge_info {
							challenge_info,
							seg_digest: digest,
							signature: signature,
							key: authority_id,
						};
		
			let result = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());

			if let Err(e) = result {
				log::error!("{:?}", e);
				return Err(OffchainErr::SubmitTransactionFailed);
			}

			Ok(())
		}

		fn offchain_sign_digest(
			now: BlockNumberOf<T>,
			authority_id: &T::AuthorityId,
		) -> Result< (<<T as pallet::Config>::AuthorityId as sp_runtime::RuntimeAppPublic>::Signature, SegDigest::<BlockNumberOf<T>>), OffchainErr> {

			let network_state =
				sp_io::offchain::network_state().map_err(|_| OffchainErr::NetworkState)?;

			let author_len = Keys::<T>::get().len();

			let digest = SegDigest::<BlockNumberOf<T>>{
				validators_len: author_len as u32,
				block_num: now,
				network_state,
			};

			let signature = authority_id.sign(&digest.encode()).ok_or(OffchainErr::FailedSigning)?;

			Ok((signature, digest))
		}

		pub fn initialize_keys(keys: &[T::AuthorityId]) {
			if !keys.is_empty() {
				assert!(Keys::<T>::get().is_empty(), "Keys are already initialized!");
				let bounded_keys = <BoundedSlice<'_, _, T::SessionKeyMax>>::try_from(keys)
					.expect("More than the maximum number of keys provided");
				Keys::<T>::put(bounded_keys);
			}
		}

		// Generate a random number from a given seed.
		pub fn random_number(seed: u32) -> u64 {
			let (random_seed, _) = T::MyRandomness::random(&(T::MyPalletId::get(), seed).encode());
			let random_seed = match random_seed {
				Some(v) => v,
				None => Default::default(),
			};
			let random_number = <u64>::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u32; qed");
			random_number
		}

		//The number of pieces generated is vec
		fn generate_challenge_random(seed: u32) -> [u8; 20] {
			let mut increase = seed;
			loop {
				increase += 1;
				let (r_seed, _) =
					T::MyRandomness::random(&(T::MyPalletId::get(), increase).encode());
				let r_seed = match r_seed {
					Some(v) => v,
					None => Default::default(),
				};
				let random_seed = <H256>::decode(&mut r_seed.as_ref())
					.expect("secure hashes should always be bigger than u32; qed");
				let random_vec = random_seed.as_bytes().to_vec();
				if random_vec.len() >= 20 {
					return random_vec[0..20].try_into().unwrap();
				}
			}
		}

		fn check_idle_verify_param(
			mut idle_result: bool, 
			front: u64, 
			rear: u64, 
			total_prove_hash: &BoundedVec<u8, T::IdleTotalHashLength>,
			accumulator: &Accumulator, 
			miner_info: &IdleProveInfo<T>,
		) -> bool {

			if accumulator != &miner_info.snap_shot.space_proof_info.accumulator {
				idle_result = false
			}

			if rear != miner_info.snap_shot.space_proof_info.rear {
				idle_result = false
			}

			if front != miner_info.snap_shot.space_proof_info.front {
				idle_result = false
			}

			if total_prove_hash != &miner_info.idle_prove {
				idle_result = false
			}

			idle_result
		}
	}
}

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
	type Public = T::AuthorityId;
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = T::AuthorityId;

	fn on_genesis_session<'a, I: 'a>(validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		let keys = validators.map(|x| x.1).collect::<Vec<_>>();
		Self::initialize_keys(&keys);
	}

	fn on_new_session<'a, I: 'a>(_changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		// Tell the offchain worker to start making the next session's heartbeats.
		// Since we consider producing blocks as being online,
		// the heartbeat is deferred a bit to prevent spamming.

		// Remember who the authorities are for the new session.
		let keys = validators.map(|x| x.1).collect::<Vec<_>>();
		let bounded_keys = WeakBoundedVec::<_, T::SessionKeyMax>::force_from(
			keys,
			Some(
				"Warning: The session has more keys than expected. \
  				A runtime configuration adjustment may be needed.",
			),
		);
		Keys::<T>::put(bounded_keys);
	}

	fn on_before_session_ending() {
		// ignore
	}

	fn on_disabled(_i: u32) {
		// ignore
	}
}