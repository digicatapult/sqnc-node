#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

pub trait WeightInfo {
    fn run_process(i: u32, o: u32) -> Weight;
    fn delete_token() -> Weight;
}

impl WeightInfo for () {
    fn run_process(_: u32, _: u32) -> Weight {
        Weight::from_parts(0,0)
    }
    fn delete_token() -> Weight {
        Weight::from_parts(0, 0)
    }
}
