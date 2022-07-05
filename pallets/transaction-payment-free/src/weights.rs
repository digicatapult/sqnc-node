#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn from() -> Weight;
}

/// TODO implement weights after benchmarking
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn from() -> Weight {
        (0 as Weight)
    }
}

impl WeightInfo for () {
    fn from() -> Weight {
        (0 as Weight)
    }
}
