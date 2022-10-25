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
        Weight::from_ref_time(0 as u64)
    }
    fn disable_process() -> Weight {
        Weight::from_ref_time(0 as u64)
    }
}

impl WeightInfo for () {
    fn create_process() -> Weight {
        Weight::from_ref_time(0 as u64)
    }
    fn disable_process() -> Weight {
        Weight::from_ref_time(0 as u64)
    }
}
