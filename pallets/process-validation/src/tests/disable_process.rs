use super::*;

use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};

const PROCESS_ID: [u8; 32] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];


#[test]
fn returns_error_if_origin_validation_fails_and_no_data_added() {
    new_test_ext().execute_with(|| {
    });
}

#[test]
fn returns_error_if_process_does_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        println!("{:?}", System::events());
        // tmp error assertation
        assert!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1).is_err()
        );
        // ideally
        /*
        assert_noop!(
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 1).is_err()
            DispatchError:ErrorName,
        );
         */ 
    });
}

#[test]
fn returns_error_if_process_is_already_disabled() {
    new_test_ext().execute_with(|| {});
}

#[test]
fn disables_process_and_dispatches_event() {
    new_test_ext().execute_with(|| {});
}
