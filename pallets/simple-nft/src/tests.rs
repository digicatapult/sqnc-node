// Tests to be written here

use crate::{mock::*, Error, Token};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::iter::FromIterator;
use vitalam_pallet_traits::{ProcessFullyQualifiedId, ProcessIO};

const NONE_PROCESS: Option<ProcessFullyQualifiedId<ProcessIdentifier, u32>> = None;
const SUCCEED_PROCESS: Option<ProcessFullyQualifiedId<ProcessIdentifier, u32>> = Some(ProcessFullyQualifiedId {
    id: ProcessIdentifier::ShouldSucceed,
    version: 0u32,
});
const FAIL_PROCESS: Option<ProcessFullyQualifiedId<ProcessIdentifier, u32>> = Some(ProcessFullyQualifiedId {
    id: ProcessIdentifier::ShouldFail,
    version: 0u32,
});

#[test]
fn it_works_for_creating_token_with_file() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_token_with_literal() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = (vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::Literal([0]))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_token_with_token_id_in_metadata() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::TokenId(0))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_token_with_no_metadata_value() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_token_with_multiple_metadata_items() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![
            (0, MetadataValue::File(H256::zero())),
            (1, MetadataValue::Literal([0])),
            (2, MetadataValue::TokenId(0)),
            (3, MetadataValue::None),
        ]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_token_with_multiple_roles() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (Role::NotOwner, 2)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None
            }]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_many_token() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata0.clone(),
                    parent_index: None
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata2.clone(),
                    parent_index: None
                }
            ]
        ));
        // last token should be 3
        assert_eq!(SimpleNFTModule::last_token(), 3);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata0.clone(),
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 2,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata1.clone(),
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFTModule::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                original_id: 3,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata2.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_many_token_with_varied_metadata() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None), (1, MetadataValue::File(H256::zero()))]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::Literal([0]))]);
        let metadata2 = BTreeMap::from_iter(vec![(1, MetadataValue::Literal([0]))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata0.clone(),
                    parent_index: None
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata2.clone(),
                    parent_index: None
                }
            ]
        ));
        // last token should be 3
        assert_eq!(SimpleNFTModule::last_token(), 3);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata0.clone(),
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 2,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata1.clone(),
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFTModule::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                original_id: 3,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata2.clone(),
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_destroying_single_token() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![1],
            Vec::new()
        ));
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
    });
}

#[test]
fn it_works_for_destroying_many_tokens() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata0.clone(),
                    parent_index: None,
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None,
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata2.clone(),
                    parent_index: None,
                },
            ],
        )
        .unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![1, 2, 3],
            Vec::new()
        ));
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 3);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata0.clone(),
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 2,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata1.clone(),
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
        let token = SimpleNFTModule::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                original_id: 3,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata2.clone(),
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
    });
}

#[test]
fn it_works_for_creating_and_destroy_single_tokens() {
    new_test_ext().execute_with(|| {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles0.clone(),
                metadata: metadata0.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // create a token with a parent
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![1],
            vec![ProcessIO {
                roles: roles1.clone(),
                metadata: metadata1.clone(),
                parent_index: Some(0)
            }]
        ));
        // assert 1 more token was created
        assert_eq!(SimpleNFTModule::last_token(), 2);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles0.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata0.clone(),
                parents: Vec::new(),
                children: Some(vec![2])
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 1,
                roles: roles1.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata1.clone(),
                parents: vec![1],
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_and_destroy_many_tokens() {
    new_test_ext().execute_with(|| {
        let roles0 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles1 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata3 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![
                ProcessIO {
                    roles: roles0.clone(),
                    metadata: metadata0.clone(),
                    parent_index: None,
                },
                ProcessIO {
                    roles: roles0.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None,
                },
            ],
        )
        .unwrap();
        // create 2 tokens with 2 parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![1, 2],
            vec![
                ProcessIO {
                    roles: roles0.clone(),
                    metadata: metadata2.clone(),
                    parent_index: Some(0)
                },
                ProcessIO {
                    roles: roles1.clone(),
                    metadata: metadata3.clone(),
                    parent_index: Some(1)
                },
            ]
        ));
        // assert 2 more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 4);
        // get the old tokens
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles0.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata0.clone(),
                parents: Vec::new(),
                children: Some(vec![3, 4])
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 2,
                roles: roles0.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata1.clone(),
                parents: Vec::new(),
                children: Some(vec![3, 4])
            }
        );
        // get the new tokens
        let token = SimpleNFTModule::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                original_id: 1,
                roles: roles0.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata2.clone(),
                parents: vec![1, 2],
                children: None
            }
        );
        let token = SimpleNFTModule::tokens_by_id(4);
        assert_eq!(
            token,
            Token {
                id: 4,
                original_id: 2,
                roles: roles1.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata3.clone(),
                parents: vec![1, 2],
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_maintaining_original_id_through_multiple_children() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        // initial token
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // token with previous token as parent
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![1],
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: Some(0)
            },]
        ));
        // token with previous token as parent again
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            vec![2],
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: Some(0)
            },]
        ));
        // check all tokens have the same original_id
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata.clone(),
                parents: Vec::new(),
                children: Some(vec![2])
            }
        );
        let token = SimpleNFTModule::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: metadata.clone(),
                parents: vec![1],
                children: Some(vec![3])
            }
        );
        let token = SimpleNFTModule::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                original_id: 1,
                roles: roles.clone(),
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: metadata.clone(),
                parents: vec![2],
                children: None
            }
        );
    });
}

