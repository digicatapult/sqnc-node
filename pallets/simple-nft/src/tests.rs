// Tests to be written here

use crate::{mock::*, Error, Token};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::iter::FromIterator;
use sp_core::H256;
use frame_support::{assert_err, assert_ok};
// use frame_support::traits::OnRuntimeUpgrade;

#[test]
fn it_works_for_creating_simple_token() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        let metadata = BTreeMap::from_iter(vec![(0, H256::zero())]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
        let metadata = BTreeMap::from_iter(vec![(0, H256::zero()), (1, H256::zero())]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, metadata.clone())]
        ));
        // last token should be 1
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata2 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, metadata0.clone()), (1, metadata1.clone()), (1, metadata2.clone())]
        ));
        // last token should be 3
        assert_eq!(SimpleNFTModule::last_token(), 3);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
                owner: 1,
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
                owner: 1,
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero()), (1, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata2 = BTreeMap::from_iter(vec![(1, H256::zero())]);
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, metadata0.clone()), (1, metadata1.clone()), (1, metadata2.clone())]
        ));
        // last token should be 3
        assert_eq!(SimpleNFTModule::last_token(), 3);
        // get the token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
                owner: 1,
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
                owner: 1,
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
        let metadata = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata.clone())]).unwrap();
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
                owner: 1,
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata2 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(
            Origin::signed(1), 
            Vec::new(), 
            vec![(1, metadata0.clone()), (1, metadata1.clone()), (1, metadata2.clone())]
        ).unwrap();
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
                owner: 1,
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
                owner: 1,
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
                owner: 1,
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata0.clone())]).unwrap();
        // create a token with a parent
        assert_ok!(SimpleNFTModule::run_process(Origin::signed(1), vec![1], vec![(2, metadata1.clone())]));
        // assert 1 more token was created
        assert_eq!(SimpleNFTModule::last_token(), 2);
        // get the old token
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
                owner: 2,
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata2 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata3 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata0.clone()), (1, metadata1.clone())]).unwrap();
        // create a token with 2 parents
        assert_ok!(SimpleNFTModule::run_process(
            Origin::signed(1),
            vec![1, 2],
            vec![(1, metadata2.clone()), (2, metadata3.clone())]
        ));
        // assert 2 more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 4);
        // get the old tokens
        let token = SimpleNFTModule::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
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
                owner: 1,
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
                owner: 1,
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
                owner: 2,
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
fn it_fails_for_destroying_single_token_as_other_signer() {
    new_test_ext().execute_with(|| {
        let metadata = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata.clone())]).unwrap();
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(2), Vec::new(), vec![(1, metadata0.clone())]).unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata1.clone())]).unwrap();
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata0.clone())]).unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), vec![1], vec![(1, metadata1.clone())]),
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata1 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata2 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata0.clone()), (1, metadata1.clone())]).unwrap();
        SimpleNFTModule::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token_1 = SimpleNFTModule::tokens_by_id(1);
        // get old token
        let token_2 = SimpleNFTModule::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), vec![1, 2], vec![(1, metadata2.clone())]),
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
        let metadata0 = BTreeMap::from_iter(vec![(0, H256::zero())]);
        let metadata_too_many = BTreeMap::from_iter(vec![(0, H256::zero()), (1, H256::zero()), (2, H256::zero())]);
        SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata0.clone())]).unwrap();
        // get old token
        let token = SimpleNFTModule::tokens_by_id(1);
        // Try to create token with too many metadata items
        assert_err!(
            SimpleNFTModule::run_process(Origin::signed(1), Vec::new(), vec![(1, metadata_too_many.clone())]),
            Error::<Test>::TooManyMetadataItems
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFTModule::last_token(), 1);
        // assert old token hasn't changed
        assert_eq!(token, SimpleNFTModule::tokens_by_id(1));
    });
}