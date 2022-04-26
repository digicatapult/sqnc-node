#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::Parameter;
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};
use sp_std::prelude::*;

use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// import the restrictions module where all our restriction types are defined
mod restrictions;
use restrictions::*;

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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Process<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>
{
    status: ProcessStatus,
    restrictions: Vec<Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>>
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> Default
    for Process<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>
{
    fn default() -> Self {
        Process {
            status: ProcessStatus::Disabled,
            restrictions: vec![]
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

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // The primary identifier for a process (i.e. it's name, and version)
        type ProcessIdentifier: Parameter;
        type ProcessVersion: Parameter + AtLeast32Bit + Default;

        #[pallet::constant]
        type MaxRestrictionDepth: Get<u8>;

        // Origins for calling these extrinsics. For now these are expected to be root
        type CreateProcessOrigin: EnsureOrigin<Self::Origin>;
        type DisableProcessOrigin: EnsureOrigin<Self::Origin>;

        type RoleKey: Parameter + Default + Ord;
        type TokenMetadataKey: Parameter + Default + Ord;
        type TokenMetadataValue: Parameter + Default;
        type TokenMetadataValueDiscriminator: Parameter + Default + From<Self::TokenMetadataValue>;

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
    #[pallet::getter(fn process_model)]
    pub(super) type ProcessModel<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        T::ProcessVersion,
        Process<T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue, T::TokenMetadataValueDiscriminator>,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn version_model)]
    pub(super) type VersionModel<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProcessIdentifier, T::ProcessVersion, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(
        ProcessIdentifier = "ProcessIdentifier",
        ProcessVersion = "ProcessVersion",
        Vec<Restriction> = "Restrictions",
        bool = "IsNew"
    )]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // id, version, restrictions, is_new
        ProcessCreated(
            T::ProcessIdentifier,
            T::ProcessVersion,
            Vec<
                Restriction<T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue, T::TokenMetadataValueDiscriminator>
            >,
            bool
        ),
        //id, version
        ProcessDisabled(T::ProcessIdentifier, T::ProcessVersion)
    }

    #[pallet::error]
    pub enum Error<T> {
        // process already exists, investigate
        AlreadyExists,
        // attempting to disable non-existing process
        NonExistingProcess,
        // process is already disabled
        AlreadyDisabled,
        // process not found for this versiion
        InvalidVersion,
        // restrictions go over maximum depth
        RestrictionsTooDeep
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_process())]
        pub(super) fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            restrictions: Vec<
                Restriction<T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue, T::TokenMetadataValueDiscriminator>
            >
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;

            for restriction in restrictions.iter() {
                ensure!(
                    !Pallet::<T>::restriction_over_max_depth(restriction.clone(), 0, T::MaxRestrictionDepth::get()),
                    Error::<T>::RestrictionsTooDeep
                );
            }

            let version: T::ProcessVersion = Pallet::<T>::update_version(id.clone()).unwrap();
            Pallet::<T>::persist_process(&id, &version, &restrictions)?;

            Self::deposit_event(Event::ProcessCreated(
                id,
                version.clone(),
                restrictions,
                version == One::one()
            ));

            return Ok(().into());
        }

        #[pallet::weight(T::WeightInfo::disable_process())]
        pub(super) fn disable_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            version: T::ProcessVersion
        ) -> DispatchResultWithPostInfo {
            T::DisableProcessOrigin::ensure_origin(origin)?;
            Pallet::<T>::validate_version_and_process(&id, &version)?;
            Pallet::<T>::set_disabled(&id, &version)?;

            Self::deposit_event(Event::ProcessDisabled(id, version));
            return Ok(().into());
        }
    }

    // helper methods
    impl<T: Config> Pallet<T> {
        pub fn restriction_over_max_depth(
            restriction: Restriction<
                T::RoleKey,
                T::TokenMetadataKey,
                T::TokenMetadataValue,
                T::TokenMetadataValueDiscriminator
            >,
            count: u8,
            max_depth: u8
        ) -> bool {
            if count > max_depth {
                return true;
            }

            match restriction {
                Restriction::Combined {
                    operator: _,
                    restriction_a,
                    restriction_b
                } => {
                    let incremented_count = count + 1;
                    Pallet::<T>::restriction_over_max_depth(*restriction_a, incremented_count, max_depth)
                        || Pallet::<T>::restriction_over_max_depth(*restriction_b, incremented_count, max_depth)
                }
                _ => false
            }
        }

        pub fn get_version(id: &T::ProcessIdentifier) -> T::ProcessVersion {
            return match <VersionModel<T>>::contains_key(&id) {
                true => <VersionModel<T>>::get(&id) + One::one(),
                false => One::one()
            };
        }

        pub fn update_version(id: T::ProcessIdentifier) -> Result<T::ProcessVersion, Error<T>> {
            let version: T::ProcessVersion = Pallet::<T>::get_version(&id);
            match version == One::one() {
                true => <VersionModel<T>>::insert(&id, version.clone()),
                false => <VersionModel<T>>::mutate(&id, |v| *v = version.clone())
            };

            return Ok(version);
        }

        pub fn persist_process(
            id: &T::ProcessIdentifier,
            v: &T::ProcessVersion,
            r: &Vec<
                Restriction<T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue, T::TokenMetadataValueDiscriminator>
            >
        ) -> Result<(), Error<T>> {
            return match <ProcessModel<T>>::contains_key(&id, &v) {
                true => Err(Error::<T>::AlreadyExists),
                false => {
                    <ProcessModel<T>>::insert(
                        id,
                        v,
                        Process {
                            restrictions: r.clone(),
                            status: ProcessStatus::Enabled
                        }
                    );
                    return Ok(());
                }
            };
        }

        pub fn set_disabled(id: &T::ProcessIdentifier, version: &T::ProcessVersion) -> Result<(), Error<T>> {
            let process = <ProcessModel<T>>::get(&id, &version);
            return match process.status == ProcessStatus::Disabled {
                true => Err(Error::<T>::AlreadyDisabled),
                false => {
                    <ProcessModel<T>>::mutate(id.clone(), version, |process| {
                        (*process).status = ProcessStatus::Disabled;
                    });
                    return Ok(());
                }
            };
        }

        pub fn validate_version_and_process(
            id: &T::ProcessIdentifier,
            version: &T::ProcessVersion
        ) -> Result<(), Error<T>> {
            ensure!(
                <ProcessModel<T>>::contains_key(&id, version.clone()),
                Error::<T>::NonExistingProcess,
            );
            ensure!(<VersionModel<T>>::contains_key(&id), Error::<T>::InvalidVersion);
            return match *version > <VersionModel<T>>::get(&id) {
                true => Err(Error::<T>::InvalidVersion),
                false => Ok(())
            };
        }
    }
}

impl<T: Config> ProcessValidator<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue> for Pallet<T> {
    type ProcessIdentifier = T::ProcessIdentifier;
    type ProcessVersion = T::ProcessVersion;

    fn validate_process(
        id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &T::AccountId,
        inputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        outputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>
    ) -> bool {
        let maybe_process = <ProcessModel<T>>::try_get(id.id, id.version);

        match maybe_process {
            Ok(process) => {
                if process.status == ProcessStatus::Disabled {
                    return false;
                }

                for restriction in process.restrictions {
                    let is_valid = validate_restriction::<
                        T::AccountId,
                        T::RoleKey,
                        T::TokenMetadataKey,
                        T::TokenMetadataValue,
                        T::TokenMetadataValueDiscriminator
                    >(restriction, &sender, &inputs, &outputs);

                    if !is_valid {
                        return false;
                    }
                }
                true
            }
            Err(_) => false
        }
    }
}
