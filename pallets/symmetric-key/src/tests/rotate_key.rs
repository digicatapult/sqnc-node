use super::*;

use frame_support::{
	assert_ok, assert_err,
	dispatch::{
		DispatchError
	}
};

#[test]
fn rotate_key_as_root() {
	new_test_ext().execute_with(|| {
		let init_key = (0..32).collect::<Vec<u8>>();
		SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

		assert_ok!(SymmetricKey::rotate_key(Origin::root()));
		assert_eq!(SymmetricKey::key(), vec![83, 89, 77, 77, 69, 84, 82, 73, 67, 95, 75, 69, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
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
