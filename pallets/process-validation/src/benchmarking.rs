// ! Benchmarking setup for pallet-template

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::bounded_vec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as ProcessValidation;

// TODO implement benchmarking
benchmarks! {
  create_process {
  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), bounded_vec![])
  verify {
  }

  disable_process {
  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), One::one())
  verify {
  }
}

impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
