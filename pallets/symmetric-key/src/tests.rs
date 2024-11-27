// Creating mock runtime here

use crate as pallet_symmetric_key;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, EqualPrivilegeOnly, OnFinalize, OnInitialize},
    weights::Weight,
    BoundedVec,
};
use frame_system as system;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::{BuildStorage, Perbill};

mod rotate_key;
mod schedule;
mod update_key;

type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test {
        System: system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>},
        SymmetricKey: pallet_symmetric_key::{Pallet, Call, Storage, Event<T>},
        Preimage: pallet_preimage,
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

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type Block = Block;
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
    type Preimages = Preimage;
}

impl pallet_preimage::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Currency = ();
    type ManagerOrigin = frame_system::EnsureRoot<u64>;
    type Consideration = ();
}

pub struct TestRandomness<Test>(sp_std::marker::PhantomData<Test>);

impl<Output: parity_scale_codec::Decode + Default, Test> frame_support::traits::Randomness<Output, BlockNumberFor<Test>>
    for TestRandomness<Test>
where
    Test: frame_system::Config,
{
    fn random(_subject: &[u8]) -> (Output, BlockNumberFor<Test>) {
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
    type RuntimeCall = RuntimeCall;
    type UpdateOrigin = system::EnsureRoot<u64>;
    type RotateOrigin = system::EnsureRoot<u64>;
    type Randomness = TestRandomness<Self>;
    type PalletsOrigin = OriginCaller;
    type Scheduler = Scheduler;
    type Preimages = Preimage;
    type WeightInfo = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Scheduler::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        Scheduler::on_initialize(System::block_number());
    }
}
