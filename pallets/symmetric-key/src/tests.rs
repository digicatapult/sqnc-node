// Creating mock runtime here

use crate as pallet_symmetric_key;
use frame_support::{
    parameter_types,
    weights::{
        Weight,
    },
    traits::{
        TestRandomness,
        OnInitialize,
    }
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

mod update_key;
mod rotate_key;
mod schedule;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Module, Call, Config, Storage, Event<T>},
        Scheduler: pallet_scheduler::{Module, Call, Storage, Config, Event<T>},
        SymmetricKey: pallet_symmetric_key::{Module, Call, Storage, Event<T>},
    }
);
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1_000_000);
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}
parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}
impl pallet_scheduler::Config for Test {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = system::EnsureRoot<u64>;
	type MaxScheduledPerBlock = ();
	type WeightInfo = ();
}

parameter_types! {
    pub const KeyLength: u32 = 32;
    pub const RefreshPeriod: u32 = 5;
}
impl pallet_symmetric_key::Config for Test {
    type Event = Event;
    type KeyLength = KeyLength;
    type RefreshPeriod = RefreshPeriod;
    type ScheduleCall = Call;
    type UpdateOrigin = system::EnsureRoot<u64>;
    type RotateOrigin = system::EnsureRoot<u64>;
    type Randomness = TestRandomness;
    type PalletsOrigin = OriginCaller;
    type Scheduler = Scheduler;
    type WeightInfo = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
