use super::*;
use frame_support::{assert_ok};
use crate::Restriction::None;

// fn if process does not exists - should return a version number of 1
// validate origin 
// get latest version
// deposit event
// fn calls validates payload

// -- fixtures --
const AccountId: [u8; 32] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

#[test]
fn create_process_test() {
    fn create_process_simple() {
        new_test_ext().execute_with(|| {
            assert_ok!(ProcessValidation::create_process(
                Origin::root(),
                AccountId,
                vec![{ None }]
            ));
        });
    }
}
