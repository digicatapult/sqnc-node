// use super::{Config, LastToken, Token};
// use codec::{Decode, Encode};
// use frame_support::migration::{put_storage_value, take_storage_value};
// use frame_support::Hashable;
// use frame_support::{
//     traits::{Get, GetPalletVersion, PalletVersion},
//     weights::Weight,
// };
// use sp_runtime::traits::One;
// use sp_std::collections::btree_map::BTreeMap;
// use sp_std::prelude::*;

// #[derive(Encode, Decode, Default, Clone, PartialEq)]
// #[cfg_attr(feature = "std", derive(Debug))]
// pub struct OldToken<AccountId, TokenId, BlockNumber, TokenMetadata> {
//     id: TokenId,
//     owner: AccountId,
//     creator: AccountId,
//     block_number: BlockNumber,
//     metadata: TokenMetadata,
//     parents: Vec<TokenId>,
//     children: Option<Vec<TokenId>>,
// }

// pub fn on_runtime_upgrade<T: Config, P: GetPalletVersion>() -> frame_support::weights::Weight {
//     let maybe_storage_version = <P as GetPalletVersion>::storage_version();
//     frame_support::debug::info!(
//         "Running migration for pallet simple_nft with storage version {:?}",
//         maybe_storage_version
//     );

//     match maybe_storage_version {
//         Some(storage_version) if storage_version < PalletVersion::new(2, 0, 0) => {
//             let last_token = LastToken::<T>::get();
//             let mut token_id = T::TokenId::default();

//             // Read all the tokens into a map
//             let mut old_tokens: BTreeMap<
//                 T::TokenId,
//                 OldToken<T::AccountId, T::TokenId, T::BlockNumber, T::TokenMetadata>,
//             > = BTreeMap::new();
//             while token_id < last_token {
//                 token_id = token_id + One::one();
//                 let key_hash = token_id.blake2_128_concat();
//                 let old_token: OldToken<T::AccountId, T::TokenId, T::BlockNumber, T::TokenMetadata> =
//                     take_storage_value(b"SimpleNFTModule", b"TokensById", &key_hash).unwrap();
//                 old_tokens.insert(token_id, old_token);
//             }

//             // Update and replace the token in storage
//             for (token_id, old_token) in old_tokens.clone() {
//                 let key_hash = token_id.blake2_128_concat();
//                 let new_token = Token {
//                     id: token_id,
//                     owner: old_token.owner,
//                     creator: old_token.creator,
//                     created_at: old_token.block_number,
//                     destroyed_at: match old_token.children.clone() {
//                         None => None,
//                         Some(arr) => {
//                             if arr.len() > 0 {
//                                 Some(old_tokens.get(&arr[0]).unwrap().block_number)
//                             } else {
//                                 Some(<frame_system::Module<T>>::block_number())
//                             }
//                         }
//                     },
//                     metadata: old_token.metadata,
//                     parents: old_token.parents,
//                     children: old_token.children,
//                 };

//                 put_storage_value(b"SimpleNFTModule", b"TokensById", &key_hash, new_token);
//             }

//             // Return the weight consumed by the migration.
//             (50_000_000 as Weight)
//                 .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(old_tokens.len() as Weight)))
//                 .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(old_tokens.len() as Weight)))
//         }
//         _ => {
//             frame_support::debug::warn!(
//                 "Attempted to apply migration to V2 but failed because storage version is {:?}",
//                 maybe_storage_version
//             );
//             0
//         }
//     }
// }

// #[cfg(test)]
// mod migration_tests {
//     use super::OldToken;
//     use crate::mock::{new_test_ext, SimpleNFTModule, Test as T};
//     use crate::{LastToken, Token, TokensById};

//     use codec::Encode;
//     use frame_support::migration::put_storage_value;
//     use frame_support::traits::{OnRuntimeUpgrade, PalletVersion, PALLET_VERSION_STORAGE_KEY_POSTFIX};
//     use frame_support::Hashable;

//     const PALLET_V100: PalletVersion = PalletVersion {
//         major: 1,
//         minor: 0,
//         patch: 0,
//     };
//     const PALLET_NAME: &str = "SimpleNFTModule";

//     /// Returns the storage key for `PalletVersion` for the given `pallet`.
//     fn get_pallet_version_storage_key_for_pallet(pallet: &str) -> [u8; 32] {
//         let pallet_name = sp_io::hashing::twox_128(pallet.as_bytes());
//         let postfix = sp_io::hashing::twox_128(PALLET_VERSION_STORAGE_KEY_POSTFIX);

//         let mut final_key = [0u8; 32];
//         final_key[..16].copy_from_slice(&pallet_name);
//         final_key[16..].copy_from_slice(&postfix);

//         final_key
//     }

//     fn set_pallet_version(pallet: &str, version: PalletVersion) {
//         let key = get_pallet_version_storage_key_for_pallet(pallet);
//         sp_io::storage::set(&key, &version.encode());
//     }

//     fn put_old_tokens<T: Encode>(old_tokens: Vec<(u64, T)>) {
//         for (id, old_token) in old_tokens {
//             let key_hash = id.blake2_128_concat();
//             put_storage_value(b"SimpleNFTModule", b"TokensById", &key_hash, old_token);
//         }
//     }

