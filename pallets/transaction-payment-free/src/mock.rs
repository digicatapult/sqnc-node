use super::*;
use crate as pallet_transaction_payment_free;
use frame_support::dispatch::DispatchInfo;
use frame_support::{derive_impl, parameter_types, weights::Weight};
use frame_system as system;
use pallet_balances::Call as BalancesCall;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPaymentFree: pallet_transaction_payment_free::{Pallet},
    }
);

pub const CALL: &<Test as frame_system::Config>::RuntimeCall =
    &RuntimeCall::Balances(BalancesCall::transfer_allow_death { dest: 2, value: 69 });

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<u64>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl Config for Test {
    type OnFreeTransaction = CurrencyAdapter<Balances, ()>;
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
    DispatchInfo {
        call_weight: w,
        ..Default::default()
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10)],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
