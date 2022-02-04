use super::*;

use frame_support::assert_ok;

const PROCESS_ID: [u8; 32] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[test]
fn create_process_simple() {
    new_test_ext().execute_with(|| {
        println!(
            "{:?}",
            ProcessValidation::disable_process(Origin::root(), PROCESS_ID, 0)
        );
    });
}
