use super::*;
use crate::Error;
use crate::{Process, ProcessModel, ProcessStatus, Restriction::None, Version, VersionModel};
use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};

const PROCESS_ID: [u8; 32] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[test]
fn returns_error_if_origin_validation_fails_and_no_data_added() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ProcessValidation::disable_process(Origin::none(), PROCESS_ID, 1,),
            DispatchError::BadOrigin,
        );
    });
}

#[test]
fn returns_error_if_process_does_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1),
            Error::<Test>::NonExistingProcess,
        );
    });
}

#[test]
fn returns_error_if_process_is_already_disabled() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(PROCESS_ID, Version { version: 1 });
        <ProcessModel<Test>>::insert(
            PROCESS_ID,
            1,
            Process {
                status: ProcessStatus::Disabled,
                restrictions: [{ None }].to_vec(),
            },
        );
        assert_noop!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1),
            Error::<Test>::AlreadyDisabled,
        );
    });
}

#[test]
fn disables_process_and_dispatches_event() {
    new_test_ext().execute_with(|| {});
}
