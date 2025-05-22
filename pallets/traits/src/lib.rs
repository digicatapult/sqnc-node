#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::RawOrigin;
use frame_support::weights::Weight;
use frame_support::Parameter;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};

use frame_support::sp_runtime::traits::AtLeast32Bit;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::prelude::*;

#[derive(Clone)]
pub struct ProcessIO<IoIdentifier, AccountId, RoleKey, TokenMetadataKey, TokenMetadataValue>
where
    RoleKey: Ord,
    TokenMetadataKey: Ord,
{
    pub id: IoIdentifier,
    pub roles: BTreeMap<RoleKey, AccountId>,
    pub metadata: BTreeMap<TokenMetadataKey, TokenMetadataValue>,
}

#[derive(Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, TypeInfo, Clone, PartialEq)]
pub struct ProcessFullyQualifiedId<
    ProcessIdentifier: Parameter + MaxEncodedLen,
    ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen,
> {
    pub id: ProcessIdentifier,
    pub version: ProcessVersion,
}

#[derive(PartialEq, RuntimeDebug)]
pub struct ValidationResult<W> {
    pub success: bool,
    pub executed_len: W,
}

pub trait ValidateProcessWeights<WeightArg> {
    fn validate_process(p: WeightArg) -> Weight;
    fn validate_process_min() -> Weight;
    fn validate_process_max() -> Weight;
}

impl ValidateProcessWeights<u32> for () {
    fn validate_process(_: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn validate_process_min() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn validate_process_max() -> Weight {
        Weight::from_parts(0, 0)
    }
}

pub trait ProcessValidator<I, A, R, T, V>
where
    I: Parameter,
    A: Parameter,
    R: Parameter + Ord,
    T: Parameter + Ord,
    V: Parameter,
{
    type ProcessIdentifier: Parameter + MaxEncodedLen + Encode + Decode;
    type ProcessVersion: Parameter + AtLeast32Bit + MaxEncodedLen + Encode + Decode;
    type WeightArg;
    type Weights: ValidateProcessWeights<Self::WeightArg>;

    fn validate_process(
        id: &ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &RawOrigin<A>,
        references: &Vec<ProcessIO<I, A, R, T, V>>,
        inputs: &Vec<ProcessIO<I, A, R, T, V>>,
        outputs: &Vec<ProcessIO<I, A, R, T, V>>,
    ) -> ValidationResult<Self::WeightArg>;
}

impl<I, A, R, T, V> ProcessValidator<I, A, R, T, V> for ()
where
    I: Parameter,
    A: Parameter,
    R: Parameter + Ord,
    T: Parameter + Ord,
    V: Parameter,
{
    type ProcessIdentifier = ();
    type ProcessVersion = u32;
    type WeightArg = u32;
    type Weights = ();

    fn validate_process(
        _id: &ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        _sender: &RawOrigin<A>,
        _references: &Vec<ProcessIO<I, A, R, T, V>>,
        _inputs: &Vec<ProcessIO<I, A, R, T, V>>,
        _outputs: &Vec<ProcessIO<I, A, R, T, V>>,
    ) -> ValidationResult<u32> {
        ValidationResult::<u32> {
            success: true,
            executed_len: 0u32,
        }
    }
}
