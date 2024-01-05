use super::*;

use crate::tests::RuntimeEvent as TestEvent;
use crate::Error;
use crate::Event;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::{bounded_vec, DispatchError};

#[test]
fn update_key_as_root() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let new_key: BoundedVec<u8, _> = bounded_vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];

        assert_ok!(SymmetricKey::update_key(RuntimeOrigin::root(), new_key.clone()));
        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            TestEvent::SymmetricKey(Event::UpdateKey(new_key)),
        )
    });
}

#[test]
fn update_key_not_as_root() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        let init_key: BoundedVec<u8, _> = bounded_vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        SymmetricKey::update_key(RuntimeOrigin::root(), init_key).unwrap();

        let new_key = bounded_vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31
        ];
        assert_noop!(
            SymmetricKey::update_key(RuntimeOrigin::signed(42), new_key),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn update_key_incorrect_key_length() {
    new_test_ext().execute_with(|| {
        run_to_block(1);

        let init_key: BoundedVec<u8, _> = bounded_vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        SymmetricKey::update_key(RuntimeOrigin::root(), init_key).unwrap();

        let new_key = bounded_vec![1, 2, 3, 4];
        assert_noop!(
            SymmetricKey::update_key(RuntimeOrigin::root(), new_key),
            Error::<Test>::IncorrectKeyLength
        );
    });
}
