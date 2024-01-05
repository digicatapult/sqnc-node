use frame_support::Parameter;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

use crate::Restriction;

#[derive(Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq, Serialize, Deserialize)]
pub enum BooleanOperator {
    Null,         // false
    Identity,     // true
    TransferL,    // A
    TransferR,    // B
    NotL,         // !A
    NotR,         // !B
    And,          // A and B
    Nand,         // !(A and B)
    Or,           // A or B
    Nor,          // !(A or B)
    Xor,          // (A and !B) or (!A and B)
    Xnor,         // A equals B
    ImplicationL, // if(A) then B else true
    ImplicationR, // if(B) then A else true
    InhibitionL,  // A and !B
    InhibitionR,  // B and !A
}

impl Default for BooleanOperator {
    fn default() -> Self {
        BooleanOperator::Null
    }
}

#[derive(Encode, Decode, Debug, Clone, MaxEncodedLen, TypeInfo, PartialEq, Serialize, Deserialize)]
pub enum BooleanExpressionSymbol<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> {
    Op(BooleanOperator),
    Restriction(Restriction<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>),
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator> Default
    for BooleanExpressionSymbol<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>
where
    RoleKey: Parameter + Default + Ord,
    TokenMetadataKey: Parameter + Default + Ord,
    TokenMetadataValue: Parameter + Default,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue>,
{
    fn default() -> Self {
        BooleanExpressionSymbol::Restriction(Restriction::Fail)
    }
}

impl BooleanOperator {
    pub fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Self::Identity => true,
            Self::Null => false,
            Self::TransferL => a,
            Self::TransferR => b,
            Self::NotL => !a,
            Self::NotR => !b,
            Self::And => a & b,
            Self::Nand => !(a & b),
            Self::Or => a | b,
            Self::Nor => !(a | b),
            Self::Xor => a ^ b,
            Self::Xnor => a == b,
            Self::ImplicationL => {
                if a {
                    b
                } else {
                    true
                }
            }
            Self::ImplicationR => {
                if b {
                    a
                } else {
                    true
                }
            }
            Self::InhibitionL => a & !b,
            Self::InhibitionR => b & !a,
        }
    }
}
