// ! Benchmarking setup for pallet-template
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as ProcessValidation;

benchmarks! {
  create_process {
    let i in 1..200;

    let mut program = BoundedVec::<_, _>::with_bounded_capacity(i as usize);
    for j in 0..i {
      if j == 0 {
        program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();
      }
      // add every other loop to have valid postfix notation
      else if j % 2 == 0 {
        program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();
        program.try_push(BooleanExpressionSymbol::Op(BooleanOperator::And)).unwrap();
      }
    }

  }: _(RawOrigin::Root,
  T::ProcessIdentifier::default(),
  program.clone())
  verify {
    // let version = ProcessValidation::<T>::get_version(&T::ProcessIdentifier::default());
    // let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
    // assert_eq!(process.status, ProcessStatus::Enabled)
  }

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
    // let version = ProcessValidation::<T>::get_version(&T::ProcessIdentifier::default());
    // let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
    // assert_eq!(process.status, ProcessStatus::Disabled)
  }
}
impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
