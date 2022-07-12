use super::*;
use crate as pallet_transaction_payment_free;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, Get},
    weights::{DispatchClass, DispatchInfo, Weight}
};
use frame_system as system;
use pallet_balances::Call as BalancesCall;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup}
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPaymentFree: pallet_transaction_payment_free::{Pallet},
    }
);

pub const CALL: &<Test as frame_system::Config>::Call = &Call::Balances(BalancesCall::transfer { dest: 2, value: 69 });

pub struct BlockWeights;
impl Get<frame_system::limits::BlockWeights> for BlockWeights {
    fn get() -> frame_system::limits::BlockWeights {
        frame_system::limits::BlockWeights::builder()
            .base_block(0)
            .for_class(DispatchClass::all(), |weights| {
                weights.base_extrinsic = 0u64.into();
            })
            .for_class(DispatchClass::non_mandatory(), |weights| {
                weights.max_total = 1024.into();
            })
            .build_or_panic()
    }
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
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
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
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
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

impl Config for Test {
    type OnFreeTransaction = CurrencyAdapter<Balances, ()>;
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
    DispatchInfo {
        weight: w,
        ..Default::default()
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10)]
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
