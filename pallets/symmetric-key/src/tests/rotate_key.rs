use super::*;

use crate::tests::Event as TestEvent;
use crate::Event;
use frame_support::{assert_err, assert_ok, bounded_vec, dispatch::DispatchError};

#[test]
fn rotate_key_as_root() {
    new_test_ext().execute_with(|| {
        let init_key = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        let new_key = bounded_vec![
            83, 89, 77, 77, 69, 84, 82, 73, 67, 95, 75, 69, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
        ];
        SymmetricKey::update_key(Origin::root(), init_key).unwrap();
        System::set_block_number(1);

        assert_ok!(SymmetricKey::rotate_key(Origin::root()));
        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            TestEvent::SymmetricKey(Event::UpdateKey(new_key)),
        )
    });
}

#[test]
fn rotate_key_not_as_root() {
    new_test_ext().execute_with(|| {
        let init_key: BoundedVec<u8, ConstU32<32>> = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

        assert_err!(SymmetricKey::rotate_key(Origin::signed(42)), DispatchError::BadOrigin);
        assert_eq!(SymmetricKey::key(), init_key);
    });
}
