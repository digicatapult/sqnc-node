#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::RawOrigin,
    traits::{Get, TryCollect},
    BoundedVec,
};
pub use pallet::*;
use parity_scale_codec::{Codec, MaxEncodedLen};
use sp_runtime::traits::{AtLeast32Bit, Hash, One};
use sqnc_pallet_traits::{self as traits};
use sqnc_pallet_traits::{ProcessFullyQualifiedId, ProcessValidator, ValidateProcessWeights};

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

mod token;

mod input;
use input::Input;

mod output;

mod graveyard;
pub use graveyard::GraveyardState;

pub mod migration;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{pallet_prelude::*, Parameter};
    use frame_system::pallet_prelude::{BlockNumberFor, *};

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type TokenId: Parameter + AtLeast32Bit + Default + Copy + Codec + MaxEncodedLen;
        type RoleKey: Parameter + Default + Ord + MaxEncodedLen;

        type TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen;
        type TokenMetadataValue: Parameter + Default + MaxEncodedLen;

        type WeightInfo: WeightInfo;

        type ProcessValidator: ProcessValidator<
            Self::TokenId,
            Self::AccountId,
            Self::RoleKey,
            Self::TokenMetadataKey,
            Self::TokenMetadataValue,
        >;

        // Maximum number of metadata items allowed per token
        #[pallet::constant]
        type MaxMetadataCount: Get<u32>;

        // Maximum number of token roles
        #[pallet::constant]
        type MaxRoleCount: Get<u32>;

        // Maximum number of process inputs
        #[pallet::constant]
        type MaxInputCount: Get<u32>;

        // Maximum number of process outputs
        #[pallet::constant]
        type MaxOutputCount: Get<u32>;

        // Maximum number of process outputs
        #[pallet::constant]
        type TokenTombstoneDuration: Get<BlockNumberFor<Self>>;
    }

    // Define some derived types off of the Config trait to clean up declarations later

    // ProcessIdentifier can be pulled off of the configured ProcessValidator
    pub(crate) type ProcessIdentifier<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
    >>::ProcessIdentifier;

    // ProcessVersion can be pulled off of the configured ProcessValidator
    pub(crate) type ProcessVersion<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
    >>::ProcessVersion;

    // Construct ProcessId
    pub(crate) type ProcessId<T> = ProcessFullyQualifiedId<ProcessIdentifier<T>, ProcessVersion<T>>;

    // The specific Token is derived from Config and the generic Token struct in this crate
    pub(crate) type Token<T> = token::Token<
        <T as Config>::MaxRoleCount,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenId,
        BlockNumberFor<T>,
        <T as Config>::MaxMetadataCount,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
        <T as Config>::MaxInputCount,
        <T as Config>::MaxOutputCount,
    >;

    // The specific ProcessIO type can be derived from Config
    pub(crate) type Output<T> = output::Output<
        <T as Config>::MaxRoleCount,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::MaxMetadataCount,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
    >;

    // The specific ProcessIO type can be derived from Config
    pub(crate) type ProcessIO<T> = traits::ProcessIO<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
    >;

    pub(crate) type ProcessValidatorWeights<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
    >>::Weights;

    /// The in-code storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Storage value definition
    #[pallet::storage]
    #[pallet::getter(fn last_token)]
    pub(super) type LastToken<T: Config> = StorageValue<_, T::TokenId, ValueQuery>;

    // Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn tokens_by_id)]
    pub(super) type TokensById<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::TokenId,
        Token<T>,
        // We need to use OptionQuery as AccountId is held in the Config trait but doesn't guarantee Copy trait
        OptionQuery,
    >;

    // Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn graveyard)]
    pub(super) type Graveyard<T: Config> = StorageMap<_, Blake2_128Concat, u64, T::TokenId, OptionQuery>;

    // Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn current_graveyard_state)]
    pub(super) type CurrentGraveyardState<T: Config> = StorageValue<_, GraveyardState, ValueQuery>;

    #[pallet::event]
    pub enum Event<T: Config> {
        /// A process was successfully run
        ProcessRan {
            sender: RawOrigin<T::AccountId>,
            process: ProcessId<T>,
            inputs: BoundedVec<T::TokenId, T::MaxInputCount>,
            outputs: BoundedVec<T::TokenId, T::MaxOutputCount>,
        },
        TokenDeleted {
            token_id: T::TokenId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Mutation was attempted on token that has already been burnt
        AlreadyBurnt,
        /// Process failed validation checks
        ProcessInvalid,
        /// An invalid input token id was provided
        InvalidInput,
        /// A token cannot be deleted if it hasn't been burnt
        NotBurnt,
        /// A token was burnt too recently to be deleted perminantly
        BurntTooRecently,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_idle(
            _block_number: BlockNumberFor<T>,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            // 1 read and 1 write to get/set the graveyard state
            let base_weight = T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1);
            let available_iter_weight = remaining_weight.checked_sub(&base_weight);

            // for each delete operation we fetch the graveyard entry, delete the token, then delete the graveyard entry
            let weight_per_iter = T::WeightInfo::delete_token()
                .saturating_add(T::DbWeight::get().reads(1))
                .saturating_add(T::DbWeight::get().writes(1));

            // count how many deletes we can afford
            let iter_count = match available_iter_weight {
                Some(weight) => weight.checked_div_per_component(&weight_per_iter).unwrap_or(0),
                None => 0,
            };

            if iter_count == 0 {
                return remaining_weight;
            }

            // read graveyard state (base_weight)
            let graveyard_state = Self::current_graveyard_state();
            let GraveyardState { start_index, end_index } = graveyard_state;
            let iter_count = sp_std::cmp::min(iter_count, end_index - start_index);

            let (delete_count, delete_op_count) = (0..iter_count)
                .find_map(|i| {
                    let index = start_index + i;
                    // read graveyard for each delete
                    let token_id = Self::graveyard(index).unwrap();

                    // do the delete
                    let delete_result = Self::delete_token_internal(token_id);

                    match delete_result {
                        Ok(_) => {
                            // write to the graveyard
                            <Graveyard<T>>::remove(index);
                            None
                        }
                        Err(Error::<T>::BurntTooRecently) => Some((i, i + 1)),
                        Err(_) => panic!("Unexpected error"),
                    }
                })
                .unwrap_or((iter_count, iter_count));

            // write graveyard state (base_weight)
            <CurrentGraveyardState<T>>::put(GraveyardState {
                start_index: start_index + delete_count,
                end_index,
            });

            let spent_weight = base_weight.saturating_add(weight_per_iter.mul(delete_op_count));
            spent_weight
        }
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(
            T::WeightInfo::run_process(0u32, inputs.len() as u32, outputs.len() as u32) +
            ProcessValidatorWeights::<T>::validate_process_max() -
            ProcessValidatorWeights::<T>::validate_process_min()
        )]
        pub fn run_process(
            origin: OriginFor<T>,
            process: ProcessId<T>,
            inputs: BoundedVec<Input<T::TokenId>, T::MaxInputCount>,
            outputs: BoundedVec<Output<T>, T::MaxOutputCount>,
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;
            let origin = RawOrigin::Signed(sender);
            Self::run_process_internal(origin, process, inputs, outputs)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::delete_token())]
        pub fn delete_token(origin: OriginFor<T>, token_id: <T as Config>::TokenId) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            ensure_signed(origin)?;
            Self::delete_token_internal(token_id)
                .map(|r| r.into())
                .map_err(|e| e.into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(
            T::WeightInfo::run_process(0u32, inputs.len() as u32, outputs.len() as u32) +
            ProcessValidatorWeights::<T>::validate_process_max() -
            ProcessValidatorWeights::<T>::validate_process_min()
        )]
        pub fn run_process_as_root(
            origin: OriginFor<T>,
            process: ProcessId<T>,
            inputs: BoundedVec<Input<T::TokenId>, T::MaxInputCount>,
            outputs: BoundedVec<Output<T>, T::MaxOutputCount>,
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            ensure_root(origin)?;
            let origin = RawOrigin::Root;
            Self::run_process_internal(origin, process, inputs, outputs)
        }
    }
}

