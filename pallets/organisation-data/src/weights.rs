#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for validator_set.
pub trait WeightInfo {
  fn set_value() -> Weight;
}


// For backwards compatibility and tests
impl WeightInfo for () {
  fn set_value() -> Weight {
      Weight::zero()
  } 
}

