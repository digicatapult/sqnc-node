use super::*;

use frame_support::assert_ok;
use vitalam_pallet_traits::ProcessValidator;

const PROCESS_ID: [u8; 32] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[test]
fn returns_error_if_process_does_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ProcessValidation::disable_process(
            Origin::root(),
            PROCESS_ID,
            1,
        ));
    });
}

#[test]
fn returns_error_if_process_is_already_disabled() {
    new_test_ext().execute_with(|| {

    });
}

#[test]
fn if_process_exists_but_wrong_version_returns_latest() {
    new_test_ext().execute_with(|| {

    });

}

#[test]
fn disables_process_and_dispatches_event() {
    new_test_ext().execute_with(|| {

    });
}
