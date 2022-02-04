use super::*;
use crate::{GetProcess, GetVersion, Process, ProcessStatus, Restriction::None};
use frame_support::assert_ok;

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
    #[allow(dead_code)]
    const PROCESS_ID: [u8; 32] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    fn mock_process() -> Process {
        return Process {
            status: ProcessStatus::Enabled,
            restrictions: [None].to_vec(),
        };
    }

    #[allow(dead_code)]
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
