#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use codec::{Decode, Encode};
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Token<AccountId, RoleKey, TokenId, BlockNumber, TokenMetadataKey: Ord, TokenMetadataValue> {
    id: TokenId,
    original_id: TokenId,
    roles: BTreeMap<RoleKey, AccountId>,
    creator: AccountId,
    created_at: BlockNumber,
    destroyed_at: Option<BlockNumber>,
    metadata: BTreeMap<TokenMetadataKey, TokenMetadataValue>,
    parents: Vec<TokenId>,
    children: Option<Vec<TokenId>>, // children is the only mutable component of the token
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Output<AccountId, RoleKey, TokenMetadataKey: Ord, TokenMetadataValue> {
    roles: BTreeMap<RoleKey, AccountId>,
    metadata: BTreeMap<TokenMetadataKey, TokenMetadataValue>,
    parent_index: Option<u32>,
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

        type TokenId: Parameter + AtLeast32Bit + Default + Copy + Codec;
        type RoleKey: Parameter + Default + Ord;

        type TokenMetadataKey: Parameter + Default + Ord;
        type TokenMetadataValue: Parameter + Default;

        type WeightInfo: WeightInfo;

        // Maximum number of metadata items allowed per token
        #[pallet::constant]
        type MaxMetadataCount: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Storage value definition
    #[pallet::storage]
    #[pallet::getter(fn last_token)]
    pub(super) type LastToken<T: Config> = StorageValue<_, T::TokenId, ValueQuery>;

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn tokens_by_id)]
    pub(super) type TokensById<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::TokenId,
        Token<T::AccountId, T::RoleKey, T::TokenId, T::BlockNumber, T::TokenMetadataKey, T::TokenMetadataValue>,
        ValueQuery, /*, DefaultForExampleStorage*/
    >;

    #[pallet::event]
    #[pallet::metadata(TokenId<T> = "TokenId", T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A token was issued.
        Minted(T::TokenId, T::AccountId, Vec<T::TokenId>),
        /// A token was burnt.
        Burnt(T::TokenId, T::AccountId, Vec<T::TokenId>),
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
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // The value of the weight is an arbitrary value, for now
        // #[weight = 10_000]
        #[pallet::weight(T::WeightInfo::run_process(inputs.len(), outputs.len()))]
        pub(super) fn run_process(
            origin: OriginFor<T>,
            inputs: Vec<T::TokenId>,
            outputs: Vec<Output<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;
            // Get the current block number
            let now = <frame_system::Module<T>>::block_number();
            // Helper closures function
            let _next_token = |id: T::TokenId| -> T::TokenId { id + One::one() };

            // TODO: add extra checks that origin is allowed to create tokens generically

            // INPUT VALIDATION

            // check multiple tokens are not trying to have the same parent
            let mut parent_indices = BTreeSet::new();

            for output in outputs.iter() {
                // check at least a default role has been set
                ensure!(
                    output.roles.contains_key(&T::RoleKey::default()),
                    Error::<T>::NoDefaultRole
                );

                // check metadata count
                ensure!(
                    output.metadata.len() <= T::MaxMetadataCount::get() as usize,
                    Error::<T>::TooManyMetadataItems
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
                ensure!(token.roles[&T::RoleKey::default()] == sender, Error::<T>::NotOwned);
                ensure!(token.children == None, Error::<T>::AlreadyBurnt);
            }

            // STORAGE MUTATIONS

            // Get the last token to be created so we can iterate the new tokens
            let last = LastToken::<T>::get();

            // Create new tokens getting a tuple of the last token created and the complete Vec of tokens created
            let (last, children) = outputs.iter().fold((last, Vec::new()), |(last, children), output| {
                let next = _next_token(last);
                let original_id = if output.parent_index.is_some() {
                    inputs.get(output.parent_index.unwrap() as usize).unwrap().clone()
                } else {
                    next
                };
                <TokensById<T>>::insert(
                    next,
                    Token {
                        id: next,
                        original_id: original_id,
                        roles: output.roles.clone(),
                        creator: sender.clone(),
                        created_at: now,
                        destroyed_at: None,
                        metadata: output.metadata.clone(),
                        parents: inputs.clone(),
                        children: None,
                    },
                );
                let mut next_children = children.clone();
                next_children.push(next);
                (next, next_children)
            });

            // Burn inputs
            inputs.iter().for_each(|id| {
                <TokensById<T>>::mutate(id, |token| {
                    (*token).children = Some(children.clone());
                    (*token).destroyed_at = Some(now);
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
