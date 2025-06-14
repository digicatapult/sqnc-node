#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::RawOrigin, traits::Get, BoundedVec, Parameter};
pub use pallet::*;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{AtLeast32Bit, One},
    RuntimeDebug,
};
use sp_std::prelude::*;

use sqnc_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator, ValidationResult};

pub mod migration;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// import the restrictions module where all our restriction types are defined
mod restrictions;
pub use restrictions::*;

mod binary_expression_tree;
pub use binary_expression_tree::*;

#[derive(Encode, Debug, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
pub enum ProcessStatus {
    Disabled,
    Enabled,
}

impl Default for ProcessStatus {
    fn default() -> Self {
        ProcessStatus::Disabled
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxProcessProgramLength))]
pub struct Process<
    RoleKey,
    TokenMetadataKey,
    TokenMetadataValue,
    TokenMetadataValueDiscriminator,
    MaxProcessProgramLength,
> where
    RoleKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataValue: Parameter + Default + MaxEncodedLen,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue> + MaxEncodedLen,
    MaxProcessProgramLength: Get<u32>,
{
    status: ProcessStatus,
    program: BoundedVec<
        BooleanExpressionSymbol<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>,
        MaxProcessProgramLength,
    >,
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator, MaxProcessProgramLength> Default
    for Process<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator, MaxProcessProgramLength>
where
    RoleKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataValue: Parameter + Default + MaxEncodedLen,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue> + MaxEncodedLen,
    MaxProcessProgramLength: Get<u32>,
{
    fn default() -> Self {
        Process {
            status: ProcessStatus::Disabled,
            program: vec![BooleanExpressionSymbol::Restriction(Restriction::None)]
                .try_into()
                .unwrap(),
        }
    }
}

impl<R, K, V, D, MR> PartialEq<Process<R, K, V, D, MR>> for Process<R, K, V, D, MR>
where
    R: Parameter + Default + Ord + MaxEncodedLen,
    K: Parameter + Default + Ord + MaxEncodedLen,
    V: Parameter + Default + MaxEncodedLen,
    D: Parameter + Default + From<V> + MaxEncodedLen,
    MR: Get<u32>,
{
    fn eq(&self, other: &Process<R, K, V, D, MR>) -> bool {
        self.status == other.status && self.program == other.program
    }
}

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;
    use parity_scale_codec::MaxEncodedLen;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // The primary identifier for a process (i.e. it's name, and version)
        type ProcessIdentifier: Parameter + Default + MaxEncodedLen + MaybeSerializeDeserialize;
        type ProcessVersion: Parameter + AtLeast32Bit + Default + MaxEncodedLen;

        #[pallet::constant]
        type MaxProcessProgramLength: Get<u32>;

        // Origins for calling these extrinsics. For now these are expected to be root
        type CreateProcessOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type DisableProcessOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        type TokenId: Parameter + Default + MaxEncodedLen + MaybeSerializeDeserialize;
        type RoleKey: Parameter + Default + Ord + MaxEncodedLen + MaybeSerializeDeserialize;
        type TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen + MaybeSerializeDeserialize;
        type TokenMetadataValue: Parameter
            + Default
            + MaxEncodedLen
            + MaybeSerializeDeserialize
            + PartialEq<Self::TokenId>;
        type TokenMetadataValueDiscriminator: Parameter
            + Default
            + From<Self::TokenMetadataValue>
            + MaxEncodedLen
            + MaybeSerializeDeserialize;

        // Origin for overriding weight calculation implementation
        type WeightInfo: WeightInfo;
    }

    /// The in-code storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn process_model)]
    pub(super) type ProcessModel<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        T::ProcessVersion,
        Process<
            T::RoleKey,
            T::TokenMetadataKey,
            T::TokenMetadataValue,
            T::TokenMetadataValueDiscriminator,
            T::MaxProcessProgramLength,
        >,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn version_model)]
    pub(super) type VersionModel<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProcessIdentifier, T::ProcessVersion, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub processes: Vec<(
            T::ProcessIdentifier,
            BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator,
                >,
                T::MaxProcessProgramLength,
            >,
        )>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { processes: Vec::new() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (process_id, program) in self.processes.iter() {
                if !Pallet::<T>::validate_program(&program) {
                    panic!("Invalid program detected in genesis!")
                }
                let version: <T as Config>::ProcessVersion = One::one();
                <VersionModel<T>>::insert(&process_id, version.clone());
                Pallet::<T>::persist_process(process_id, &version, program).unwrap();
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // id, version, program, is_new
        ProcessCreated(
            T::ProcessIdentifier,
            T::ProcessVersion,
            BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator,
                >,
                T::MaxProcessProgramLength,
            >,
            bool,
        ),
        //id, version
        ProcessDisabled(T::ProcessIdentifier, T::ProcessVersion),
    }

    #[pallet::error]
    pub enum Error<T> {
        // process already exists, investigate
        AlreadyExists,
        // attempting to disable non-existing process
        NonExistingProcess,
        // process is already disabled
        AlreadyDisabled,
        // process not found for this version
        InvalidVersion,
        // restriction program is invalid
        InvalidProgram,
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_process(program.len() as u32))]
        pub fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            version: T::ProcessVersion,
            program: BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator,
                >,
                T::MaxProcessProgramLength,
            >,
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;

            ensure!(Pallet::<T>::validate_program(&program), Error::<T>::InvalidProgram);
            ensure!(version > Zero::zero(), Error::<T>::InvalidVersion);

            let previous_version = Pallet::<T>::get_previous_version(&id).unwrap_or(Zero::zero());
            ensure!(version > previous_version, Error::<T>::AlreadyExists);

            <VersionModel<T>>::insert(&id, version.clone());
            Pallet::<T>::persist_process(&id, &version, &program)?;

            Self::deposit_event(Event::ProcessCreated(
                id,
                version.clone(),
                program,
                previous_version.is_zero(),
            ));

            return Ok(().into());
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::disable_process())]
        pub fn disable_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            version: T::ProcessVersion,
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
        pub fn validate_program(
            program: &BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator,
                >,
                T::MaxProcessProgramLength,
            >,
        ) -> bool {
            let executed_stack_height = program.iter().try_fold(0u8, |stack_height, symbol| match symbol {
                BooleanExpressionSymbol::Op(_) => {
                    let stack_height = stack_height.checked_sub(2);
                    return stack_height.and_then(|stack_height| stack_height.checked_add(1));
                }
                BooleanExpressionSymbol::Restriction(_) => stack_height.checked_add(1),
            });
            executed_stack_height == Some(1u8)
        }

        pub fn get_previous_version(id: &T::ProcessIdentifier) -> Option<T::ProcessVersion> {
            <VersionModel<T>>::try_get(&id).ok()
        }

        pub fn persist_process(
            id: &T::ProcessIdentifier,
            v: &T::ProcessVersion,
            p: &BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator,
                >,
                T::MaxProcessProgramLength,
            >,
        ) -> Result<(), Error<T>> {
            return match <ProcessModel<T>>::contains_key(&id, &v) {
                true => Err(Error::<T>::AlreadyExists),
                false => {
                    <ProcessModel<T>>::insert(
                        id,
                        v,
                        Process {
                            program: p.clone(),
                            status: ProcessStatus::Enabled,
                        },
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
            version: &T::ProcessVersion,
        ) -> Result<(), Error<T>> {
            ensure!(
                <ProcessModel<T>>::contains_key(&id, version.clone()),
                Error::<T>::NonExistingProcess,
            );
            ensure!(<VersionModel<T>>::contains_key(&id), Error::<T>::InvalidVersion);
            return match *version > <VersionModel<T>>::get(&id) {
                true => Err(Error::<T>::InvalidVersion),
                false => Ok(()),
            };
        }
    }
}

impl<T: Config> ProcessValidator<T::TokenId, T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>
    for Pallet<T>
{
    type ProcessIdentifier = T::ProcessIdentifier;
    type ProcessVersion = T::ProcessVersion;
    type WeightArg = u32;
    type Weights = T::WeightInfo;

    fn validate_process<'a>(
        id: &'a ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &'a RawOrigin<T::AccountId>,
        references: &'a Vec<
            ProcessIO<T::TokenId, T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>,
        >,
        inputs: &'a Vec<ProcessIO<T::TokenId, T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        outputs: &'a Vec<ProcessIO<T::TokenId, T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
    ) -> ValidationResult<u32> {
        let maybe_process = <ProcessModel<T>>::try_get(id.id.clone(), id.version.clone());

        let get_args = |arg_type: ArgType| match arg_type {
            ArgType::Input => inputs,
            ArgType::Output => outputs,
            ArgType::Reference => references,
        };

        match maybe_process {
            Ok(process) => {
                if process.status == ProcessStatus::Disabled {
                    return ValidationResult {
                        success: false,
                        executed_len: 0,
                    };
                }

                let mut stack: Vec<bool> = Vec::with_capacity(T::MaxProcessProgramLength::get() as usize);
                let mut executed_len: u32 = 0;
                for symbol in process.program {
                    executed_len = executed_len + 1;
                    match symbol {
                        BooleanExpressionSymbol::Op(op) => {
                            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                                stack.push(op.eval(a, b));
                            } else {
                                return ValidationResult {
                                    success: false,
                                    executed_len: executed_len,
                                };
                            }
                        }
                        BooleanExpressionSymbol::Restriction(r) => {
                            stack.push(validate_restriction(r, sender, get_args));
                        }
                    }
                }

                ValidationResult {
                    success: stack.pop().unwrap_or(false),
                    executed_len: executed_len,
                }
            }
            Err(_) => ValidationResult {
                success: false,
                executed_len: 0,
            },
        }
    }
}
