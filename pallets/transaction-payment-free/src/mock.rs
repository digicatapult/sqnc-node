use super::*;
use crate as pallet_transaction_payment_free;
use frame_support::{
    parameter_types,
    traits::Get,
    weights::{DispatchClass, DispatchInfo, Weight}
};
use frame_system as system;
use pallet_balances::Call as BalancesCall;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup}
};
use std::cell::RefCell;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPaymentFree: pallet_transaction_payment_free::{Module},
    }
);

pub const CALL: &<Test as frame_system::Config>::Call = &Call::Balances(BalancesCall::transfer(2, 69));

thread_local! {
    static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
}

pub struct BlockWeights;
impl Get<frame_system::limits::BlockWeights> for BlockWeights {
    fn get() -> frame_system::limits::BlockWeights {
        frame_system::limits::BlockWeights::builder()
            .base_block(0)
            .for_class(DispatchClass::all(), |weights| {
                weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow()).into();
            })
            .for_class(DispatchClass::non_mandatory(), |weights| {
                weights.max_total = 1024.into();
            })
            .build_or_panic()
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type WeightInfo = ();
}

impl Config for Test {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
    DispatchInfo {
        weight: w,
        ..Default::default()
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10)]
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
