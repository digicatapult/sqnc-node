#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet::*;
use sp_std::prelude::*;

use vitalam_pallet_traits::{ProcessIO, ProcessValidator};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// import the restrictions module where all our restriction types are defined
mod restrictions;
use restrictions::{Restriction};

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ProcessStatus {
    Disabled,
    Enabled
}

impl Default for ProcessStatus {
    fn default() -> Self {
        ProcessStatus::Disabled
    }
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Process {
    status: ProcessStatus,
    restrictions: Vec<Restriction>
}

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
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // The primary identifier for a process (i.e. it's name)
        type ProcessIdentifier: Parameter;

        // Origins for calling these extrinsics. For now these are expected to be root
        type CreateProcessOrigin: EnsureOrigin<Self::Origin>;
        type DisableProcessOrigin: EnsureOrigin<Self::Origin>;

        type RoleKey: Parameter + Default + Ord;
        type TokenMetadataKey: Parameter + Default + Ord;
        type TokenMetadataValue: Parameter + Default;

        // Origin for overriding weight calculation implementation
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn processes_by_id_and_version)]
    pub(super) type ProcessesByIdAndVersion<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        u32,
        Process,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO implement correct events for extrinsics
        ProcessCreated,
        ProcessDisabled
    }

    #[pallet::error]
    pub enum Error<T> {
        // TODO implement errors for extrinsics
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO implement create_process with correct parameters and impl
        #[pallet::weight(T::WeightInfo::create_process())]
        pub(super) fn create_process(
            origin: OriginFor<T>
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            T::CreateProcessOrigin::ensure_origin(origin)?;

            Self::deposit_event(Event::ProcessCreated);
            Ok(().into())
        }

        // TODO implement disable_process with correct parameters and impl
        #[pallet::weight(T::WeightInfo::disable_process())]
        pub(super) fn disable_process(
            origin: OriginFor<T>
        ) -> DispatchResultWithPostInfo {
            T::DisableProcessOrigin::ensure_origin(origin)?;
            Self::deposit_event(Event::ProcessDisabled);
            Ok(().into())
        }
    }
}

impl<T: Config> ProcessValidator<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue> for Pallet<T> {
    type ProcessIdentifier = T::ProcessIdentifier;

    // TODO: add arguments for validate process and implement so it can be used by pallet-simple-nft
    fn validate_process(
        _id: T::ProcessIdentifier,
        _version: u32,
        _sender: T::AccountId,
        _inputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        _outputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>) -> bool {
        true
    }
}
