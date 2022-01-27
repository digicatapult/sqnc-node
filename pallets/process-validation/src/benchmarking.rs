// ! Benchmarking setup for pallet-template

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};

#[allow(unused)]
use crate::Module as ProcessValidation;

// TODO implement benchmarking
benchmarks! {
  create_process {
  }: _(RawOrigin::Root)
  verify {
  }

  disable_process {
  }: _(RawOrigin::Root)
  verify {
  }
}

impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
