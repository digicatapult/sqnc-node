#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, MaxEncodedLen};
use dscp_pallet_traits as traits;
use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessValidator, ValidateProcessWeights};
use frame_support::{
    traits::{Get, TryCollect},
    BoundedVec
};
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

mod token;

mod output;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{ensure, pallet_prelude::*, Parameter};
    use frame_system::pallet_prelude::*;

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
            Self::TokenMetadataValue
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
    }

    // Define some derived types off of the Config trait to clean up declarations later

    // ProcessIdentifier can be pulled off of the configured ProcessValidator
    type ProcessIdentifier<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >>::ProcessIdentifier;

    // ProcessVersion can be pulled off of the configured ProcessValidator
    type ProcessVersion<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >>::ProcessVersion;

    // Construct ProcessId
    type ProcessId<T> = ProcessFullyQualifiedId<ProcessIdentifier<T>, ProcessVersion<T>>;

    // The specific Token is derived from Config and the generic Token struct in this crate
    type Token<T> = token::Token<
        <T as Config>::MaxRoleCount,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenId,
        <T as frame_system::Config>::BlockNumber,
        <T as Config>::MaxMetadataCount,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue,
        <T as Config>::MaxInputCount,
        <T as Config>::MaxOutputCount
    >;

    // The specific ProcessIO type can be derived from Config
    type Output<T> = output::Output<
        <T as Config>::MaxRoleCount,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::MaxMetadataCount,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >;

    // The specific ProcessIO type can be derived from Config
    type ProcessIO<T> = traits::ProcessIO<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >;

    type ProcessValidatorWeights<T> = <<T as Config>::ProcessValidator as ProcessValidator<
        <T as Config>::TokenId,
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >>::Weights;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

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
        OptionQuery
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A token was issued.
        Minted(T::TokenId, T::AccountId, BoundedVec<T::TokenId, T::MaxInputCount>),
        /// A token was burnt.
        Burnt(T::TokenId, T::AccountId, BoundedVec<T::TokenId, T::MaxOutputCount>)
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Mutation was attempted on token not owned by origin
        NotOwned,
        /// Mutation was attempted on token that has already been burnt
        AlreadyBurnt,
        /// Token mint was attempted with too many metadata items
        TooManyMetadataItems,
        /// Token mint was attempted without setting a default role
        NoDefaultRole,
        /// Index for the consumed token to set as parent is out of bounds
        OutOfBoundsParent,
        /// Attempted to set the same parent on multiple tokens to mint
        DuplicateParents,
        /// Process failed validation checks
        ProcessInvalid,
        /// An invalid input token id was provided
        InvalidInput
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(
            T::WeightInfo::run_process(inputs.len() as u32, outputs.len() as u32) +
            ProcessValidatorWeights::<T>::validate_process_max() -
            ProcessValidatorWeights::<T>::validate_process_min()
        )]
        pub fn run_process(
            origin: OriginFor<T>,
            process: ProcessId<T>,
            inputs: BoundedVec<T::TokenId, T::MaxInputCount>,
            outputs: BoundedVec<Output<T>, T::MaxOutputCount>
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;
            // Get the current block number
            let now = <frame_system::Pallet<T>>::block_number();
            // Helper closures function
            let _next_token = |id: T::TokenId| -> T::TokenId { id + One::one() };

            // Fetch all valid inputs and ensure all inputs provided exist
            let storage_inputs = inputs.iter().map_while(|i| Self::tokens_by_id(i)).collect::<Vec<_>>();
            ensure!(storage_inputs.len() == inputs.len(), Error::<T>::InvalidInput);

            // Map storage inputs to ProcessIO for validation and ensure all inputs are not burnt
            let io_inputs = storage_inputs
                .into_iter()
                .map_while(|token| match token.children {
                    Some(_) => None,
                    None => Some(ProcessIO::<T> {
                        id: token.id,
                        roles: token.roles.into(),
                        metadata: token.metadata.into()
                    })
                })
                .collect::<Vec<_>>();
            ensure!(io_inputs.len() == inputs.len(), Error::<T>::AlreadyBurnt);

            let (last, io_outputs) = outputs.iter().fold(
                (LastToken::<T>::get(), Vec::<ProcessIO<T>>::new()),
                |(last, mut outputs), output| {
                    let next = _next_token(last);
                    let output = ProcessIO::<T> {
                        id: next.clone(),
                        roles: output.roles.clone().into(),
                        metadata: output.metadata.clone().into()
                    };
                    outputs.push(output);
                    (next, outputs)
                }
            );

            let process_is_valid = T::ProcessValidator::validate_process(process, &sender, &io_inputs, &io_outputs);
            ensure!(process_is_valid.success, Error::<T>::ProcessInvalid);

            // STORAGE MUTATIONS

            // Burn inputs
            let children: BoundedVec<T::TokenId, T::MaxOutputCount> =
                io_outputs.iter().map(|output| output.id.clone()).try_collect().unwrap();
            io_inputs.iter().for_each(|input| {
                <TokensById<T>>::mutate(input.id, |token| {
                    let mut token = token.as_mut().unwrap();
                    token.children = Some(children.clone());
                    token.destroyed_at = Some(now);
                });
            });

            // Mint outputs
            io_outputs.into_iter().for_each(|output| {
                <TokensById<T>>::insert(
                    output.id.clone(),
                    Token::<T> {
                        id: output.id,
                        roles: output.roles.try_into().unwrap(),
                        creator: sender.clone(),
                        created_at: now,
                        destroyed_at: None,
                        metadata: output.metadata.try_into().unwrap(),
                        parents: inputs.clone().try_into().unwrap(),
                        children: None
                    }
                );
            });

            // Update last token
            <LastToken<T>>::put(last);

            // EVENTS

            // Emit events
            for token_id in children.iter() {
                Self::deposit_event(Event::Minted(*token_id, sender.clone(), inputs.clone()));
            }
            for token_id in inputs.iter() {
                Self::deposit_event(Event::Burnt(*token_id, sender.clone(), children.clone()));
            }

            let actual_weight = T::WeightInfo::run_process(inputs.len() as u32, outputs.len() as u32)
                + ProcessValidatorWeights::<T>::validate_process(process_is_valid.executed_len)
                - ProcessValidatorWeights::<T>::validate_process_min();

            Ok(Some(actual_weight).into())
        }
    }
}
