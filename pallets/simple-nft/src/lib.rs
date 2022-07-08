#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, MaxEncodedLen};
use dscp_pallet_traits as traits;
use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessValidator};
use frame_support::{traits::Get, BoundedVec};
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};

use sp_std::collections::btree_set::BTreeSet;
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
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type TokenId: Parameter + AtLeast32Bit + Default + Copy + Codec + MaxEncodedLen;
        type RoleKey: Parameter + Default + Ord + MaxEncodedLen;

        type TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen;
        type TokenMetadataValue: Parameter + Default + MaxEncodedLen;

        type WeightInfo: WeightInfo;

        type ProcessValidator: ProcessValidator<
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
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >>::ProcessIdentifier;

    // ProcessVersion can be pulled off of the configured ProcessValidator
    type ProcessVersion<T> = <<T as Config>::ProcessValidator as ProcessValidator<
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
        <T as frame_system::Config>::AccountId,
        <T as Config>::RoleKey,
        <T as Config>::TokenMetadataKey,
        <T as Config>::TokenMetadataValue
    >;

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
        #[pallet::weight(T::WeightInfo::run_process(inputs.len(), outputs.len()))]
        pub fn run_process(
            origin: OriginFor<T>,
            process: Option<ProcessId<T>>,
            inputs: BoundedVec<T::TokenId, T::MaxInputCount>,
            outputs: BoundedVec<Output<T>, T::MaxOutputCount>
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;
            // Get the current block number
            let now = <frame_system::Pallet<T>>::block_number();
            // Helper closures function
            let _next_token = |id: T::TokenId| -> T::TokenId { id + One::one() };

            if let Some(process) = process {
                let inputs: Vec<Option<ProcessIO<T>>> = inputs
                    .iter()
                    .map(|i| {
                        let token = Self::tokens_by_id(i);
                        token.map(|t| ProcessIO::<T> {
                            roles: t.roles.into(),
                            metadata: t.metadata.into(),
                            parent_index: None
                        })
                    })
                    .collect();

                ensure!(
                    inputs.iter().all(|opt_t| {
                        match opt_t {
                            None => false,
                            Some(_) => true
                        }
                    }),
                    Error::<T>::InvalidInput
                );

                let inputs = inputs.into_iter().map(|t| t.unwrap()).collect();
                let outputs = outputs
                    .iter()
                    .map(|o| ProcessIO::<T> {
                        roles: o.roles.clone().into(),
                        metadata: o.metadata.clone().into(),
                        parent_index: o.parent_index.clone()
                    })
                    .collect();

                let process_is_valid = T::ProcessValidator::validate_process(process, &sender, &inputs, &outputs);
                ensure!(process_is_valid, Error::<T>::ProcessInvalid);
            } else {
                // check multiple tokens are not trying to have the same parent
                let mut parent_indices = BTreeSet::new();

                for output in outputs.iter() {
                    // check at least a default role has been set
                    ensure!(
                        output.roles.contains_key(&T::RoleKey::default()),
                        Error::<T>::NoDefaultRole
                    );

                    // check parent index
                    if output.parent_index.is_some() {
                        let index = output.parent_index.unwrap() as usize;
                        ensure!(inputs.get(index).is_some(), Error::<T>::OutOfBoundsParent);
                        ensure!(parent_indices.insert(index), Error::<T>::DuplicateParents);
                    }
                }

                // check origin owns inputs and that inputs have not been burnt
                for id in inputs.iter() {
                    let token = <TokensById<T>>::get(id);
                    ensure!(token != None, Error::<T>::InvalidInput);
                    let token = token.unwrap();
                    ensure!(token.roles[&T::RoleKey::default()] == sender, Error::<T>::NotOwned);
                    ensure!(token.children == None, Error::<T>::AlreadyBurnt);
                }
            }

            // STORAGE MUTATIONS

            // Get the last token to be created so we can iterate the new tokens
            let last = LastToken::<T>::get();

            // Create new tokens getting a tuple of the last token created and the complete Vec of tokens created
            let (last, children) = outputs.iter().fold(
                (last, BoundedVec::<T::TokenId, T::MaxOutputCount>::default()),
                |(last, children), output| {
                    let next = _next_token(last);
                    let original_id = if output.parent_index.is_some() {
                        let parent_id = inputs.get(output.parent_index.unwrap() as usize).unwrap();
                        <TokensById<T>>::get(parent_id).unwrap().original_id.clone()
                    } else {
                        next
                    };
                    <TokensById<T>>::insert(
                        next,
                        Token::<T> {
                            id: next,
                            original_id: original_id,
                            roles: output.roles.clone(),
                            creator: sender.clone(),
                            created_at: now,
                            destroyed_at: None,
                            metadata: output.metadata.clone(),
                            parents: inputs.clone(),
                            children: None
                        }
                    );
                    let mut next_children = children.clone();
                    next_children.force_push(next);
                    (next, next_children)
                }
            );

            // Burn inputs
            inputs.iter().for_each(|id| {
                <TokensById<T>>::mutate(id, |token| {
                    let mut token = token.as_mut().unwrap();
                    token.children = Some(children.clone());
                    token.destroyed_at = Some(now);
                });
            });

            <LastToken<T>>::put(last);

            // EVENTS

            // Emit events
            for token_id in children.iter() {
                Self::deposit_event(Event::Minted(*token_id, sender.clone(), inputs.clone()));
            }
            for token_id in inputs.iter() {
                Self::deposit_event(Event::Burnt(*token_id, sender.clone(), children.clone()));
            }

            Ok(().into())
        }
    }
}
