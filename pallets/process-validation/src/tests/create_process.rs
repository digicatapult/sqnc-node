use super::*;
use crate::{Process, Version, ProcessStatus, Restriction::None, ProcessModel, VersionModel};
use crate::Event::*;
use frame_support::{assert_ok};
use sp_std::prelude::*;

// -- fixtures --
#[allow(dead_code)]
const PROCESS_ID1: [u8; 32] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
const PROCESS_ID2: [u8; 32] = [
    1, 2, 3, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[test]
fn returns_error_if_origin_validation_fails_and_no_data_added() {
    new_test_ext().execute_with(|| {
        // update to check for error type
        assert!(ProcessValidation::create_process(
            Origin::none(),
            PROCESS_ID1,
            vec![{ None }]
        ).is_err());

        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0);
        assert_eq!(<ProcessModel<Test>>::get(PROCESS_ID1, 1), Process {
            status: ProcessStatus::Disabled, 
            restrictions: [].to_vec(),
        });
    });
}

#[test]
fn if_no_version_found_it_should_return_default_and_insert_new_one() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 0);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            vec![{ None }],
        ));
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 1);
        
    });
}

#[test]
fn for_existing_process_it_mutates_an_existing_version() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ProcessValidation::_update_version(PROCESS_ID1));
        assert_ok!(ProcessValidation::_update_version(PROCESS_ID1));
        assert_ok!(ProcessValidation::_update_version(PROCESS_ID1));

        let items: Vec<i32> = <VersionModel<Test>>::iter()
            .map(|item: ([u8; 32], i32)| item.1.clone())
            .collect();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0], 3);
        assert_eq!(<VersionModel<Test>>::get(PROCESS_ID1), 3);
    });
}

#[test]
fn updates_correctly_versions_for_multiple_processes() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let mut ids: Vec<[u8; 32]> = [PROCESS_ID2; 10].to_vec();
        ids.extend([PROCESS_ID1; 15].to_vec());
        ids.iter().for_each(|id: &[u8; 32]| -> () {
            assert_ok!(ProcessValidation::_update_version(*id));
        });

        let id1_expected = Event::pallet_process_validation(ProcessCreated(PROCESS_ID1, 16, vec![{ None }], false));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            vec![{ None }],
        ));
        let id2_expected = Event::pallet_process_validation(ProcessCreated(PROCESS_ID2, 11, vec![{ None }], false));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID2,
            vec![{ None }],
        ));

        assert_eq!(System::events()[0].event, id1_expected);
        assert_eq!(System::events()[1].event, id2_expected);
    });
}

#[test]
fn updates_version_correctly_for_existing_process() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        <VersionModel<Test>>::insert(
            PROCESS_ID1,
            Version {
                version: 10,
            }
        );
        let expected = Event::pallet_process_validation(ProcessCreated(PROCESS_ID1, 11, vec![{ None }], false));
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            vec![{ None }],
        ));
        assert_eq!(System::events()[0].event, expected);
    });
}

#[test]
fn updates_version_correctly_for_new_process() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID1,
            vec![{ None }],
        ));
        let expected = Event::pallet_process_validation(ProcessCreated(
            PROCESS_ID1,
            1,
            vec![{ None }],
            true
        ));
        // sets version to 1 and returns true to identify that this is a new event
        assert_eq!(System::events()[0].event, expected);
    });
}