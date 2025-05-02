// ! Benchmarking setup for pallet-template
use super::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use sp_std::collections::btree_map::BTreeMap;

#[allow(unused)]
use crate::Pallet as ProcessValidation;

type BooleanExpressionSymbol<T> = crate::BooleanExpressionSymbol<
    <T as Config>::RoleKey,
    <T as Config>::TokenMetadataKey,
    <T as Config>::TokenMetadataValue,
    <T as Config>::TokenMetadataValueDiscriminator,
>;

type ProcessIO<T> = sqnc_pallet_traits::ProcessIO<
    <T as Config>::TokenId,
    <T as frame_system::Config>::AccountId,
    <T as Config>::RoleKey,
    <T as Config>::TokenMetadataKey,
    <T as Config>::TokenMetadataValue,
>;

type ProcessFullyQualifiedId<T> =
    sqnc_pallet_traits::ProcessFullyQualifiedId<<T as Config>::ProcessIdentifier, <T as Config>::ProcessVersion>;

fn prepare_program<T: Config>(l: u32) -> BoundedVec<BooleanExpressionSymbol<T>, T::MaxProcessProgramLength> {
    let metadata_restriction = Restriction::MatchArgsMetadataValue {
        left_arg_type: ArgType::Input,
        left_index: 0u32,
        left_metadata_key: Default::default(),
        right_arg_type: ArgType::Input,
        right_index: 0u32,
        right_metadata_key: Default::default(),
    };

    let mut program = BoundedVec::<_, _>::with_bounded_capacity(T::MaxProcessProgramLength::get() as usize);
    program
        .try_push(BooleanExpressionSymbol::<T>::Restriction(Restriction::None))
        .unwrap();

    for _ in 0..(l - 1) {
        program
            .try_push(BooleanExpressionSymbol::<T>::Restriction(metadata_restriction.clone()))
            .unwrap();
        program
            .try_push(BooleanExpressionSymbol::<T>::Op(BooleanOperator::And))
            .unwrap();
    }

    program
}

fn create_process_fixture<T: Config>(
    program: &BoundedVec<BooleanExpressionSymbol<T>, T::MaxProcessProgramLength>,
) -> ProcessFullyQualifiedId<T> {
    ProcessValidation::<T>::create_process(RawOrigin::Root.into(), T::ProcessIdentifier::default(), program.clone())
        .unwrap();

    ProcessFullyQualifiedId::<T> {
        id: T::ProcessIdentifier::default(),
        version: VersionModel::<T>::get(T::ProcessIdentifier::default()),
    }
}

benchmarks! {
    create_process {
      // valid programs have x Restrictions and (x-1) Ops, therefore number of BooleanExpressionSymbol to add is always odd
      let r in 1 .. (1 + T::MaxProcessProgramLength::get() / 2);
      let program = prepare_program::<T>(r);
    }: _(RawOrigin::Root, T::ProcessIdentifier::default(), program.clone())
    verify {
      let version = VersionModel::<T>::get(T::ProcessIdentifier::default());
      let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
      assert_eq!(process.status, ProcessStatus::Enabled);
      assert_eq!(process.program, program);
    }

    disable_process {
        let program = prepare_program::<T>(1);
        let process = create_process_fixture::<T>(&program);
    }: _(RawOrigin::Root, process.id, process.version)
    verify {
        let version = VersionModel::<T>::get(T::ProcessIdentifier::default());
        let process = ProcessModel::<T>::get(T::ProcessIdentifier::default(), version);
        assert_eq!(process.status, ProcessStatus::Disabled);
        assert_eq!(process.program, program);
    }

    validate_process {
        let r in 1 .. (1 + T::MaxProcessProgramLength::get() / 2);

        let origin: RawOrigin<T::AccountId> = RawOrigin::Signed(account("owner", 0, 0));
        let program = prepare_program::<T>(r);
        let process = create_process_fixture::<T>(&program);

        let args = vec![ProcessIO::<T> {
            id: Default::default(),
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(Default::default(), Default::default())])
        }; 10];
    }: {
        let _ = ProcessValidation::<T>::validate_process(&process, &origin, &args, &args, &args);
    }

    validate_process_min {
        let origin: RawOrigin<T::AccountId> = RawOrigin::Signed(account("owner", 0, 0));
        let program = prepare_program::<T>(1);
        let process = create_process_fixture::<T>(&program);

        let args = vec![ProcessIO::<T> {
            id: Default::default(),
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(Default::default(), Default::default())])
        }; 10];
    }: {
        let _ = ProcessValidation::<T>::validate_process(&process, &origin, &args, &args, &args);
    }

    validate_process_max {
        let origin: RawOrigin<T::AccountId> = RawOrigin::Signed(account("owner", 0, 0));
        let program = prepare_program::<T>(1 + T::MaxProcessProgramLength::get() / 2);
        let process = create_process_fixture::<T>(&program);

        let args = vec![ProcessIO::<T> {
            id: Default::default(),
            roles: BTreeMap::new(),
            metadata: BTreeMap::from_iter(vec![(Default::default(), Default::default())])
        }; 10];
    }: {
        let _ = ProcessValidation::<T>::validate_process(&process, &origin, &args, &args, &args);
    }
}

impl_benchmark_test_suite!(ProcessValidation, crate::mock::new_test_ext(), crate::mock::Test,);
