#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_symmetric_key`.
pub trait WeightInfo {
    fn update_key() -> Weight;
    fn rotate_key() -> Weight;
}

impl WeightInfo for () {
    fn update_key() -> Weight {
        Weight::from_parts(0, 0)
    }
    fn rotate_key() -> Weight {
        Weight::from_parts(0, 0)
    }
}
