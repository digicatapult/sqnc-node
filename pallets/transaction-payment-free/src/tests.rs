use super::*;
use frame_support::weights::Weight;
use mock::{info_from_weight, new_test_ext, Balances, Test, CALL};
use sp_runtime::traits::TxBaseImplication;

#[test]
fn no_fee_is_charged_for_transaction() {
    new_test_ext().execute_with(|| {
        let implication = TxBaseImplication(CALL.clone());
        let user = 1;
        let initial_balance = Balances::free_balance(user);

        let ext = ChargeTransactionPayment::<Test>::from(0);
        let result = ext.validate(
            frame_system::RawOrigin::Signed(user).into(),
            &CALL,
            &info_from_weight(Weight::zero()),
            0,
            (),
            &implication,
            TransactionSource::External,
        );

        assert!(result.is_ok());
        assert_eq!(Balances::free_balance(user), initial_balance);
    });
}
