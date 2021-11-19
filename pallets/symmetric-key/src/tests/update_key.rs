use super::*;

#[test]
fn first_test() {
	new_test_ext().execute_with(|| {
		assert_eq!(0, 0);
	});
}
