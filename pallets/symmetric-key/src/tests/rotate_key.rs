use super::*;

use frame_support::{assert_err, assert_ok, dispatch::DispatchError};

#[test]
fn rotate_key_as_root() {
    new_test_ext().execute_with(|| {
        let init_key = (0..32).collect::<Vec<u8>>();
        let new_key = vec![
            83, 89, 77, 77, 69, 84, 82, 73, 67, 95, 75, 69, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0
        ];
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();
        System::set_block_number(1);

        assert_ok!(SymmetricKey::rotate_key(Origin::root()));
        assert_eq!(SymmetricKey::key(), new_key);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            Event::pallet_symmetric_key(pallet_symmetric_key::Event::UpdateKey(new_key)),
        )
    });
}

#[test]
fn rotate_key_not_as_root() {
    new_test_ext().execute_with(|| {
        let init_key = (0..32).collect::<Vec<u8>>();
        SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

        assert_err!(SymmetricKey::rotate_key(Origin::signed(42)), DispatchError::BadOrigin);
        assert_eq!(SymmetricKey::key(), init_key.clone());
    });
}
