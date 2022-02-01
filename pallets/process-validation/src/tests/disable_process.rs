use super::*;

use frame_support::assert_ok;

#[test]
fn create_process_simple() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::disable_process(Origin::root()));
    });
}
