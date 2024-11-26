#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};
use sqnc_pallet_traits::ValidateProcessWeights;

use crate::Config;

pub trait WeightInfo: ValidateProcessWeights<u32> {
    fn create_process(i: u32) -> Weight;
    fn disable_process() -> Weight;
    fn validate_process(p: u32) -> Weight;
    fn validate_process_min() -> Weight;
    fn validate_process_max() -> Weight;
}

impl WeightInfo for () {
    fn create_process(_: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn disable_process() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn validate_process(_p: u32) -> Weight {
        Weight::from_parts(0, 0)
    }

    fn validate_process_min() -> Weight {
        Weight::from_parts(0, 0)
    }

    fn validate_process_max() -> Weight {
        Weight::from_parts(0, 0)
    }
}
