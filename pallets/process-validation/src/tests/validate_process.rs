use super::*;

use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator};
use frame_support::bounded_vec;
use sp_std::collections::btree_map::BTreeMap;

use crate::restrictions::Restriction;
use crate::{Process, ProcessModel, ProcessStatus};

#[test]
fn it_succeeds_when_process_exists() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![]
            }
        );

        assert!(ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::A,
                version: 1u32
            },
            &0u64,
            &Vec::new(),
            &Vec::new(),
        ));
    });
}

#[test]
fn it_fails_when_process_id_doesnt_exist() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![]
            }
        );

        assert!(!ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::B,
                version: 1u32
            },
            &0u64,
            &bounded_vec![],
            &bounded_vec![],
        ));
    });
}

#[test]
fn it_fails_when_process_version_doesnt_exist() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![]
            }
        );

        assert!(!ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::A,
                version: 2u32
            },
            &0u64,
            &bounded_vec![],
            &bounded_vec![],
        ));
    });
}

#[test]
fn it_fails_when_process_disabled() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Disabled,
                restrictions: bounded_vec![]
            }
        );

        assert!(!ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::A,
                version: 1u32
            },
            &0u64,
            &bounded_vec![],
            &bounded_vec![],
        ));
    });
}

#[test]
fn it_succeeds_when_all_restrictions_succeed() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![Restriction::None, Restriction::SenderOwnsAllInputs]
            }
        );

        let mut token_roles: BTreeMap<u32, u64> = BTreeMap::new();
        token_roles.insert(Default::default(), 0u64);

        assert!(ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::A,
                version: 1u32
            },
            &0u64,
            &vec![ProcessIO {
                roles: token_roles,
                metadata: BTreeMap::new(),
                parent_index: None
            }],
            &bounded_vec![],
        ));
    });
}

#[test]
fn it_fails_when_one_restrictions_fails() {
    new_test_ext().execute_with(|| {
        ProcessModel::<Test>::insert(
            ProcessIdentifier::A,
            1u32,
            Process {
                status: ProcessStatus::Enabled,
                restrictions: bounded_vec![Restriction::None, Restriction::SenderOwnsAllInputs]
            }
        );

        let mut token_roles: BTreeMap<u32, u64> = BTreeMap::new();
        token_roles.insert(Default::default(), 1u64);

        assert!(!ProcessValidation::validate_process(
            ProcessFullyQualifiedId {
                id: ProcessIdentifier::A,
                version: 1u32
            },
            &0u64,
            &vec![ProcessIO {
                roles: token_roles,
                metadata: BTreeMap::new(),
                parent_index: None
            }],
            &bounded_vec![],
        ));
    });
}
