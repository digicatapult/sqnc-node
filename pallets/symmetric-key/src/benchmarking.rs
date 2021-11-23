//! Benchmarking setup for pallet-template

use super::*;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};

#[allow(unused)]
use crate::Module as SymmetricKey;

const SEED: u32 = 0;

benchmarks! {
  update_key {
    let key = (0..32).collect::<Vec<u8>>();
  }: _(RawOrigin::Root(), key)
  verify {
    assert_eq!(Key::<T>::get(), key);
  }
}

impl_benchmark_test_suite!(SymmetricKey, crate::tests::new_test_ext(), crate::tests::Test,);