#[test]
fn it_fails_for_destroying_single_token_as_incorrect_role() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1), (Role::NotOwner, 2)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), NONE_PROCESS, vec![1], Vec::new()),
            Error::<Test>::NotOwned
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_destroying_single_token_as_other_signer() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), NONE_PROCESS, vec![1], Vec::new()),
            Error::<Test>::NotOwned
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_destroying_multiple_tokens_as_other_signer() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(2),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata0.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata1.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token_1 = SimpleNFTModule::tokens_by_id(1);
        let token_2 = SimpleNFTModule::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), NONE_PROCESS, vec![1, 2], Vec::new()),
            Error::<Test>::NotOwned
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 2);
        // assert old token hasn't changed
        assert_eq!(token_1, SimpleNFTModule::tokens_by_id(1));
        // assert old token hasn't changed
        assert_eq!(token_2, SimpleNFTModule::tokens_by_id(2));
    });
}

#[test]
fn it_fails_for_destroying_single_burnt_token() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata0.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), NONE_PROCESS, vec![1], Vec::new()).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                NONE_PROCESS,
                vec![1],
                vec![ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None
                }]
            ),
            Error::<Test>::AlreadyBurnt
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_destroying_multiple_tokens_with_burnt_token() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata0.clone(),
                    parent_index: None,
                },
                ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata1.clone(),
                    parent_index: None,
                },
            ],
        )
        .unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), NONE_PROCESS, vec![1], Vec::new()).unwrap();
        // get old token
        let token_1 = SimpleNFTModule::tokens_by_id(1);
        // get old token
        let token_2 = SimpleNFTModule::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                NONE_PROCESS,
                vec![1, 2],
                vec![ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata2.clone(),
                    parent_index: None,
                },],
            ),
            Error::<Test>::AlreadyBurnt
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 2);
        // assert old token hasn't changed
        assert_eq!(token_1, SimpleNFTModule::tokens_by_id(1));
        // assert old token hasn't changed
        assert_eq!(token_2, SimpleNFTModule::tokens_by_id(2));
    });
}

#[test]
fn it_fails_for_invalid_index_to_set_parent_from_inputs() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // try to use an out of bounds index to set parents from one of the inputs
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(2),
                NONE_PROCESS,
                vec![1],
                vec![ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata.clone(),
                    parent_index: Some(10),
                },],
            ),
            Error::<Test>::OutOfBoundsParent
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_setting_multiple_tokens_to_have_the_same_parent() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // try to set two tokens to have the same parent
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(2),
                NONE_PROCESS,
                vec![1],
                vec![
                    ProcessIO {
                        roles: roles.clone(),
                        metadata: metadata.clone(),
                        parent_index: Some(0),
                    },
                    ProcessIO {
                        roles: roles.clone(),
                        metadata: metadata.clone(),
                        parent_index: Some(0),
                    },
                ],
            ),
            Error::<Test>::DuplicateParents
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_creating_single_token_with_too_many_metadata_items() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata_too_many = BTreeMap::from_iter(vec![
            (0, MetadataValue::None),
            (1, MetadataValue::None),
            (2, MetadataValue::None),
            (3, MetadataValue::None),
            (4, MetadataValue::None),
        ]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata0.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to create token with too many metadata items
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                NONE_PROCESS,
                Vec::new(),
                vec![ProcessIO {
                    roles: roles.clone(),
                    metadata: metadata_too_many.clone(),
                    parent_index: None,
                },],
            ),
            Error::<Test>::TooManyMetadataItems
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_creating_single_token_with_no_default_role() {
    new_test_ext().execute_with(|| {
        let roles = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let roles_empty = BTreeMap::new();
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            NONE_PROCESS,
            Vec::new(),
            vec![ProcessIO {
                roles: roles.clone(),
                metadata: metadata.clone(),
                parent_index: None,
            }],
        )
        .unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to create token without setting default role in roles
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                NONE_PROCESS,
                Vec::new(),
                vec![ProcessIO {
                    roles: roles_empty.clone(),
                    metadata: metadata.clone(),
                    parent_index: None,
                },],
            ),
            Error::<Test>::NoDefaultRole
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}

#[test]
fn it_works_for_running_success_process() {
    new_test_ext().execute_with(|| {
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            SUCCEED_PROCESS,
            Vec::new(),
            Vec::new()
        ));
    });
}

#[test]
fn it_fails_for_running_success_process() {
    new_test_ext().execute_with(|| {
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), FAIL_PROCESS, Vec::new(), Vec::new()),
            Error::<Test>::ProcessInvalid
        );
    });
}