impl<T: Config> Pallet<T> {
    fn process_inputs(
        inputs: &BoundedVec<Input<T::TokenId>, T::MaxInputCount>,
    ) -> Result<(Vec<ProcessIO<T>>, Vec<ProcessIO<T>>), Error<T>> {
        let mut io_inputs = Vec::new();
        let mut io_references = Vec::new();
        for input in inputs {
            let token = Self::tokens_by_id(input.inner());

            // if the token isn't valid we should error
            if token.is_none() {
                return Err(Error::<T>::InvalidInput);
            }
            let token = token.unwrap();

            // Make sure the token hasn't already been burnt indicated by having children
            if token.children.is_some() {
                return Err(Error::<T>::AlreadyBurnt);
            }

            let io = ProcessIO::<T> {
                id: token.id,
                roles: token.roles.into(),
                metadata: token.metadata.into(),
            };

            match input {
                Input::Reference(_) => io_references.push(io),
                Input::Token(_) => io_inputs.push(io),
            };
        }

        Ok((io_references, io_inputs))
    }

    fn process_outputs(outputs: &BoundedVec<Output<T>, T::MaxOutputCount>) -> Vec<ProcessIO<T>> {
        let (_, io_outputs) = outputs.iter().fold(
            (LastToken::<T>::get(), Vec::<ProcessIO<T>>::new()),
            |(last, mut outputs), output| {
                let next = Self::next_token(last);
                let output = ProcessIO::<T> {
                    id: next.clone(),
                    roles: output.roles.clone().into(),
                    metadata: output.metadata.clone().into(),
                };
                outputs.push(output);
                (next, outputs)
            },
        );

        io_outputs
    }

