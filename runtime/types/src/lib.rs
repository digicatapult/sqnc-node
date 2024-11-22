#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{traits::ConstU32, BoundedVec};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};
use strum_macros::EnumDiscriminants;

pub use pallet_process_validation::{BooleanExpressionSymbol, BooleanOperator, Restriction};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

pub type TokenId = u128;
pub type TokenMetadataKey = BoundedVec<u8, ConstU32<32>>;
pub type TokenMetadataValue = MetadataValue<TokenId>;
pub type ProcessIdentifier = BoundedVec<u8, ConstU32<32>>;
pub type ProcessVersion = u32;
pub type MaxProcessProgramLength = ConstU32<501>;

pub type RuntimeExpressionSymbol =
    BooleanExpressionSymbol<Role, TokenMetadataKey, TokenMetadataValue, MetadataValueType>;
pub type RuntimeRestriction = Restriction<Role, TokenMetadataKey, TokenMetadataValue, MetadataValueType>;
pub type RuntimeProgram = BoundedVec<RuntimeExpressionSymbol, MaxProcessProgramLength>;

pub type Role = BoundedVec<u8, ConstU32<32>>;

#[derive(
    Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq, Debug, Eq, EnumDiscriminants, Serialize, Deserialize,
)]
#[strum_discriminants(derive(Encode, Decode, MaxEncodedLen, TypeInfo, Serialize, Deserialize))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(MetadataValueType))]
pub enum MetadataValue<TokenId> {
    File(Hash),
    Literal(BoundedVec<u8, ConstU32<32>>),
    TokenId(TokenId),
    Integer(i128),
    None,
}

impl<T> Default for MetadataValue<T> {
    fn default() -> Self {
        MetadataValue::None
    }
}
impl Default for MetadataValueType {
    fn default() -> Self {
        MetadataValueType::None
    }
}

impl<T: PartialEq> PartialEq<T> for MetadataValue<T> {
    fn eq(&self, rhs: &T) -> bool {
        match self {
            MetadataValue::<T>::TokenId(v) => v == rhs,
            _ => false,
        }
    }
}
