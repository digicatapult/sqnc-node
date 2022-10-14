use super::*;
use crate::binary_expression_tree::BooleanExpressionSymbol;
use crate::binary_expression_tree::BooleanOperator;
use crate::tests::Event as TestEvent;
use crate::tests::ProcessIdentifier;
use crate::Error;
use crate::Event::*;
use crate::{Process, ProcessModel, ProcessStatus, Restriction::None, VersionModel};
use frame_support::bounded_vec;
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
use sp_runtime::ModuleError;

// -- fixtures --
#[allow(dead_code)]
const PROCESS_ID1: ProcessIdentifier = ProcessIdentifier::A;
const PROCESS_ID2: ProcessIdentifier = ProcessIdentifier::B;

#[test]
fn returns_error_if_origin_validation_fails_and_no_data_added() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::create_process(
                Origin::none(),
                PROCESS_ID1,
                bounded_vec![BooleanExpressionSymbol::Restriction(None)]
            ),
            DispatchError::BadOrigin,
        );
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0u32);
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID1, 1u32), Default::default());
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn handles_if_process_exists_for_the_new_version() {
    new_test_ext().execute_with(|| {
        <ProcessModel<Test>>::insert(
            PROCESS_ID1,
            1,
            Process {
                status: ProcessStatus::Disabled,
                program: bounded_vec![BooleanExpressionSymbol::Restriction(None)]
            }
        );
        let result = ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)]
        );
        assert_noop!(result, Error::<Test>::AlreadyExists);
    });
}

#[test]
fn if_no_version_found_it_should_return_default_and_insert_new_one() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0u32);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            bounded_vec![
                BooleanExpressionSymbol::Restriction(None),
                BooleanExpressionSymbol::Restriction(None),
                BooleanExpressionSymbol::Op(BooleanOperator::And)
            ]
        ));

        let expected = TestEvent::ProcessValidation(ProcessCreated(
            PROCESS_ID1,
            1u32,
            bounded_vec![
                BooleanExpressionSymbol::Restriction(None),
                BooleanExpressionSymbol::Restriction(None),
                BooleanExpressionSymbol::Op(BooleanOperator::And)
            ],
            true
        ));
        assert_eq!(System::events()[0].event, expected);
        assert_eq!(
            <ProcessModel<Test>>::get(PROCESS_ID1, 1u32),
            Process {
                status: ProcessStatus::Enabled,
                program: bounded_vec![
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Op(BooleanOperator::And)
                ]
            }
        );
    });
}

#[test]
fn for_existing_process_it_mutates_an_existing_version() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ProcessValidation::update_version(PROCESS_ID1));
        assert_ok!(ProcessValidation::update_version(PROCESS_ID1));
        assert_ok!(ProcessValidation::update_version(PROCESS_ID1));

        let items: Vec<u32> = <VersionModel<Test>>::iter().map(|item| item.1.clone()).collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0], 3);
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 3u32);
    });
}

#[test]
fn sets_versions_correctly_for_multiple_processes() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let mut ids = [PROCESS_ID2; 10].to_vec();
        ids.extend([PROCESS_ID1; 15].to_vec());
        ids.iter().for_each(|id| -> () {
            assert_ok!(ProcessValidation::update_version(id.clone()));
        });

        let id1_expected = TestEvent::ProcessValidation(ProcessCreated(
            PROCESS_ID1,
            16u32,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
            false
        ));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
        ));
        let id2_expected = TestEvent::ProcessValidation(ProcessCreated(
            PROCESS_ID2,
            11u32,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
            false
        ));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID2,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
        ));

        assert_eq!(System::events()[0].event, id1_expected);
        assert_eq!(System::events()[1].event, id2_expected);
    });
}

#[test]
fn updates_version_correctly_for_existing_proces_and_dispatches_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(PROCESS_ID1, 9u32);
        let expected = TestEvent::ProcessValidation(ProcessCreated(
            PROCESS_ID1,
            10u32,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
            false
        ));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
        ));
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 10u32);
        assert_eq!(System::events()[0].event, expected);
    });
}

#[test]
fn updates_version_correctly_for_new_process_and_dispatches_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
        ));
        let expected = TestEvent::ProcessValidation(ProcessCreated(
            PROCESS_ID1,
            1u32,
            bounded_vec![BooleanExpressionSymbol::Restriction(None)],
            true
        ));
        // sets version to 1 and returns true to identify that this is a new event
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 1u32);
        assert_eq!(System::events()[0].event, expected);
    });
}

#[test]
fn program_invalid_negative_stack() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::create_process(
                Origin::root(),
                PROCESS_ID1,
                bounded_vec![
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Op(BooleanOperator::And),
                ]
            ),
            DispatchError::Module(ModuleError {
                index: 1,
                error: [4, 0, 0, 0],
                message: Some("InvalidProgram")
            }),
        );
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0u32);
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID1, 1u32), Default::default());
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn program_invalid_expect_single_stack() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::create_process(
                Origin::root(),
                PROCESS_ID1,
                bounded_vec![
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Restriction(None),
                    BooleanExpressionSymbol::Op(BooleanOperator::And),
                ]
            ),
            DispatchError::Module(ModuleError {
                index: 1,
                error: [4, 0, 0, 0],
                message: Some("InvalidProgram")
            }),
        );
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0u32);
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID1, 1u32), Default::default());
        assert_eq!(System::events().len(), 0);
    });
}
