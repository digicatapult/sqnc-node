#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet::*;
use sp_std::prelude::*;
use frame_support::dispatch::EncodeLike;

use vitalam_pallet_traits::{ProcessIO, ProcessValidator};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// import the restrictions module where all our restriction types are defined
mod restrictions;
use restrictions::Restriction;

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ProcessStatus {
    Disabled,
    Enabled,
}

impl Default for ProcessStatus {
    fn default() -> Self {
        ProcessStatus::Disabled
    }
}

/*
- process
- version

1. look up a version storage map
2. increment
3. update verion in storage
4. create a process in storage

also for the process
*/

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Version {
    version: i32,
}
impl EncodeLike<i32> for Version {}

impl Default for Version {
    fn default() -> Self {
        Version { version: 1 }
    }
}


#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Process {
    status: ProcessStatus,
    restrictions: Vec<Restriction>,
    version: i32,
}

impl Default for Process {
    fn default() -> Self {
        Process {
            status: ProcessStatus::Disabled,
            restrictions: vec![{ Restriction::None }],
            version: 0,
        }
    }
}

pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AtLeast32Bit;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // The primary identifier for a process (i.e. it's name)
        type ProcessIdentifier: Parameter;
        type ProcessVersion: Parameter + AtLeast32Bit;

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
        i32,
        Process,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn latest_process_version)]
    pub(super) type LatestProcessVersion<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        i32,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO: implement correct events for extrinsics including params
        ProcessCreated,
        ProcessDisabled,
    }

    #[pallet::error]
    pub enum Error<T> {
        // TODO: implement errors for extrinsics
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO: implement create_process with correct parameters and impl
        #[pallet::weight(T::WeightInfo::create_process())]
        pub(super) fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            restrictions: Vec<Restriction>,
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;
            let version = <LatestProcessVersion<T>>::get(id.clone());
            let process = <ProcessesByIdAndVersion<T>>::get(id.clone(), version);
            let new_version= if process.version == 0 { version } else { version + 1 };

            <ProcessesByIdAndVersion<T>>::insert(
                id.clone(),
                version, 
                Process {
                    version: version,
                    status: ProcessStatus::Disabled,
                    restrictions: restrictions,
                }
            );
            <LatestProcessVersion<T>>::insert(
                id.clone(),
                Version {
                    version: new_version,
                }
            );

            Self::deposit_event(Event::ProcessCreated);
            Ok(().into())
        }

        // TODO: implement disable_process with correct parameters and impl
        #[pallet::weight(T::WeightInfo::disable_process())]
        pub(super) fn disable_process(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            T::DisableProcessOrigin::ensure_origin(origin)?;
            Self::deposit_event(Event::ProcessDisabled);
            Ok(().into())
        }
    }
}

impl<T: Config> ProcessValidator<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue> for Pallet<T> {
    type ProcessIdentifier = T::ProcessIdentifier;
    type ProcessVersion = T::ProcessVersion;

    // TODO: implement lookup of process and checking of restrictions
    fn validate_process(
        _id: T::ProcessIdentifier,
        _version: T::ProcessVersion,
        _sender: T::AccountId,
        _inputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        _outputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
    ) -> bool {
        true
    }
}