    fn next_token(id: T::TokenId) -> T::TokenId {
        id + One::one()
    }

    fn deposit_event(topics: Vec<T::Hash>, event: Event<T>) {
        <frame_system::Pallet<T>>::deposit_event_indexed(&topics, <T as Config>::RuntimeEvent::from(event).into())
    }

    fn run_process_internal(
        origin: RawOrigin<T::AccountId>,
        process: ProcessId<T>,
        inputs: BoundedVec<Input<T::TokenId>, T::MaxInputCount>,
        outputs: BoundedVec<Output<T>, T::MaxOutputCount>,
    ) -> frame_support::dispatch::DispatchResultWithPostInfo {
        use frame_support::ensure;

        // Get the current block number
        let now = <frame_system::Pallet<T>>::block_number();

        let (io_references, io_inputs) = Self::process_inputs(&inputs)?;
        let io_outputs = Self::process_outputs(&outputs);

        let process_is_valid =
            T::ProcessValidator::validate_process(&process, &origin, &io_references, &io_inputs, &io_outputs);
        ensure!(process_is_valid.success, Error::<T>::ProcessInvalid);

        let graveyard_state = Self::current_graveyard_state();

        // STORAGE MUTATIONS

        // Burn inputs
        let children: BoundedVec<T::TokenId, T::MaxOutputCount> =
            io_outputs.iter().map(|output| output.id.clone()).try_collect().unwrap();
        io_inputs.iter().enumerate().for_each(|(index, input)| {
            <TokensById<T>>::mutate(input.id, |token| {
                let token = token.as_mut().unwrap();
                token.children = Some(children.clone());
                token.destroyed_at = Some(now);
            });
            let graveyard_insert_index = graveyard_state.end_index + (index as u64);
            <Graveyard<T>>::insert(graveyard_insert_index, input.id);
        });

        // update graveyard state
        let graveyard_state = GraveyardState {
            start_index: graveyard_state.start_index,
            end_index: graveyard_state.end_index + (io_inputs.len() as u64),
        };
        <CurrentGraveyardState<T>>::put(graveyard_state);

        let last = io_outputs.last();
        // Update last token if we created new tokens
        if let Some(last) = last {
            <LastToken<T>>::put(last.id);
        }

        // parents only include the burnt tokens not references
        // this is to make sure parent's children match the child's parents
        let parents = BoundedVec::truncate_from(io_inputs.iter().map(|i| i.id).collect());

        // Mint outputs
        io_outputs.into_iter().for_each(|output| {
            <TokensById<T>>::insert(
                output.id.clone(),
                Token::<T> {
                    id: output.id,
                    roles: output.roles.try_into().unwrap(),
                    creator: origin.clone(),
                    created_at: now,
                    destroyed_at: None,
                    metadata: output.metadata.try_into().unwrap(),
                    parents: parents.clone(),
                    children: None,
                },
            );
        });

        let actual_weight =
            T::WeightInfo::run_process(io_references.len() as u32, io_inputs.len() as u32, outputs.len() as u32)
                + ProcessValidatorWeights::<T>::validate_process(process_is_valid.executed_len)
                - ProcessValidatorWeights::<T>::validate_process_min();

        // EVENTS
        let process_id = &process.id;
        let process_version = &process.version;
        Self::deposit_event(
            vec![
                T::Hashing::hash_of(&b"utxoNFT.ProcessRan"),
                T::Hashing::hash_of(&(b"utxoNFT.ProcessRan", process_id)),
                T::Hashing::hash_of(&(b"utxoNFT.ProcessRan", process_id, process_version)),
            ],
            Event::ProcessRan {
                sender: origin,
                process,
                inputs: BoundedVec::truncate_from(inputs.iter().map(|i| i.inner()).collect()),
                outputs: children,
            },
        );

        Ok(Some(actual_weight).into())
    }

    fn delete_token_internal(token_id: <T as Config>::TokenId) -> Result<(), Error<T>> {
        use frame_support::ensure;

        let token = Self::tokens_by_id(token_id);

        if token.is_none() {
            return Ok(());
        }
        let destroyed_at = token.unwrap().destroyed_at;

        ensure!(destroyed_at.is_some(), Error::<T>::NotBurnt);
        let destroyed_at = destroyed_at.unwrap();

        let now = <frame_system::Pallet<T>>::block_number();

        ensure!(
            now - destroyed_at >= T::TokenTombstoneDuration::get(),
            Error::<T>::BurntTooRecently
        );

        <TokensById<T>>::remove(token_id);
        Self::deposit_event(
            vec![
                T::Hashing::hash_of(&b"utxoNFT.DeleteToken"),
                T::Hashing::hash_of(&(b"utxoNFT.DeleteToken", token_id)),
            ],
            Event::TokenDeleted { token_id },
        );

        Ok(())
    }
}
