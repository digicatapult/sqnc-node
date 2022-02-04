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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Version {
    version: i32, // TODO: sort this type, should be included from trait
}
// TODO remove once type has been soprted <version>
impl EncodeLike<i32> for Version {}


#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Process {
    status: ProcessStatus,
    restrictions: Vec<Restriction>,
}

impl Default for Process {
    fn default() -> Process {
        Process {
            status: ProcessStatus::Disabled,
            restrictions: vec![{ Restriction::None }],
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

    type Restrictions = Vec<Restriction>;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // The primary identifier for a process (i.e. it's name, and version)
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
    #[pallet::getter(fn get_process)]
    pub(super) type GetProcess<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        i32,
        Process,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn get_version)]
    pub(super) type GetVersion<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        i32,
        ValueQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // id, version, restrictions, is_new
        ProcessCreated(T::ProcessIdentifier, i32, Vec<Restriction>, bool),
        ProcessDisabled,
    }

    #[pallet::error]
    pub enum Error<T> {
        // TODO: implement errors for extrinsics
    }

    // helper functions
    fn get_version<T: Config>(id: T::ProcessIdentifier) -> i32 {
        return <GetVersion<T>>::get(&id);
    }

    fn uopdate_version<T: Config>(id: T::ProcessIdentifier) -> i32 {
        let version: i32 = get_version::<T>(id.clone());
        let exists: Result<Process, ()> = <GetProcess<T>>::try_get(id.clone(),version);
        let new_version: i32 = version + if exists.is_ok() { 0 } else { 1 };
        <GetVersion<T>>::insert(
            &id,
            Version {
                version: new_version,
            }
        );

        return new_version;
    } 
    fn persist_process<T: Config>(id: &T::ProcessIdentifier, version: &i32, restrictions: Restrictions) {
        <GetProcess<T>>::insert(
            id,
            version,
            Process {
                restrictions: restrictions.clone(),
                ..Default::default()
            },
        );
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(T::WeightInfo::create_process())]
        pub(super) fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            restrictions: Vec<Restriction>
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;
            let new_version: i32 = uopdate_version::<T>(id.clone());
            persist_process::<T>(&id, &new_version, restrictions.clone());


            Self::deposit_event(Event::ProcessCreated(
                id,
                new_version,
                restrictions,
                new_version == 1
            ));

            return Ok(().into());
        }

        // TODO: implement disable_process with correct parameters and impl
        // For Danniel! - Good Morning:)
        /*
            - use an existing method -> GetProcess to query storage
            - call the method right after the origing validation
            - a handler of psuedo code for already disabled process
                - if disabled
                    - return ok()
                - if not
                    - updated process with the Disabled status
            - create an event return args Line 124
            - unit tests
                - if ensure_origing fails
                - if process is already disabled
                - if process does not exist
                - happy path
         */
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
