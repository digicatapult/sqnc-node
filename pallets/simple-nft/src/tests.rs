// Tests to be written here

use crate::{mock::*, Error, Token};
use frame_support::{assert_err, assert_ok};
use frame_support::traits::OnRuntimeUpgrade;

#[test]
fn it_works_for_creating_simple_token() {
    new_test_ext().execute_with(|| {
        // create a token with no parents
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, 42)]
        ));
        // last token should be 1
        assert_eq!(SimpleNFT::last_token(), 1);
        // get the token
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 42,
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
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, 42), (1, 43), (1, 44)]
        ));
        // last token should be 3
        assert_eq!(SimpleNFT::last_token(), 3);
        // get the token
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 42,
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFT::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 43,
                parents: Vec::new(),
                children: None
            }
        );
        let token = SimpleNFT::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 44,
                parents: Vec::new(),
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_destroying_single_token() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42)]).unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            vec![1],
            Vec::new()
        ));
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 1);
        // get the old token
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 42,
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
    });
}

#[test]
fn it_works_for_destroying_many_tokens() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, 42), (1, 43), (1, 44)],
        )
        .unwrap();
        // create a token with no parents
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            vec![1, 2, 3],
            Vec::new()
        ));
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 3);
        // get the old token
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 42,
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
        let token = SimpleNFT::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 43,
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
        let token = SimpleNFT::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 44,
                parents: Vec::new(),
                children: Some(Vec::new())
            }
        );
    });
}

#[test]
fn it_works_for_creating_and_destroy_single_tokens() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42)]).unwrap();
        // create a token with a parent
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            vec![1],
            vec![(2, 43)]
        ));
        // assert 1 more token was created
        assert_eq!(SimpleNFT::last_token(), 2);
        // get the old token
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 42,
                parents: Vec::new(),
                children: Some(vec![2])
            }
        );
        let token = SimpleNFT::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                owner: 2,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 43,
                parents: vec![1],
                children: None
            }
        );
    });
}

#[test]
fn it_works_for_creating_and_destroy_many_tokens() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42), (1, 43)]).unwrap();
        // create a token with 2 parents
        assert_ok!(SimpleNFT::run_process(
            Origin::signed(1),
            vec![1, 2],
            vec![(1, 44), (2, 45)]
        ));
        // assert 2 more tokens were created
        assert_eq!(SimpleNFT::last_token(), 4);
        // get the old tokens
        let token = SimpleNFT::tokens_by_id(1);
        assert_eq!(
            token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 42,
                parents: Vec::new(),
                children: Some(vec![3, 4])
            }
        );
        let token = SimpleNFT::tokens_by_id(2);
        assert_eq!(
            token,
            Token {
                id: 2,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: Some(0),
                metadata: 43,
                parents: Vec::new(),
                children: Some(vec![3, 4])
            }
        );
        // get the new tokens
        let token = SimpleNFT::tokens_by_id(3);
        assert_eq!(
            token,
            Token {
                id: 3,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 44,
                parents: vec![1, 2],
                children: None
            }
        );
        let token = SimpleNFT::tokens_by_id(4);
        assert_eq!(
            token,
            Token {
                id: 4,
                owner: 2,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 45,
                parents: vec![1, 2],
                children: None
            }
        );
    });
}

#[test]
fn it_fails_for_destroying_single_token_as_other_signer() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42)]).unwrap();
        // get old token
        let token = SimpleNFT::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFT::run_process(Origin::signed(2), vec![1], Vec::new()),
            Error::<Test>::NotOwned
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 1);
        // asset old token hasn't changed
        assert_eq!(token, SimpleNFT::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_destroying_multiple_tokens_as_other_signer() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(2), Vec::new(), vec![(1, 42)]).unwrap();
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 43)]).unwrap();
        // get old token
        let token_1 = SimpleNFT::tokens_by_id(1);
        let token_2 = SimpleNFT::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFT::run_process(Origin::signed(2), vec![1, 2], Vec::new()),
            Error::<Test>::NotOwned
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 2);
        // asset old token hasn't changed
        assert_eq!(token_1, SimpleNFT::tokens_by_id(1));
        // asset old token hasn't changed
        assert_eq!(token_2, SimpleNFT::tokens_by_id(2));
    });
}

#[test]
fn it_fails_for_destroying_single_burnt_token() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42)]).unwrap();
        SimpleNFT::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token = SimpleNFT::tokens_by_id(1);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFT::run_process(Origin::signed(1), vec![1], vec![(1, 43)]),
            Error::<Test>::AlreadyBurnt
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 1);
        // asset old token hasn't changed
        assert_eq!(token, SimpleNFT::tokens_by_id(1));
    });
}

#[test]
fn it_fails_for_destroying_multiple_tokens_with_burnt_token() {
    new_test_ext().execute_with(|| {
        SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42), (1, 43)]).unwrap();
        SimpleNFT::run_process(Origin::signed(1), vec![1], Vec::new()).unwrap();
        // get old token
        let token_1 = SimpleNFT::tokens_by_id(1);
        // get old token
        let token_2 = SimpleNFT::tokens_by_id(2);
        // Try to destroy token as incorrect user
        assert_err!(
            SimpleNFT::run_process(Origin::signed(1), vec![1, 2], vec![(1, 44)]),
            Error::<Test>::AlreadyBurnt
        );
        // assert no more tokens were created
        assert_eq!(SimpleNFT::last_token(), 2);
        // asset old token hasn't changed
        assert_eq!(token_1, SimpleNFT::tokens_by_id(1));
        // asset old token hasn't changed
        assert_eq!(token_2, SimpleNFT::tokens_by_id(2));
    });
}

#[test]
fn it_upgrades_from_main_successfully() {
    new_test_ext().execute_with(|| {

        // Create one token
        SimpleNFT::run_process(
            Origin::signed(1),
            Vec::new(),
            vec![(1, 42)]
        );

        let old_token = SimpleNFT::tokens_by_id(1);

        // SimpleNFT::run_process(Origin::signed(1), Vec::new(), vec![(1, 42), (1, 43)]).unwrap();

        // Perform upgrade
		// assert_eq!(
        //     SimpleNFT::on_runtime_upgrade(),
        //     <Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 2),
        // );

        assert_eq!(
            old_token,
            Token {
                id: 1,
                owner: 1,
                creator: 1,
                created_at: 0,
                destroyed_at: None,
                metadata: 42,
                parents: Vec::new(),
                children: None
            }
        );
    });
}
