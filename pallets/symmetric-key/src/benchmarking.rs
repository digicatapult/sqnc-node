//! Benchmarking setup for pallet-template

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

#[allow(unused)]
use crate::Pallet as SymmetricKey;

benchmarks! {
  update_key {
    let key = (0..32).collect::<Vec<u8>>();
  }: _(RawOrigin::Root, key.clone().try_into().unwrap())
  verify {
    assert_eq!(Key::<T>::get(), key);
  }

  rotate_key {

  }: _(RawOrigin::Root)
}

impl_benchmark_test_suite!(SymmetricKey, crate::tests::new_test_ext(), crate::tests::Test,);
