#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
    traits::{
        schedule::{v3::Named as ScheduleNamed, DispatchTime, LOWEST_PRIORITY},
        Bounded, ConstU32, Get, QueryPreimage, Randomness, StorePreimage,
    },
    weights::Weight,
    BoundedVec,
};
use parity_scale_codec::Encode;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::Dispatchable;

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

const KEY_ROTATE_ID: [u8; 12] = *b"SymmetricKey";
const KEY_RANDOM_ID: [u8; 13] = *b"SYMMETRIC_KEY";

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod migrations;
pub mod weights;

pub use weights::WeightInfo;

type CallOf<T> = <T as Config>::RuntimeCall;
type BoundedCallOf<T> = Bounded<<T as Config>::RuntimeCall, <T as frame_system::Config>::Hashing>;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // The runtime call type which can be constructed from a call in this pallet
        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + From<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>
            + From<frame_system::Call<Self>>;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type KeyLength: Get<u32>;

        /// The origin which can update the key
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        /// The origin which can rotate the key
        type RotateOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        /// Source of randomness when generating new keys.
        /// In production this should come from a secure source such as the Babe pallet
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        #[pallet::constant]
        type RefreshPeriod: Get<BlockNumberFor<Self>>;
        /// Overarching type of all pallets origins.
        type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;
        /// The Scheduler.
        type Scheduler: ScheduleNamed<BlockNumberFor<Self>, CallOf<Self>, Self::PalletsOrigin, Hasher = Self::Hashing>;
        /// The Preimage provider.
        type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;

        type WeightInfo: WeightInfo;
    }

    /// The in-code storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: BlockNumberFor<T>) -> frame_support::weights::Weight {
            use sp_runtime::traits::Zero;

            let existing_schedule = <KeyScheduleId<T>>::get();

            match existing_schedule {
                None => {
                    let id = blake2_256(&KEY_ROTATE_ID.encode());

                    let call: <T as Config>::RuntimeCall = Call::rotate_key {}.into();
                    let bounded_call: BoundedCallOf<T> = <T as Config>::Preimages::bound(call).unwrap();

                    if T::Scheduler::schedule_named(
                        id,
                        DispatchTime::After(BlockNumberFor::<T>::zero()),
                        Some((T::RefreshPeriod::get(), u32::max_value())),
                        LOWEST_PRIORITY,
                        frame_system::RawOrigin::Root.into(),
                        bounded_call,
                    )
                    .is_err()
                    {
                        frame_support::print("Error initialising symmetric key rotation schedule");
                        return Weight::zero();
                    }

                    let id: Vec<u8> = id.encode();
                    <KeyScheduleId<T>>::put(Some(BoundedVec::truncate_from(id)));

                    Weight::zero()
                }
                Some(_) => Weight::zero(),
            }
        }
    }

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn key)]
    pub(super) type Key<T: Config> = StorageValue<_, BoundedVec<u8, T::KeyLength>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn key_schedule)]
    pub(super) type KeyScheduleId<T: Config> = StorageValue<_, Option<BoundedVec<u8, ConstU32<32>>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // key was updated.
        UpdateKey(BoundedVec<u8, T::KeyLength>),
    }

    // TODO: Fix this
    #[pallet::error]
    pub enum Error<T> {
        // The supplied key had incorrect length
        IncorrectKeyLength,
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::update_key())]
        pub fn update_key(origin: OriginFor<T>, new_key: BoundedVec<u8, T::KeyLength>) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            ensure!(
                new_key.len() == T::KeyLength::get() as usize,
                Error::<T>::IncorrectKeyLength
            );

            <Key<T>>::put(&new_key);
            Self::deposit_event(Event::UpdateKey(new_key));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::rotate_key())]
        pub fn rotate_key(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            T::RotateOrigin::ensure_origin(origin)?;

            let new_key = generate_key::<T>();
            <Key<T>>::put(&new_key);
            Self::deposit_event(Event::UpdateKey(new_key));

            Ok(().into())
        }
    }

    fn generate_key<T: Config>() -> BoundedVec<u8, T::KeyLength> {
        let key_length = T::KeyLength::get() as usize;
        let mut output = Vec::<u8>::new();

        while output.len() < key_length {
            let random_seed = T::Randomness::random(&KEY_RANDOM_ID[..]);
            let random = random_seed.0.as_ref();
            output.extend_from_slice(random);
        }

        BoundedVec::<_, T::KeyLength>::truncate_from(output)
    }
}

impl<T: Config> Pallet<T> {
    /// Migrate storage format from V0 to V1.
    ///
    /// Returns the weight consumed by this migration.
    pub fn migrate_v0_to_v1() -> Weight {
        let id = <KeyScheduleId<T>>::get();
        let id = match id {
            None => None,
            Some(id) => {
                let id = id.to_vec();
                let id = blake2_256(&id);
                let id: Vec<u8> = id.encode();
                Some(BoundedVec::<u8, ConstU32<32>>::truncate_from(id))
            }
        };
        <KeyScheduleId<T>>::put(id);

        T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1)
    }
}
