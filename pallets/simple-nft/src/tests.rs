// Tests to be written here

use crate::{mock::*, Error, Token};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::iter::FromIterator;
// use frame_support::traits::OnRuntimeUpgrade;

#[test]
fn it_works_for_creating_token_with_file() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner.clone(), metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::Literal([0]))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner.clone(), metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner.clone(), metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![
            (0, MetadataValue::File(H256::zero())),
            (1, MetadataValue::Literal([0])),
            (2, MetadataValue::None),
        ]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner.clone(), metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::File(H256::zero()))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![
                (owner.clone(), metadata0.clone()),
                (owner.clone(), metadata1.clone()),
                (owner.clone(), metadata2.clone())
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None), (1, MetadataValue::File(H256::zero()))]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::Literal([0]))]);
        let metadata2 = BTreeMap::from_iter(vec![(1, MetadataValue::Literal([0]))]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![
                (owner.clone(), metadata0.clone()),
                (owner.clone(), metadata1.clone()),
                (owner.clone(), metadata2.clone())
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata.clone())]).unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFTModule::run_process(Origin::signed(1), vec![1], Vec::new()));
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner.clone(),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![
                (owner.clone(), metadata0.clone()),
                (owner.clone(), metadata1.clone()),
                (owner.clone(), metadata2.clone()),
            ],
        )
        .unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
                owner: owner.clone(),
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
        let owner1 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let owner2 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner1.clone(), metadata0.clone())]).unwrap();
        // create a token with a parent
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            vec![1],
            vec![(owner2.clone(), metadata1.clone())]
        ));
        // assert 1 more token was created
        assert_eq!(SimpleNFTModule::last_token(), 2);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner1.clone(),
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
                owner: owner2.clone(),
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
        let owner1 = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let owner2 = BTreeMap::from_iter(vec![(Default::default(), 2)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata3 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner1.clone(), metadata0.clone()), (owner1.clone(), metadata1.clone())],
        )
        .unwrap();
        // create a token with 2 parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            vec![1, 2],
            vec![(owner1.clone(), metadata2.clone()), (owner2.clone(), metadata3.clone())]
        ));
        // assert 2 more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 4);
        // get the old tokens
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: owner1.clone(),
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
                owner: owner1.clone(),
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
                owner: owner1.clone(),
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
                owner: owner2.clone(),
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
fn it_fails_for_destroying_single_token_as_incorrect_role() {
    new_test_ext().execute_with(|| {
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1), (Role::NotAdmin, 2)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata.clone())]).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), vec![1], Vec::new()),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata.clone())]).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), vec![1], Vec::new()),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(2), Vec::new(), vec![(owner.clone(), metadata0.clone())]).unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata1.clone())]).unwrap();
        // get old token
        let token_1 = SimpleNFTModule::tokens_by_id(1);
        let token_2 = SimpleNFTModule::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(2), vec![1, 2], Vec::new()),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata0.clone())]).unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), vec![1], vec![(owner.clone(), metadata1.clone())]),
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata1 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata2 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(owner.clone(), metadata0.clone()), (owner.clone(), metadata1.clone())],
        )
        .unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token_1 = SimpleNFTModule::tokens_by_id(1);
        // get old token
        let token_2 = SimpleNFTModule::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), vec![1, 2], vec![(owner.clone(), metadata2.clone())]),
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
fn it_fails_for_creating_single_token_with_too_many_metadata_items() {
    new_test_ext().execute_with(|| {
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let metadata0 = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        let metadata_too_many = BTreeMap::from_iter(vec![
            (0, MetadataValue::None),
            (1, MetadataValue::None),
            (2, MetadataValue::None),
            (3, MetadataValue::None),
        ]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata0.clone())]).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to create token with too many metadata items
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                Vec::new(),
                vec![(owner.clone(), metadata_too_many.clone())]
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
        let owner = BTreeMap::from_iter(vec![(Default::default(), 1)]);
        let owner_empty = BTreeMap::new();
        let metadata = BTreeMap::from_iter(vec![(0, MetadataValue::None)]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(owner.clone(), metadata.clone())]).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to create token without setting default role in owner
        assert_err!(
            SimpleNFTModule::run_process(
                Origin::signed(1),
                Vec::new(),
                vec![(owner_empty.clone(), metadata.clone())]
            ),
            Error::<Test>::NoDefaultRole
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}
