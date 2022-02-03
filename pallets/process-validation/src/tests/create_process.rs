use super::*;
use frame_system::{pallet_prelude::*, Config};
use frame_support::{assert_ok};
use rand;
use crate::Restriction::None;

// fn if process does not exists - should return a version number of 1
// validate origin 
// get latest version
// deposit event
// fn calls validates payload

#[test]
fn create_process_test() {
    const AccountId: [u8; 32] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
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
