use super::*;

use crate::Error;
use frame_support::{
	assert_ok, assert_noop,
	dispatch::{
		DispatchError
	}
};

#[test]
fn update_key_as_root() {
	new_test_ext().execute_with(|| {
		let new_key = (0..32).collect::<Vec<u8>>();
		assert_ok!(SymmetricKey::update_key(Origin::root(), new_key.clone()));
		assert_eq!(SymmetricKey::key(), new_key.clone());
	});
}

#[test]
fn update_key_not_as_root() {
	new_test_ext().execute_with(|| {
		let init_key = (0..32).collect::<Vec<u8>>();
		SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

		let new_key = (32..64).collect::<Vec<u8>>();
		assert_noop!(SymmetricKey::update_key(Origin::signed(42), new_key.clone()), DispatchError::BadOrigin);
	});
}

#[test]
fn update_key_incorrect_key_length() {
	new_test_ext().execute_with(|| {
		let init_key = (0..32).collect::<Vec<u8>>();
		SymmetricKey::update_key(Origin::root(), init_key.clone()).unwrap();

		let new_key = (32..63).collect::<Vec<u8>>();
		assert_noop!(SymmetricKey::update_key(Origin::root(), new_key.clone()), Error::<Test>::IncorrectKeyLength);
	});
}
