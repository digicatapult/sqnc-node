#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{Parameter, RuntimeDebug};

use frame_support::codec::MaxEncodedLen;
use frame_support::{
    BoundedBTreeMap,
    sp_runtime::traits::AtLeast32Bit,
    traits::{Get}
};
use scale_info::TypeInfo;
use sp_std::prelude::*;

#[derive(Encode, Decode, Default, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxRoleCount, MaxMetadataCount))]
pub struct ProcessIO<
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

impl<MR, A, R, MM, TK, TV> Clone for ProcessIO<MR, A, R, MM, TK, TV>
where
    MR: Get<u32>, MM: Get<u32>, R: Ord, TK: Ord,
    BoundedBTreeMap<R, A, MR>: Clone,
    BoundedBTreeMap<TK, TV, MM>: Clone
{
	fn clone(&self) -> Self {
		ProcessIO {
            roles: self.roles.clone(),
            metadata: self.metadata.clone(),
            parent_index: self.parent_index.clone()
        }
	}
}

impl<MR, A, R, MM, TK, TV> sp_std::fmt::Debug for ProcessIO<MR, A, R, MM, TK, TV>
where
    MR: Get<u32>, MM: Get<u32>, R: Ord, TK: Ord,
	BoundedBTreeMap<R, A, MR>: sp_std::fmt::Debug,
    BoundedBTreeMap<TK, TV, MM>: sp_std::fmt::Debug
{
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		f.debug_struct("ProcessIO")
            .field("roles", &self.roles)
            .field("metadata", &self.metadata)
            .field("parent_index", &self.parent_index)
            .finish()
	}
}

impl<MR, A, R, MM, TK, TV> PartialEq<ProcessIO<MR, A, R, MM, TK, TV>> for ProcessIO<MR, A, R, MM, TK, TV>
where
    R: Ord, TK: Ord,
	BoundedBTreeMap<R, A, MR>: PartialEq,
    BoundedBTreeMap<TK, TV, MM>: PartialEq,
	MR: Get<u32>,
	MM: Get<u32>,
{
	fn eq(&self, other: &ProcessIO<MR, A, R, MM, TK, TV>) -> bool {
        self.roles == other.roles &&
        self.metadata == other.metadata &&
        self.parent_index == other.parent_index
	}
}

#[derive(Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
pub struct ProcessFullyQualifiedId<ProcessIdentifier: Parameter + MaxEncodedLen, ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen> {
    pub id: ProcessIdentifier,
    pub version: ProcessVersion
}

pub trait ProcessValidator<RC, A, R, MC, T, V>
where
    RC: Get<u32>,
    A: Parameter,
    R: Parameter + Ord,
    MC: Get<u32>,
    T: Parameter + Ord,
    V: Parameter
{
    type ProcessIdentifier: Parameter + MaxEncodedLen + Encode + Decode;
    type ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen  + Encode + Decode;

    fn validate_process(
        id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &A,
        inputs: &Vec<ProcessIO<RC, A, R, MC, T, V>>,
        outputs: &Vec<ProcessIO<RC, A, R, MC, T, V>>
    ) -> bool;
}

impl<RC, A, R, MC, T, V> ProcessValidator<RC, A, R, MC, T, V> for ()
where
    RC: Get<u32>,
    A: Parameter,
    R: Parameter + Ord,
    MC: Get<u32>,
    T: Parameter + Ord,
    V: Parameter
{
    type ProcessIdentifier = ();
    type ProcessVersion = u32;

    fn validate_process(
        _id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        _sender: &A,
        _inputs: &Vec<ProcessIO<RC, A, R, MC, T, V>>,
        _outputs: &Vec<ProcessIO<RC, A, R, MC, T, V>>
    ) -> bool {
        true
    }
}
