use super::*;
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};
use frame_support::{dispatch::RawOrigin, traits::Get, BoundedVec};
use frame_system::pallet_prelude::BlockNumberFor;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_runtime::{BoundedBTreeMap, Weight};

/// The log target.
const TARGET: &'static str = "runtime::symmetric-key::migration";

pub mod v2 {
    use super::*;

    /// Migrate the symmetric-key pallet from V1 to V2.
    pub struct MigrateToV2<T>(core::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
        fn on_runtime_upgrade() -> Weight {
            let version = StorageVersion::get::<Pallet<T>>();
            if version != 1 {
                log::warn!(
                  target: TARGET,
                  "skipping v1 to v2 migration: executed on wrong storage version. Expected version 1, found {:?}",
                  version,
                );
                return T::DbWeight::get().reads(1);
            }

            Pallet::<T>::migrate_v1_to_v2() + T::DbWeight::get().reads(1)
        }
    }
}

impl<T: Config> Pallet<T> {
    fn migrate_v1_to_v2() -> Weight {
        let mut count = 0u64;
        <TokensById<T>>::translate_values(
            |old_value: TokenOld<
                T::MaxRoleCount,
                T::AccountId,
                T::RoleKey,
                T::TokenId,
                BlockNumberFor<T>,
                T::MaxMetadataCount,
                T::TokenMetadataKey,
                T::TokenMetadataValue,
                T::MaxInputCount,
                T::MaxOutputCount,
            >| {
                count += 1;
                Some(Token::<T> {
                    id: old_value.id,
                    roles: old_value.roles,
                    creator: RawOrigin::Signed(old_value.creator),
                    created_at: old_value.created_at,
                    destroyed_at: old_value.destroyed_at,
                    metadata: old_value.metadata,
                    parents: old_value.parents,
                    children: old_value.children,
                })
            },
        );

        StorageVersion::new(2).put::<Pallet<T>>();
        T::DbWeight::get().reads(count) + T::DbWeight::get().writes(count + 1)
    }
}

#[derive(Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone)]
#[scale_info(skip_type_params(MaxRoleCount, MaxMetadataCount, MaxParentCount, MaxChildCount))]
struct TokenOld<
    MaxRoleCount: Get<u32>,
    AccountId,
    RoleKey: Ord,
    TokenId,
    BlockNumber,
    MaxMetadataCount: Get<u32>,
    TMK: Ord,
    TokenMetadataValue,
    MaxParentCount: Get<u32>,
    MaxChildCount: Get<u32>,
> {
    pub(crate) id: TokenId,
    pub(crate) roles: BoundedBTreeMap<RoleKey, AccountId, MaxRoleCount>,
    pub(crate) creator: AccountId,
    pub(crate) created_at: BlockNumber,
    pub(crate) destroyed_at: Option<BlockNumber>,
    pub(crate) metadata: BoundedBTreeMap<TMK, TokenMetadataValue, MaxMetadataCount>,
    pub(crate) parents: BoundedVec<TokenId, MaxParentCount>,
    pub(crate) children: Option<BoundedVec<TokenId, MaxChildCount>>, // children is the only mutable component of the token
}

#[cfg(test)]
mod test {
    use super::{v2::MigrateToV2, *};
    use crate::tests::mock::*;
    use frame_support::migration::put_storage_value;
    use frame_support::{pallet_prelude::*, StorageHasher};
    use sp_runtime::{bounded_btree_map, bounded_vec};

    type TokenOldTest =
        TokenOld<ConstU32<2>, u64, Role, u64, u64, ConstU32<4>, u64, MetadataValue<u64>, ConstU32<5>, ConstU32<5>>;

    type TokenTest = crate::token::Token<
        ConstU32<2>,
        u64,
        Role,
        u64,
        u64,
        ConstU32<4>,
        u64,
        MetadataValue<u64>,
        ConstU32<5>,
        ConstU32<5>,
    >;

