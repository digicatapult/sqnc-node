#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::traits::{
    LockIdentifier, Randomness, schedule::{Named as ScheduleNamed, DispatchTime, LOWEST_PRIORITY}
};
use sp_runtime::traits::Dispatchable;

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

const IPFS_KEY_ID: LockIdentifier = *b"ipfs_key";

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// mod migration;

// #[derive(Encode, Decode, Default, Clone, PartialEq)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct Token<AccountId, TokenId, BlockNumber, TokenMetadata> {
//     id: TokenId,
//     owner: AccountId,
//     creator: AccountId,
//     created_at: BlockNumber,
//     destroyed_at: Option<BlockNumber>,
//     metadata: TokenMetadata,
//     parents: Vec<TokenId>,
//     children: Option<Vec<TokenId>>, // children is the only mutable component of the token
// }

// pub mod weights;

// pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// what does this do!!!!
        type ScheduleCall: Parameter + Dispatchable<Origin=Self::Origin> + From<Call<Self>>;
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type KeyLength: Get<u32>;

        /// The origin which can update the IPFS key
	    type UpdateOrigin: EnsureOrigin<Self::Origin>;
        /// The origin which can rotate the IPFS key
	    type RotateOrigin: EnsureOrigin<Self::Origin>;

        type Randomness: Randomness<Self::Hash>;

        #[pallet::constant]
        type RefreshPeriod: Get<Self::BlockNumber>;
        /// Overarching type of all pallets origins.
	    type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;
        /// The Scheduler.
	    type Scheduler: ScheduleNamed<Self::BlockNumber, Self::ScheduleCall, Self::PalletsOrigin>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: T::BlockNumber) -> frame_support::weights::Weight {
            let existing_schedule = <IpfsKeySchedule<T>>::get();

            match existing_schedule {
                None => {
                    let schedule_result = T::Scheduler::schedule_named(
                        IPFS_KEY_ID.encode(),
                        DispatchTime::After(T::BlockNumber::from(1u32)),
                        Some((T::RefreshPeriod::get(), u32::max_value())),
                        LOWEST_PRIORITY,
                        frame_system::RawOrigin::Root.into(),
                        Call::rotate_ipfs_key().into(),
                    );

                    if schedule_result.is_err() {
                        frame_support::print("Ahhhhh");
                        return 0;
                    }

                    <IpfsKeySchedule<T>>::put(Some(schedule_result.unwrap()));

                    0
                }
                Some(_) => 0
            }
        }
    }

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn ipfs_key)]
    pub(super) type IpfsKey<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn ipfs_key_schedule)]
    pub(super) type IpfsKeySchedule<T: Config> = StorageValue<_, Option<<<T as pallet::Config>::Scheduler as ScheduleNamed<T::BlockNumber, T::ScheduleCall, T::PalletsOrigin>>::Address>, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(Vec<u8> = "IpfsKey")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // IPFS key was updated.
        UpdateIpfsKey(Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        // The supplied IPFS key had incorrect length
        IncorrectKeyLength,
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO benchmark weights
        #[pallet::weight(10_000)]
        pub(super) fn update_ipfs_key(
            origin: OriginFor<T>,
            new_ipfs_key: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            T::UpdateOrigin::ensure_origin(origin)?;
            ensure!(new_ipfs_key.len() == T::KeyLength::get() as usize, Error::<T>::IncorrectKeyLength);

            <IpfsKey<T>>::put(&new_ipfs_key);

            Ok(().into())
        }

        // TODO benchmark weights
        #[pallet::weight(10_000)]
        pub(super) fn rotate_ipfs_key(
            origin: OriginFor<T>
        ) -> DispatchResultWithPostInfo {
            T::RotateOrigin::ensure_origin(origin)?;

            let new_ipfs_key = generate_key::<T>();
            <IpfsKey<T>>::put(&new_ipfs_key);
            Self::deposit_event(Event::UpdateIpfsKey(new_ipfs_key));

            Ok(().into())
        }
    }

    fn generate_key<T: Config>() -> Vec<u8> {
        let key_length = T::KeyLength::get() as usize;
        let mut output = Vec::<u8>::new();

        while output.len() < key_length {
            let random_seed = T::Randomness::random(&b"IPFS_SWARM_KEY"[..]);
            let random = random_seed.as_ref();
            output.extend_from_slice(random);
        }

        (&output[0..key_length]).to_vec()
    }
}

