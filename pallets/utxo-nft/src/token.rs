use frame_support::{dispatch::RawOrigin, traits::Get, BoundedBTreeMap, BoundedVec};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// A FRAME pallet for handling non-fungible tokens
use sp_std::prelude::*;

#[derive(Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone)]
#[scale_info(skip_type_params(MaxRoleCount, MaxMetadataCount, MaxParentCount, MaxChildCount))]
pub struct Token<
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
    pub(crate) creator: RawOrigin<AccountId>,
    pub(crate) created_at: BlockNumber,
    pub(crate) destroyed_at: Option<BlockNumber>,
    pub(crate) metadata: BoundedBTreeMap<TMK, TokenMetadataValue, MaxMetadataCount>,
    pub(crate) parents: BoundedVec<TokenId, MaxParentCount>,
    pub(crate) children: Option<BoundedVec<TokenId, MaxChildCount>>, // children is the only mutable component of the token
}

impl<MRC, A, RK, T, BN, MMC, TMK, TMV, MPC, MCC> Default for Token<MRC, A, RK, T, BN, MMC, TMK, TMV, MPC, MCC>
where
    MRC: Get<u32>,
    RK: Ord,
    T: Default,
    BN: Default,
    MMC: Get<u32>,
    TMK: Ord,
    MPC: Get<u32>,
    MCC: Get<u32>,
{
    fn default() -> Self {
        Self {
            id: Default::default(),
            roles: Default::default(),
            creator: RawOrigin::None,
            created_at: Default::default(),
            destroyed_at: Default::default(),
            metadata: Default::default(),
            parents: Default::default(),
            children: Default::default(),
        }
    }
}

impl<MR, A, RK, TID, BN, MM, TK, TV, MP, MC> PartialEq<Token<MR, A, RK, TID, BN, MM, TK, TV, MP, MC>>
    for Token<MR, A, RK, TID, BN, MM, TK, TV, MP, MC>
where
    BoundedBTreeMap<RK, A, MR>: PartialEq,
    BoundedBTreeMap<TK, TV, MM>: PartialEq,
    BoundedVec<TID, MP>: PartialEq,
    BoundedVec<TID, MC>: PartialEq,
    TID: PartialEq,
    A: PartialEq,
    BN: PartialEq,
    RK: Ord,
    TK: Ord,
    MR: Get<u32>,
    MM: Get<u32>,
    MP: Get<u32>,
    MC: Get<u32>,
{
    fn eq(&self, other: &Token<MR, A, RK, TID, BN, MM, TK, TV, MP, MC>) -> bool {
        self.id == other.id
            && self.roles == other.roles
            && self.creator == other.creator
            && self.created_at == other.created_at
            && self.destroyed_at == other.destroyed_at
            && self.metadata == other.metadata
            && self.parents == other.parents
            && self.children == other.children
    }
}
