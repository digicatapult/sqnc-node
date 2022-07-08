use super::*;
use crate::tests::{Event as TestEvent, ProcessIdentifier};
use crate::Error;
use crate::Event::*;
use crate::{Process, ProcessModel, ProcessStatus, Restriction::None, VersionModel};
use frame_support::bounded_vec;
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};

const PROCESS_ID: ProcessIdentifier = ProcessIdentifier::A;

#[test]
fn returns_error_if_origin_validation_fails_and_no_data_added() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::disable_process(Origin::none(), PROCESS_ID, 1u32),
            DispatchError::BadOrigin,
        );
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn returns_error_if_process_does_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1u32),
            Error::<Test>::NonExistingProcess,
        );
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn returns_error_if_process_is_already_disabled() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(PROCESS_ID, 1u32);
        <ProcessModel<Test>>::insert(
            PROCESS_ID,
            1u32,
            Process {
                status: ProcessStatus::Disabled,
                restrictions: bounded_vec![{ None }]
            }
        );
        assert_noop!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1),
            Error::<Test>::AlreadyDisabled,
        );
        assert_eq!(System::events().len(), 0);
    });
}

#[test]
fn disables_process_and_dispatches_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(PROCESS_ID, 1u32);
        <ProcessModel<Test>>::insert(
            PROCESS_ID,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![{ None }]
            }
        );
        assert_ok!(ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1u32,));
        let expected = TestEvent::ProcessValidation(ProcessDisabled(PROCESS_ID, 1));
        assert_eq!(System::events()[0].event, expected);
    });
}

#[test]
fn disables_process_and_dispatches_event_previous_version() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(PROCESS_ID, 2u32);
        <ProcessModel<Test>>::insert(
            PROCESS_ID,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![{ None }]
            }
        );
        <ProcessModel<Test>>::insert(
            PROCESS_ID,
            2u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![{ None }]
            }
        );
        assert_ok!(ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1u32,));
        let expected = TestEvent::ProcessValidation(ProcessDisabled(PROCESS_ID, 1));
        assert_eq!(System::events()[0].event, expected);
    });
}
