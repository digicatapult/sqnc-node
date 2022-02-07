#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::Parameter;
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};
use sp_std::prelude::*;

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

#[derive(Encode, Default, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Process {
    status: ProcessStatus,
    restrictions: Vec<Restriction>,
}

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    type Restrictions = Vec<Restriction>;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // The primary identifier for a process (i.e. it's name, and version)
        type ProcessIdentifier: Parameter;
        type ProcessVersion: Parameter + AtLeast32Bit + Default;

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
    #[pallet::getter(fn process_model)] // not sure about name, store?, map?
    pub(super) type ProcessModel<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        T::ProcessVersion,
        Process,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn version_model)]
    pub(super) type VersionModel<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProcessIdentifier, T::ProcessVersion, ValueQuery>;

    #[pallet::event]
    // looking by the type, same type for multiple things - bnad idea
    #[pallet::metadata(ProcessIdentifier = "Process Id", ProcessVersion = "Process Version", bool = "Is New")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // id, version, restrictions, is_new
        ProcessCreated(T::ProcessIdentifier, T::ProcessVersion, Vec<Restriction>, bool),
        //id, version
        ProcessDisabled(T::ProcessIdentifier, T::ProcessVersion),
    }

    #[pallet::error]
    pub enum Error<T> {
        // attempting to disable non-existing process
        NonExistingProcess,
        // process is already disabled
        AlreadyDisabled,
        // process not found for this versiion
        InvalidVersion,
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_process())]
        pub(super) fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            restrictions: Vec<Restriction>,
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;
            let new_version: T::ProcessVersion = Pallet::<T>::update_version(id.clone()).unwrap();
            Pallet::<T>::persist_process(&id, &new_version, restrictions.clone());

            Self::deposit_event(Event::ProcessCreated(
                id,
                new_version.clone(),
                restrictions,
                new_version == One::one(),
            ));

            return Ok(().into());
        }

        #[pallet::weight(T::WeightInfo::disable_process())]
        pub(super) fn disable_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            version: T::ProcessVersion,
        ) -> DispatchResultWithPostInfo {
            T::DisableProcessOrigin::ensure_origin(origin)?;


            Pallet::<T>::validate_version_and_id(&id, &version)?;
            Pallet::<T>::disable_process(&id, &version)?;

            Self::deposit_event(Event::ProcessDisabled(id, version));
            return Ok(().into());
        }
    }

    // helper methods
    impl<T: Config> Pallet<T> {
        pub fn get_version(id: &T::ProcessIdentifier) -> T::ProcessVersion {
            let version: T::ProcessVersion = <VersionModel<T>>::get(&id);
            return version;
        }

        // rebase with master
        pub fn update_version(id: T::ProcessIdentifier) -> Result<T::ProcessVersion, ()> {
            let version: T::ProcessVersion = Pallet::<T>::get_version(&id);
            let exists: bool = <ProcessModel<T>>::contains_key(&id, version.clone());
            let new_version: T::ProcessVersion = match exists {
                true => version,
                false => version + One::one(),
            };
            match &new_version == &One::one() {
                true => <VersionModel<T>>::insert(&id, new_version.clone()),
                false => <VersionModel<T>>::mutate(&id, |v| *v = new_version.clone()),
            };

            return Ok(new_version);
        }

        // remove underscores from helper methods
        pub fn persist_process(id: &T::ProcessIdentifier, version: &T::ProcessVersion, restrictions: Restrictions) {
            <ProcessModel<T>>::insert(
                id,
                version,
                Process {
                    restrictions: restrictions.clone(),
                    ..Default::default()
                },
            );
        }

        pub fn disable_process(id: &T::ProcessIdentifier, version: &T::ProcessVersion) -> Result<bool, Error<T>> {
            let process: Process = <ProcessModel<T>>::get(&id, &version);
            if process.status == ProcessStatus::Disabled {
                return Err(Error::<T>::AlreadyDisabled);
            };

            <ProcessModel<T>>::mutate(id.clone(), version, |process| {
                (*process).status = ProcessStatus::Disabled;
            });

            return Ok(true);
        }
        
        pub fn validate_version_and_id(id: &T::ProcessIdentifier, version: &T::ProcessVersion) -> Result<bool, Error<T>> {
            ensure!(
                <ProcessModel<T>>::contains_key(&id, version.clone()),
                Error::<T>::NonExistingProcess,
            );
            ensure!(<VersionModel<T>>::contains_key(&id), Error::<T>::InvalidVersion);
            let version_found: T::ProcessVersion = <VersionModel<T>>::get(&id);

            if *version != version_found {
                // TODO should return version_found if any
                return Err(Error::<T>::InvalidVersion);
            }

            return Ok(true);
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
