use super::*;
use frame_support::weights::Pays;
use mock::{info_from_weight, new_test_ext, Balances, System, CALL};

#[test]
fn signed_extension_transaction_payment_work() {
    new_test_ext().execute_with(|| {
        let len = 10;
        let pre = ChargeTransactionPayment::from(0)
            .pre_dispatch(&2, CALL, &info_from_weight(100), len)
            .unwrap();
        assert_eq!(Balances::free_balance(2), 200 - 5 - 10 - 100 - 5);
    });
}

#[test]
fn zero_transfer_on_free_transaction() {
    new_test_ext().execute_with(|| {
        // So events are emitted
        System::set_block_number(10);
        let len = 10;
        let dispatch_info = DispatchInfo {
            weight: 100,
            pays_fee: Pays::No,
            class: DispatchClass::Normal
        };
        let user = 69;
        let pre = ChargeTransactionPayment::from(0)
            .pre_dispatch(&user, CALL, &dispatch_info, len)
            .unwrap();
        assert_eq!(Balances::free_balance(&user), 0);
        assert_eq!(Balances::free_balance(&user), 0);
        // No events for such a scenario
        assert_eq!(System::events().len(), 0);
    });
}
