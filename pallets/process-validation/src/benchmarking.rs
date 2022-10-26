// ! Benchmarking setup for pallet-template
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::{BoundedVec};
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
    let mut program = BoundedVec::<_, _>::with_bounded_capacity(1);
    program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();

      ProcessValidation::<T>::create_process(
            RawOrigin::Root.into(),
            T::ProcessIdentifier::default(),
            program,
        );
  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), One::one())
  verify {
    // assert_eq!(
      // ProcessModel::<T>::get(T::ProcessIdentifier::default(), One::one()),
      // Process {
      //     status: ProcessStatus::Disabled,
      //     program: BoundedVec::<_, _>::with_bounded_capacity(0),
      // }
  //)
  }
}
impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
