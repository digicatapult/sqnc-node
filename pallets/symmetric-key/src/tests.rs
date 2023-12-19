// Creating mock runtime here

use crate as pallet_symmetric_key;
use frame_support::{
    parameter_types,
    traits::{ConstU32, EqualPrivilegeOnly, OnFinalize, OnInitialize},
    weights::Weight,
    BoundedVec,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
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
        frame_system::limits::BlockWeights::simple_max(
            Weight::from_parts(frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
        );
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
    type MaxScheduledPerBlock = ConstU32<100>;
    type WeightInfo = ();
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type Preimages = ();
}

pub struct TestRandomness<Test>(sp_std::marker::PhantomData<Test>);

impl<Output: parity_scale_codec::Decode + Default, Test> frame_support::traits::Randomness<Output, Test::BlockNumber>
    for TestRandomness<Test>
where
    Test: frame_system::Config,
{
    fn random(_subject: &[u8]) -> (Output, Test::BlockNumber) {
        use sp_runtime::traits::TrailingZeroInput;
        let bn = frame_system::Pallet::<Test>::block_number();
        let bn_u8: u8 = bn.try_into().unwrap_or_default();
        let arr: [u8; 8] = core::array::from_fn(|i| 8 * bn_u8 + i as u8);

        (
            Output::decode(&mut TrailingZeroInput::new(&arr)).unwrap_or_default(),
            frame_system::Pallet::<Test>::block_number(),
        )
    }
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

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Scheduler::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        Scheduler::on_initialize(System::block_number());
    }
}