    fn get_expected_weight<T: Config>(count: u64) -> Weight {
        let read_count = count + 1;
        let write_count = match count {
            0 => 0,
            count => count + 1,
        };
        T::DbWeight::get().reads(read_count) + T::DbWeight::get().writes(write_count)
    }

    fn get_process_model_key_hash(id: u64) -> Vec<u8> {
        let key_hashed = id.using_encoded(Blake2_128Concat::hash);

        let mut final_key = Vec::with_capacity(key_hashed.len());

        final_key.extend_from_slice(key_hashed.as_ref());

        final_key
    }

    #[test]
    fn migration_v1_to_v2_works_with_no_entries() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<UtxoNFT>();

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();

            assert_eq!(weight, get_expected_weight::<Test>(0));
        })
    }

    #[test]
    fn migration_v1_to_v2_works_with_single_entry() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<UtxoNFT>();

            put_storage_value(
                b"UtxoNFT",
                b"TokensById",
                get_process_model_key_hash(1).as_slice(),
                TokenOldTest {
                    id: 1,
                    roles: bounded_btree_map!(Default::default() => 1),
                    creator: 2,
                    created_at: 3,
                    destroyed_at: None,
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![],
                    children: None,
                },
            );

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();
            assert_eq!(weight, get_expected_weight::<Test>(1));

            let value = <TokensById<Test>>::get(1);
            assert_eq!(
                value,
                Some(TokenTest {
                    id: 1,
                    roles: bounded_btree_map!(Default::default() => 1),
                    creator: RawOrigin::Signed(2),
                    created_at: 3,
                    destroyed_at: None,
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![],
                    children: None,
                })
            );

            let storage_version = StorageVersion::get::<UtxoNFT>();
            assert_eq!(storage_version, StorageVersion::new(2));
        })
    }

    #[test]
    fn migration_v1_to_v2_works_with_multiple_entries() {
        new_test_ext().execute_with(|| {
            // Assume that we are at V1 as that's the initial state
            StorageVersion::new(1).put::<UtxoNFT>();

            put_storage_value(
                b"UtxoNFT",
                b"TokensById",
                get_process_model_key_hash(1).as_slice(),
                TokenOldTest {
                    id: 1,
                    roles: bounded_btree_map!(Default::default() => 1),
                    creator: 2,
                    created_at: 3,
                    destroyed_at: Some(4),
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![],
                    children: Some(bounded_vec![2]),
                },
            );

            put_storage_value(
                b"UtxoNFT",
                b"TokensById",
                get_process_model_key_hash(2).as_slice(),
                TokenOldTest {
                    id: 2,
                    roles: bounded_btree_map!(Default::default() => 2),
                    creator: 3,
                    created_at: 4,
                    destroyed_at: None,
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![1],
                    children: None,
                },
            );

            // do the runtime upgrade
            let weight = MigrateToV2::<Test>::on_runtime_upgrade();
            assert_eq!(weight, get_expected_weight::<Test>(2));

            let value = <TokensById<Test>>::get(1);
            assert_eq!(
                value,
                Some(TokenTest {
                    id: 1,
                    roles: bounded_btree_map!(Default::default() => 1),
                    creator: RawOrigin::Signed(2),
                    created_at: 3,
                    destroyed_at: Some(4),
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![],
                    children: Some(bounded_vec![2]),
                })
            );

            let value = <TokensById<Test>>::get(2);
            assert_eq!(
                value,
                Some(TokenTest {
                    id: 2,
                    roles: bounded_btree_map!(Default::default() => 2),
                    creator: RawOrigin::Signed(3),
                    created_at: 4,
                    destroyed_at: None,
                    metadata: bounded_btree_map!(0 => MetadataValue::Literal([0])),
                    parents: bounded_vec![1],
                    children: None,
                })
            );

            let storage_version = StorageVersion::get::<UtxoNFT>();
            assert_eq!(storage_version, StorageVersion::new(2));
        })
    }
}