//     #[test]
//     fn it_upgrades_single_token_from_main_successfully() {
//         new_test_ext().execute_with(|| {
//             set_pallet_version(PALLET_NAME, PALLET_V100);

//             // Create one token
//             let old_tokens: Vec<(u64, OldToken<u64, u64, u64, u64>)> = vec![(
//                 1,
//                 OldToken {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     block_number: 0,
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: None,
//                 },
//             )];
//             put_old_tokens(old_tokens);
//             LastToken::<T>::put(1);

//             SimpleNFTModule::on_runtime_upgrade();

//             let new_token = TokensById::<T>::get(1);

//             assert_eq!(
//                 new_token,
//                 Token {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 0,
//                     destroyed_at: None,
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: None
//                 }
//             );
//         });
//     }

//     #[test]
//     fn it_upgrades_with_children_from_main_successfully() {
//         new_test_ext().execute_with(|| {
//             set_pallet_version(PALLET_NAME, PALLET_V100);

//             // Create one token
//             let old_tokens: Vec<(u64, OldToken<u64, u64, u64, u64>)> = vec![
//                 (
//                     1,
//                     OldToken {
//                         id: 1,
//                         owner: 1,
//                         creator: 1,
//                         block_number: 0,
//                         metadata: 42,
//                         parents: Vec::new(),
//                         children: Some(vec![2]),
//                     },
//                 ),
//                 (
//                     2,
//                     OldToken {
//                         id: 2,
//                         owner: 1,
//                         creator: 1,
//                         block_number: 1,
//                         metadata: 42,
//                         parents: vec![1],
//                         children: None,
//                     },
//                 ),
//             ];
//             put_old_tokens(old_tokens);
//             LastToken::<T>::put(2);

//             SimpleNFTModule::on_runtime_upgrade();

//             let new_token_1 = TokensById::<T>::get(1);
//             let new_token_2 = TokensById::<T>::get(2);

//             assert_eq!(
//                 new_token_1,
//                 Token {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 0,
//                     destroyed_at: Some(1),
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: Some(vec![2])
//                 }
//             );

//             assert_eq!(
//                 new_token_2,
//                 Token {
//                     id: 2,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 1,
//                     destroyed_at: None,
//                     metadata: 42,
//                     parents: vec![1],
//                     children: None
//                 }
//             );
//         });
//     }

//     #[test]
//     fn it_upgrades_without_children_from_main_successfully() {
//         new_test_ext().execute_with(|| {
//             set_pallet_version(PALLET_NAME, PALLET_V100);

//             frame_system::Pallet::<T>::set_block_number(10);

//             // Create one token
//             let old_tokens: Vec<(u64, OldToken<u64, u64, u64, u64>)> = vec![
//                 (
//                     1,
//                     OldToken {
//                         id: 1,
//                         owner: 1,
//                         creator: 1,
//                         block_number: 0,
//                         metadata: 42,
//                         parents: Vec::new(),
//                         children: Some(Vec::new()),
//                     },
//                 ),
//                 (
//                     2,
//                     OldToken {
//                         id: 2,
//                         owner: 1,
//                         creator: 1,
//                         block_number: 1,
//                         metadata: 42,
//                         parents: Vec::new(),
//                         children: None,
//                     },
//                 ),
//             ];
//             put_old_tokens(old_tokens);
//             LastToken::<T>::put(2);

//             SimpleNFTModule::on_runtime_upgrade();

//             let new_token_1 = TokensById::<T>::get(1);
//             let new_token_2 = TokensById::<T>::get(2);

//             assert_eq!(
//                 new_token_1,
//                 Token {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 0,
//                     destroyed_at: Some(10),
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: Some(Vec::new())
//                 }
//             );

//             assert_eq!(
//                 new_token_2,
//                 Token {
//                     id: 2,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 1,
//                     destroyed_at: None,
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: None
//                 }
//             );
//         });
//     }

//     #[test]
//     fn it_does_not_upgrade_if_upgraded_successfully() {
//         new_test_ext().execute_with(|| {
//             set_pallet_version(
//                 PALLET_NAME,
//                 PalletVersion {
//                     major: 2,
//                     minor: 0,
//                     patch: 0,
//                 },
//             );

//             // Create one token
//             let tokens: Vec<(u64, Token<u64, u64, u64, u64>)> = vec![(
//                 1,
//                 Token {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 0,
//                     destroyed_at: None,
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: Some(Vec::new()),
//                 },
//             )];
//             put_old_tokens(tokens);
//             LastToken::<T>::put(2);

//             let weight = SimpleNFTModule::on_runtime_upgrade();

//             assert_eq!(weight, 0);

//             let token_1 = TokensById::<T>::get(1);

//             assert_eq!(
//                 token_1,
//                 Token {
//                     id: 1,
//                     owner: 1,
//                     creator: 1,
//                     created_at: 0,
//                     destroyed_at: None,
//                     metadata: 42,
//                     parents: Vec::new(),
//                     children: Some(Vec::new()),
//                 },
//             );
//         });
//     }
// }
