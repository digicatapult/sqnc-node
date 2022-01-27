#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn create_process() -> Weight;
    fn disable_process() -> Weight;
}

/// TODO implement weights after benchmarking
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn create_process() -> Weight {
        (0 as Weight)
    }
    fn disable_process() -> Weight {
        (0 as Weight)
    }
}

impl WeightInfo for () {
    fn create_process() -> Weight {
        (0 as Weight)
    }
    fn disable_process() -> Weight {
        (0 as Weight)
    }
}
