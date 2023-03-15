#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{Parameter, RuntimeDebug};

use frame_support::codec::MaxEncodedLen;
use frame_support::sp_runtime::traits::AtLeast32Bit;
use scale_info::TypeInfo;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::prelude::*;

pub struct ProcessIO<IoIdentifier, AccountId, RoleKey: Ord, TokenMetadataKey: Ord, TokenMetadataValue> {
    pub id: IoIdentifier,
    pub roles: BTreeMap<RoleKey, AccountId>,
    pub metadata: BTreeMap<TokenMetadataKey, TokenMetadataValue>,
    pub parent_index: Option<u32>
}

#[derive(Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
pub struct ProcessFullyQualifiedId<
    ProcessIdentifier: Parameter + MaxEncodedLen,
    ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen
> {
    pub id: ProcessIdentifier,
    pub version: ProcessVersion
}

pub trait ProcessValidator<I, A, R, T, V>
where
    I: Parameter,
    A: Parameter,
    R: Parameter + Ord,
    T: Parameter + Ord,
    V: Parameter
{
    type ProcessIdentifier: Parameter + MaxEncodedLen + Encode + Decode;
    type ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen + Encode + Decode;

    fn validate_process(
        id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &A,
        inputs: &Vec<ProcessIO<I, A, R, T, V>>,
        outputs: &Vec<ProcessIO<I, A, R, T, V>>
    ) -> bool;
}

impl<I, A, R, T, V> ProcessValidator<I, A, R, T, V> for ()
where
    I: Parameter,
    A: Parameter,
    R: Parameter + Ord,
    T: Parameter + Ord,
    V: Parameter
{
    type ProcessIdentifier = ();
    type ProcessVersion = u32;

    fn validate_process(
        _id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        _sender: &A,
        _inputs: &Vec<ProcessIO<I, A, R, T, V>>,
        _outputs: &Vec<ProcessIO<I, A, R, T, V>>
    ) -> bool {
        true
    }
}
