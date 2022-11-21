// Creating mock runtime here

use crate as pallet_symmetric_key;
use frame_support::{
    parameter_types,
    traits::{ConstU32, EqualPrivilegeOnly, OnInitialize, OnFinalize},
    weights::Weight,
    BoundedVec
};
use frame_support_test::TestRandomness;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill
};

mod rotate_key;
mod schedule;
mod update_key;

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
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>},
        SymmetricKey: pallet_symmetric_key::{Pallet, Call, Storage, Event<T>},
    }
);
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(Weight::from_ref_time(1_000_000).set_proof_size(u64::MAX));
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}
parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}
impl pallet_scheduler::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = system::EnsureRoot<u64>;
    type MaxScheduledPerBlock = ();
    type WeightInfo = ();
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type Preimages = ();
}

parameter_types! {
    pub const RefreshPeriod: u32 = 5;
}
impl pallet_symmetric_key::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type KeyLength = ConstU32<32>;
    type RefreshPeriod = RefreshPeriod;
    type ScheduleCall = RuntimeCall;
    type UpdateOrigin = system::EnsureRoot<u64>;
    type RotateOrigin = system::EnsureRoot<u64>;
    type Randomness = TestRandomness<Self>;
    type PalletsOrigin = OriginCaller;
    type Scheduler = Scheduler;
    type WeightInfo = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
