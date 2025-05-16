use parity_scale_codec::{Decode, Encode, MaxEncodedLen};

use scale_info::TypeInfo;
use sp_std::prelude::*;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxRoleCount, MaxMetadataCount))]
pub enum Input<TokenId> {
    Reference(TokenId),
    Token(TokenId),
}

impl<T> Input<T>
where
    T: Clone,
{
    pub fn inner(&self) -> T {
        match self {
            Input::Reference(id) => id.clone(),
            Input::Token(id) => id.clone(),
        }
    }
}

impl<T> Clone for Input<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Input::Reference(id) => Input::Reference(id.clone()),
            Input::Token(id) => Input::Token(id.clone()),
        }
    }
}

impl<T> sp_std::fmt::Debug for Input<T>
where
    T: sp_std::fmt::Debug,
{
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        match self {
            Input::Reference(id) => f.debug_struct("InputReference").field("id", id).finish(),
            Input::Token(id) => f.debug_struct("InputToken").field("id", id).finish(),
        }
    }
}

impl<T> PartialEq<Input<T>> for Input<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Input<T>) -> bool {
        match (self, other) {
            (Input::Reference(a), Input::Reference(b)) => a == b,
            (Input::Token(a), Input::Token(b)) => a == b,
            _ => false,
        }
    }
}
