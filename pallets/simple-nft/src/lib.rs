#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
pub use pallet::*;
use sp_runtime::traits::{AtLeast32Bit, One};
/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;
use frame_support::migration::{take_storage_value, put_storage_value};
use frame_support::Hashable;
use codec::Codec;
use sp_std::collections::btree_map::BTreeMap;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct OldToken<AccountId, TokenId, BlockNumber, TokenMetadata> {
    id: TokenId,
    owner: AccountId,
    creator: AccountId,
    block_number: BlockNumber,
    metadata: TokenMetadata,
    parents: Vec<TokenId>,
    children: Option<Vec<TokenId>>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Token<AccountId, TokenId, BlockNumber, TokenMetadata> {
    id: TokenId,
    owner: AccountId,
    creator: AccountId,
    created_at: BlockNumber,
    destroyed_at: Option<BlockNumber>,
    metadata: TokenMetadata,
    parents: Vec<TokenId>,
    children: Option<Vec<TokenId>>, // children is the only mutable component of the token
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
        type TokenMetadata: Parameter + Default + Copy;

        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            frame_support::debug::info!("Upgrading runtime");

            let last_token = LastToken::<T>::get();
            let mut token_id = T::TokenId::default();

            // Read all the tokens into a map
            let mut old_tokens: BTreeMap<T::TokenId, OldToken<T::AccountId, T::TokenId, T::BlockNumber, T::TokenMetadata>> = BTreeMap::new();
            while token_id < last_token {
                token_id = token_id + One::one();
                let key_hash = token_id.blake2_128_concat();
                let old_token: OldToken<T::AccountId, T::TokenId, T::BlockNumber, T::TokenMetadata> = take_storage_value(b"SimpleNFTModule", b"TokensById", &key_hash).unwrap();
                old_tokens.insert(token_id, old_token);
            }

            // Update and replace the token in storage
            for (token_id, old_token) in old_tokens.clone() {
                let key_hash = token_id.blake2_128_concat();
                let new_token = Token {
                    id: token_id,
                    owner: old_token.owner,
                    creator: old_token.creator,
                    created_at: old_token.block_number,
                    destroyed_at: match old_token.children.clone() {
                        None => { None }
                        Some(arr) => { 
                            if arr.len() > 0 { Some(old_tokens.get(&arr[0]).unwrap().block_number) }
                            else { Some(<frame_system::Module<T>>::block_number()) }
                        }
                    },
                    metadata: old_token.metadata,
                    parents: old_token.parents,
                    children: old_token.children,
                };

                put_storage_value(b"SimpleNFTModule", b"TokensById", &key_hash, new_token);
            }

            // Return the weight consumed by the migration.
            (50_000_000 as Weight)
                .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(old_tokens.len() as Weight)))
                .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(old_tokens.len() as Weight)))
        }
    }

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
        Token<T::AccountId, T::TokenId, T::BlockNumber, T::TokenMetadata>,
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
            outputs: Vec<(T::AccountId, T::TokenMetadata)>,
        ) -> DispatchResultWithPostInfo {
            // Check it was signed and get the signer
            let sender = ensure_signed(origin)?;
            // Get the current block number
            let now = <frame_system::Module<T>>::block_number();
            // Helper closures function
            let _next_token = |id: T::TokenId| -> T::TokenId { id + One::one() };

            // TODO: add extra checks that origin is allowed to create tokens generically

            // INPUT VALIDATION

            // check origin owns inputs and that inputs have not been burnt
            for id in inputs.iter() {
                let token = <TokensById<T>>::get(id);
                ensure!(token.owner == sender, Error::<T>::NotOwned);
                ensure!(token.children == None, Error::<T>::AlreadyBurnt);
            }

            // STORAGE MUTATIONS

            // Get the last token to be created so we can iterate the new tokens
            let last = LastToken::<T>::get();

            // Create new tokens getting a tuple of the last token created and the complete Vec of tokens created
            let (last, children) =
                outputs
                    .iter()
                    .fold((last, Vec::new()), |(last, children), (owner, metadata)| {
                        let next = _next_token(last);
                        <TokensById<T>>::insert(
                            next,
                            Token {
                                id: next,
                                owner: owner.clone(),
                                creator: sender.clone(),
                                created_at: now,
                                destroyed_at: None,
                                metadata: metadata.clone(),
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
