use super::*;
use frame_support::{assert_err, assert_ok};
use mock::{info_from_weight, new_test_ext, Balances, System, Test, CALL};
use sp_runtime::transaction_validity::InvalidTransaction;

#[test]
fn transaction_payment_works_with_no_fee_for_account_with_balance() {
    new_test_ext().execute_with(|| {
        let user = 1;
        assert_eq!(Balances::free_balance(user), 10);
        assert_ok!(ChargeTransactionPayment::<Test>::from(0).pre_dispatch(&user, CALL, &info_from_weight(0), 0));
        assert_eq!(Balances::free_balance(user), 10);
    });
}

#[test]
fn transaction_payment_fails_for_account_with_no_balance() {
    new_test_ext().execute_with(|| {
        // So events are emitted
        System::set_block_number(10);

        let user = 2;
        assert_eq!(Balances::free_balance(user), 0);
        assert_err!(
            ChargeTransactionPayment::<Test>::from(0).pre_dispatch(&user, CALL, &info_from_weight(0), 0),
            InvalidTransaction::Payment
        );
        // No events for such a scenario
        assert_eq!(System::events().len(), 0);
    });
}
