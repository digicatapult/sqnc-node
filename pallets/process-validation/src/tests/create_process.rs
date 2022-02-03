
use super::*;
use frame_support::{assert_ok};
use crate::{Restriction::None, ProcessStatus, Process};

// fn if process does not exists - should return a version number of 1
// validate origin 
// get latest version
// deposit event
// fn calls validates payload

// -- fixtures --
const PROCESS_ID: [u8; 32] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

#[test]
fn creates_a_new_process() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID,
            vec![{ None }]
        ));
        let process = ProcessValidation::processes_by_id_and_version(PROCESS_ID, 1);
        assert_eq!(
            process,
            Process {
                status: ProcessStatus::Disabled,
                restrictions: [None].to_vec(),
                version: 0,
            }
        )
    });
}

#[test]
fn updates_a_version_and_creates_a_process() {
    new_test_ext().execute_with(|| {
        // Create first process
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID,
            vec![{ None }],
        ));

        // creates a second one for the same id
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID,
            vec![{ None }],
        ));

        // retrieve a process
        let process = ProcessValidation::processes_by_id_and_version(PROCESS_ID, 1);
        println!("process: {:?}", process);
        println!("version: {:?}", ProcessValidation::latest_process_version(PROCESS_ID));

        assert_eq!(
            process,
            Process {
                status: ProcessStatus::Disabled,
                restrictions: [None].to_vec(),
                version: 1,
            }
        )
    });
}


#[test]
fn create_process_simple() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID,
            vec![{ None }],
        ));
    });
}

