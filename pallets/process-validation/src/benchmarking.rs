// ! Benchmarking setup for pallet-template
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as ProcessValidation;

benchmarks! {
  create_process {
    // valid programs have x Restrictions and (x-1) Ops, therefore number of BooleanExpressionSymbol to add is always odd
    let r in 1 .. (1 + T::MaxProcessProgramLength::get() / 2);

    let mut program = BoundedVec::<_, _>::with_bounded_capacity(T::MaxProcessProgramLength::get() as usize);
    program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();

    for j in 0..(r - 1) {
        program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();
        program.try_push(BooleanExpressionSymbol::Op(BooleanOperator::And)).unwrap();
    }

  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), program.clone())
  verify {
    let version = VersionModel::<T>::get(T::ProcessIdentifier::default());
    let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
    assert_eq!(process.status, ProcessStatus::Enabled);
    assert_eq!(process.program, program);
  }

  disable_process {
    let mut program = BoundedVec::<_, _>::with_bounded_capacity(T::MaxProcessProgramLength::get() as usize);
    program.try_push(BooleanExpressionSymbol::Restriction(Restriction::None)).unwrap();

      ProcessValidation::<T>::create_process(
            RawOrigin::Root.into(),
            T::ProcessIdentifier::default(),
            program.clone(),
        ).unwrap();
  }: _(RawOrigin::Root, T::ProcessIdentifier::default(), 1u32.into())
  verify {
    let version = VersionModel::<T>::get(T::ProcessIdentifier::default());
    let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
    assert_eq!(process.status, ProcessStatus::Disabled);
    assert_eq!(process.program, program);
  }
}
impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
