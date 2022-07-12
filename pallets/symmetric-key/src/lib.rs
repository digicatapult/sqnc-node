#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
    traits::{
        schedule::{DispatchTime, Named as ScheduleNamed, LOWEST_PRIORITY},
        Get, Randomness
    },
    BoundedVec
};
use sp_runtime::traits::Dispatchable;

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

const KEY_ROTATE_ID: [u8; 12] = *b"SymmetricKey";
const KEY_RANDOM_ID: [u8; 13] = *b"SYMMETRIC_KEY";

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// what does this do!!!!
        type ScheduleCall: Parameter + Dispatchable<Origin = Self::Origin> + From<Call<Self>>;
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type KeyLength: Get<u32>;

        /// The origin which can update the key
        type UpdateOrigin: EnsureOrigin<Self::Origin>;
        /// The origin which can rotate the key
        type RotateOrigin: EnsureOrigin<Self::Origin>;
        /// Source of randomness when generating new keys.
        /// In production this should come from a secure source such as the Babe pallet
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

        #[pallet::constant]
        type RefreshPeriod: Get<Self::BlockNumber>;
        /// Overarching type of all pallets origins.
        type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;
        /// The Scheduler.
        type Scheduler: ScheduleNamed<Self::BlockNumber, Self::ScheduleCall, Self::PalletsOrigin>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: T::BlockNumber) -> frame_support::weights::Weight {
            let existing_schedule = <KeyScheduleId<T>>::get();

            match existing_schedule {
                None => {
                    let id: Vec<u8> = KEY_ROTATE_ID.encode();
                    if T::Scheduler::schedule_named(
                        id.clone(),
                        DispatchTime::After(T::BlockNumber::from(1u32)),
                        Some((T::RefreshPeriod::get(), u32::max_value())),
                        LOWEST_PRIORITY,
                        frame_system::RawOrigin::Root.into(),
                        Call::rotate_key {}.into()
                    )
                    .is_err()
                    {
                        frame_support::print("Error initialising symmetric key rotation schedule");
                        return 0;
                    }

                    <KeyScheduleId<T>>::put(Some(BoundedVec::<_, _>::truncate_from(id)));

                    0
                }
                Some(_) => 0
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
        UpdateKey(BoundedVec<u8, T::KeyLength>)
    }

    // TODO: Fix this
    #[pallet::error]
    pub enum Error<T> {
        // The supplied key had incorrect length
        IncorrectKeyLength
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
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
