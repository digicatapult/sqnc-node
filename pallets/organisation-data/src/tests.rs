#![cfg(test)]

use crate::{mock::*, Error, OrgData, OrgDataCount};
use frame_support::{
    assert_err, assert_ok,
    traits::{ChangeMembers, InitializeMembers},
};

const INIT_MEMBERS: [u64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const NEW_MEMBERS: [u64; 2] = [10, 11];
const MEMBER_1: u64 = 1u64;
const MEMBER_2: u64 = 2u64;
const NOT_MEMBER: u64 = 10u64;

const KEY_1: u8 = 1u8;
const KEY_2: u8 = 2u8;
const KEY_3: u8 = 3u8;
const VALUE: u32 = 42u32;

#[test]
fn it_stores_zero_count_for_members_on_initialise() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);

        let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
        values.sort_by_key(|(m, _)| *m);
        assert_eq!(values, Vec::from(INIT_MEMBERS.map(|m| (m, 0u32))));
    })
}

#[test]
fn it_errors_if_not_member() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        let result = OrganisationData::set_value(RuntimeOrigin::signed(NOT_MEMBER), KEY_1, VALUE);

        assert_err!(result, Error::<Test>::NotMember);
    })
}

#[test]
fn it_succeeds_if_a_member() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        let result = OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE);

        assert_ok!(result);
    })
}

#[test]
fn it_sets_value() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();

        let value = <OrgData<Test>>::get(MEMBER_1, KEY_1);
        assert_eq!(value, VALUE);
    })
}

#[test]
fn it_increments_value_count_1() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();

        let value = <OrgDataCount<Test>>::get(MEMBER_1);
        assert_eq!(value, 1u32);
    })
}

#[test]
fn it_increments_value_count_2_distinct() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_2, VALUE).unwrap();

        let value = <OrgDataCount<Test>>::get(MEMBER_1);
        assert_eq!(value, 2u32);
    })
}

#[test]
fn it_increments_value_count_2_equal() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();

        let value = <OrgDataCount<Test>>::get(MEMBER_1);
        assert_eq!(value, 1u32);
    })
}

#[test]
fn it_errors_if_too_many_entries_for_single_member() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_2, VALUE).unwrap();
        let result = OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_3, VALUE);

        assert_err!(result, Error::<Test>::TooManyEntries);
    })
}

#[test]
fn it_succeeds_with_many_entries_across_multiple_members() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_2, VALUE).unwrap();
        let result = OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_2), KEY_3, VALUE);

        assert_ok!(result);
    })
}

#[test]
fn it_adds_new_zero_counts_on_change_with_new_members() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::change_members(&NEW_MEMBERS, &[], vec![]);

        let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
        values.sort_by_key(|(m, _)| *m);
        assert_eq!(
            values,
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0),
                (10, 0),
                (11, 0)
            ]
        );
    })
}

#[test]
fn it_removes_counts_on_change_with_outgoing_members() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_2, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_2), KEY_3, VALUE).unwrap();

        OrganisationData::change_members(&[], &[MEMBER_1, MEMBER_2], vec![]);

        let mut values = <OrgDataCount<Test>>::iter().collect::<Vec<_>>();
        values.sort_by_key(|(m, _)| *m);
        assert_eq!(
            values,
            vec![(0, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0),]
        );
    })
}

#[test]
fn it_removes_values_on_change_with_outgoing_members() {
    new_test_ext().execute_with(|| {
        OrganisationData::initialize_members(&INIT_MEMBERS);
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_1, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_1), KEY_2, VALUE).unwrap();
        OrganisationData::set_value(RuntimeOrigin::signed(MEMBER_2), KEY_3, VALUE).unwrap();

        OrganisationData::change_members(&[], &[MEMBER_1, MEMBER_2], vec![]);

        let value1 = <OrgData<Test>>::try_get(MEMBER_1, KEY_1);
        let value2 = <OrgData<Test>>::try_get(MEMBER_1, KEY_2);
        let value3 = <OrgData<Test>>::try_get(MEMBER_2, KEY_3);

        assert_err!(value1, ());
        assert_err!(value2, ());
        assert_err!(value3, ());
    })
}
