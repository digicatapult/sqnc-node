// ! Benchmarking setup for pallet-template

use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::bounded_vec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as ProcessValidation;

// TODO implement benchmarking
benchmarks! {
  // create_process {
  // }: _(RawOrigin::Root, T::ProcessIdentifier::default(), bounded_vec![])
  // verify {
  // }

  disable_process {
      ProcessValidation::<T>::create_process(
            RawOrigin::Root.into(),
            T::ProcessIdentifier::default(),
            bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::None)],
        );
  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), One::one())
  verify {
    // assert_eq!(
      // ProcessModel::<T>::get(T::ProcessIdentifier::default(), One::one()),
      // Process {
      //     status: ProcessStatus::Disabled,
      //     program: bounded_vec![BooleanExpressionSymbol::Restriction(Restriction::None)]
      // }
  //)
  }
}
impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
