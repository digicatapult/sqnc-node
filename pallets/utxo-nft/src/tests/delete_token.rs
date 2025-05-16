use crate::{graveyard::GraveyardState, input::Input, output::Output, tests::mock::*, Error, Event};
use frame_support::{assert_err, assert_ok, traits::Hooks, weights::Weight};
use sp_core::H256;
use sp_runtime::{bounded_btree_map, bounded_vec};
use sqnc_pallet_traits::ProcessFullyQualifiedId;

const SUCCEED_PROCESS: ProcessFullyQualifiedId<ProcessIdentifier, u32> = ProcessFullyQualifiedId {
    id: ProcessIdentifier::ShouldSucceed,
    version: 0u32,
};

fn create_and_burn_token() {
    let roles = bounded_btree_map!(Default::default() => 1);
    let metadata = bounded_btree_map!(0 => MetadataValue::File(H256::zero()));
    UtxoNFT::run_process(
        RuntimeOrigin::signed(1),
        SUCCEED_PROCESS,
        bounded_vec![],
        bounded_vec![Output {
            roles: roles.clone(),
            metadata: metadata.clone()
        }],
    )
    .unwrap();
    UtxoNFT::run_process(
        RuntimeOrigin::signed(1),
        SUCCEED_PROCESS,
        bounded_vec![Input::Token(UtxoNFT::last_token())],
        bounded_vec![],
    )
    .unwrap();
}

#[test]
fn it_succeeds_if_token_already_deleted() {
    new_test_ext().execute_with(|| {
        assert_ok!(UtxoNFT::delete_token(RuntimeOrigin::signed(1), 1));
        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token, None);
    });
}

#[test]
fn it_errors_unburnt_for_unspent_token() {
    new_test_ext().execute_with(|| {
        let roles = bounded_btree_map!(Default::default() => 1);
        let metadata = bounded_btree_map!(0 => MetadataValue::File(H256::zero()));
        assert_ok!(UtxoNFT::run_process(
            RuntimeOrigin::signed(1),
            SUCCEED_PROCESS,
            bounded_vec![],
            bounded_vec![Output {
                roles: roles.clone(),
                metadata: metadata.clone()
            }]
        ));
        // last token should be 1
        assert_eq!(UtxoNFT::last_token(), 1);

        assert_err!(
            UtxoNFT::delete_token(RuntimeOrigin::signed(1), 1),
            Error::<Test>::NotBurnt
        );

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), true);
    });
}

#[test]
fn it_errors_burnt_too_recently_for_newly_burnt_token() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();

        assert_err!(
            UtxoNFT::delete_token(RuntimeOrigin::signed(1), 1),
            Error::<Test>::BurntTooRecently
        );

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), true);
    });
}

#[test]
fn it_allows_delete_token_after_tombstone_period() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();

        run_to_block(100u64, false);

        assert_ok!(UtxoNFT::delete_token(RuntimeOrigin::signed(1), 1));

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), false);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            RuntimeEvent::UtxoNFT(Event::TokenDeleted { token_id: 1 }),
        );
    });
}

#[test]
fn it_does_not_delete_token_in_on_idle_before_tombstone_period() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();

        run_to_block(99u64, true);

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), true);
        assert_eq!(
            UtxoNFT::current_graveyard_state(),
            GraveyardState {
                start_index: 0,
                end_index: 1
            }
        );
        assert_eq!(UtxoNFT::graveyard(0), Some(1));
    });
}

#[test]
fn it_deletes_token_in_on_idle_after_tombstone_period() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();

        run_to_block(100u64, true);

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), false);
        assert_eq!(
            UtxoNFT::current_graveyard_state(),
            GraveyardState {
                start_index: 1,
                end_index: 1
            }
        );
        assert_eq!(UtxoNFT::graveyard(0), None);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            RuntimeEvent::UtxoNFT(Event::TokenDeleted { token_id: 1 }),
        );
    });
}

#[test]
fn it_deletes_multiple_tokens_in_on_idle_after_tombstone_period() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();
        create_and_burn_token();

        run_to_block(100u64, true);

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), false);
        let token = UtxoNFT::tokens_by_id(2);
        assert_eq!(token.is_some(), false);
        assert_eq!(
            UtxoNFT::current_graveyard_state(),
            GraveyardState {
                start_index: 2,
                end_index: 2
            }
        );
        assert_eq!(UtxoNFT::graveyard(0), None);
        assert_eq!(UtxoNFT::graveyard(1), None);
        assert_eq!(
            System::events().iter().last().unwrap().event,
            RuntimeEvent::UtxoNFT(Event::TokenDeleted { token_id: 2 }),
        );
    });
}

#[test]
fn it_does_not_delete_tokens_in_on_idle_not_enough_weight() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();

        run_to_block(100u64, false);

        UtxoNFT::on_idle(System::block_number(), Weight::from_parts(0, 0));

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), true);
        assert_eq!(
            UtxoNFT::current_graveyard_state(),
            GraveyardState {
                start_index: 0,
                end_index: 1
            }
        );
        assert_eq!(UtxoNFT::graveyard(0), Some(1));
    });
}

#[test]
fn it_deletes_limited_tokens_in_on_idle_base_on_available_weight() {
    new_test_ext().execute_with(|| {
        create_and_burn_token();
        create_and_burn_token();

        run_to_block(100u64, false);

        UtxoNFT::on_idle(System::block_number(), Weight::from_parts(1, 1));

        let token = UtxoNFT::tokens_by_id(1);
        assert_eq!(token.is_some(), false);
        let token = UtxoNFT::tokens_by_id(2);
        assert_eq!(token.is_some(), true);
        assert_eq!(
            UtxoNFT::current_graveyard_state(),
            GraveyardState {
                start_index: 1,
                end_index: 2
            }
        );
        assert_eq!(UtxoNFT::graveyard(0), None);
        assert_eq!(UtxoNFT::graveyard(1), Some(2));
        assert_eq!(
            System::events().iter().last().unwrap().event,
            RuntimeEvent::UtxoNFT(Event::TokenDeleted { token_id: 1 }),
        );
    });
}
