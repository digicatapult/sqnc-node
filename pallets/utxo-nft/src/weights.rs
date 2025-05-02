#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

pub trait WeightInfo {
    fn run_process(r: u32, i: u32, o: u32) -> Weight;
    fn delete_token() -> Weight;
}

impl WeightInfo for () {
    fn run_process(_: u32, _: u32, _: u32) -> Weight {
        Weight::from_parts(0, 0)
    }
    fn delete_token() -> Weight {
        Weight::from_parts(0, 0)
    }
}
