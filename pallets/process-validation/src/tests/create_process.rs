use super::*;
use frame_support::{assert_ok};
use crate::{Restriction::None, ProcessStatus, Process, GetProcess, GetVersion};

/*
    // best todo of all is to finish a todo list
    - some major changes are required to the test.rs
        [ ] - ability to listen for evens by using System:events()
        [ ] - ability to seeds some data
        [ ] - pallet methods should be able to store data to mock storage and query as well
        [ ] - better block management
        [ ] - include fixtures, ideally to load dynamicaly into <Test> instance
        -- some EXAMPLES -> 
            - https://github.com/paritytech/substrate/blob/master/frame/contracts/src/tests.rs
            - https://github.com/paritytech/substrate/blob/master/frame/contracts/src/tests.rs
*/


#[test]
fn process_validation() {
    // -- fixtures --
    const PROCESS_ID: [u8; 32] = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];
    fn mock_process() -> Process {
        return Process {
            status: ProcessStatus::Enabled,
            restrictions: [None].to_vec(),
        };
    }

    fn updates_version_for_existing() {
        new_test_ext().execute_with(|| {
            assert_eq!(1, 1);
        });
    }

    #[allow(dead_code)]
    fn creates_new() {
        new_test_ext().execute_with(|| {
            assert_ok!(ProcessValidation::create_process(
                Origin::root(),
                PROCESS_ID,
                vec![{ None }]
            ));
        });
    }
}
/*
fn creates_a_new_process() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID0.clone(),
            vec![{ None }]
        ));
        let process: Process = ProcessValidation::processes_by_id_and_version(PROCESS_ID0.clone(), 1);
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
        let _a = <LatestProcessVersion<Test>>::get(PROCESS_ID0);
        println!("events:: {:?}", System::events());
        println!("id4554: {}", _a);
    });

    new_test_ext().execute_with(|| {


        // create a process
        <ProcessesByIdAndVersion<Test>>::insert(
            PROCESS_ID0,
            1,
            mock_process,
        );
        ProcessValidation::

        ProcessValidation::::insert(
            PROCESS_ID0,
            1,
            mock_process,
        );

        let _a = <LatestProcessVersion<Test>>::get(PROCESS_ID0);
        println!("id: {}", _a);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID0.clone(),
            vec![{ None }],
        ));
        println!("events:: 1{:?}", System::events());

        assert_eq!(<LatestProcessVersion<Test>>::get(PROCESS_ID0), 0);
        //assert_eq!(ProcessValidation::latest_process_version(PROCESS_ID0.clone()), 10);
        println!("events:: 2{:?}", System::events())
    });
}


#[test]
fn create_process_simple() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID0,
            vec![{ None }],
        ));
            
        println!("events:: 1{:?}", System::events());

        let _a = <LatestProcessVersion<Test>>::get(PROCESS_ID0);
        println!("id: {}", _a);
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID0.clone(),
            vec![{ None }],
        ));

        assert_eq!(<LatestProcessVersion<Test>>::get(PROCESS_ID0), 0);
        //assert_eq!(ProcessValidation::latest_process_version(PROCESS_ID0.clone()), 10);
        println!("events:: 2{:?}", System::events());
    });
}


#[test]
fn create_process_simple() {
    new_test_ext().execute_with(|| {
        assert_ok!(ProcessValidation::create_process(
            Origin::root(),
            PROCESS_ID0,
            vec![{ None }],
        ));
    });
}
*/
