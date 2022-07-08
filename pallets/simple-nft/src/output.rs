use codec::{Decode, Encode};

use frame_support::codec::MaxEncodedLen;
use frame_support::{traits::Get, BoundedBTreeMap};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[derive(Encode, Decode, Default, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxRoleCount, MaxMetadataCount))]
pub struct Output<
    MaxRoleCount: Get<u32>,
    AccountId,
    RoleKey: Ord,
    MaxMetadataCount: Get<u32>,
    TokenMetadataKey: Ord,
    TokenMetadataValue
> {
    pub roles: BoundedBTreeMap<RoleKey, AccountId, MaxRoleCount>,
    pub metadata: BoundedBTreeMap<TokenMetadataKey, TokenMetadataValue, MaxMetadataCount>,
    pub parent_index: Option<u32>
}

impl<MR, A, R, MM, TK, TV> Clone for Output<MR, A, R, MM, TK, TV>
where
    MR: Get<u32>,
    MM: Get<u32>,
    R: Ord,
    TK: Ord,
    BoundedBTreeMap<R, A, MR>: Clone,
    BoundedBTreeMap<TK, TV, MM>: Clone
{
    fn clone(&self) -> Self {
        Output {
            roles: self.roles.clone(),
            metadata: self.metadata.clone(),
            parent_index: self.parent_index.clone()
        }
    }
}

impl<MR, A, R, MM, TK, TV> sp_std::fmt::Debug for Output<MR, A, R, MM, TK, TV>
where
    MR: Get<u32>,
    MM: Get<u32>,
    R: Ord,
    TK: Ord,
    BoundedBTreeMap<R, A, MR>: sp_std::fmt::Debug,
    BoundedBTreeMap<TK, TV, MM>: sp_std::fmt::Debug
{
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        f.debug_struct("Output")
            .field("roles", &self.roles)
            .field("metadata", &self.metadata)
            .field("parent_index", &self.parent_index)
            .finish()
    }
}

impl<MR, A, R, MM, TK, TV> PartialEq<Output<MR, A, R, MM, TK, TV>> for Output<MR, A, R, MM, TK, TV>
where
    R: Ord,
    TK: Ord,
    BoundedBTreeMap<R, A, MR>: PartialEq,
    BoundedBTreeMap<TK, TV, MM>: PartialEq,
    MR: Get<u32>,
    MM: Get<u32>
{
    fn eq(&self, other: &Output<MR, A, R, MM, TK, TV>) -> bool {
        self.roles == other.roles && self.metadata == other.metadata && self.parent_index == other.parent_index
    }
}
