// Creating mock runtime here

use crate as pallet_process_validation;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64}
};
use frame_system as system;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup}
};

mod create_process;
mod disable_process;
mod validate_process;

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
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ProcessValidation: pallet_process_validation::{Pallet, Call, Storage, Event<T>},

    }
);
parameter_types! {
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
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
    type BlockHashCount = ConstU64<250>;
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

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq, Debug, Eq)]
pub enum ProcessIdentifier {
    A,
    B
}

impl Default for ProcessIdentifier {
    fn default() -> Self {
        ProcessIdentifier::A
    }
}

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq, Debug, Default, Eq)]
pub struct TokenMetadataValueDiscriminator {
    value: u8
}
impl From<u128> for TokenMetadataValueDiscriminator {
    fn from(a: u128) -> TokenMetadataValueDiscriminator {
        return TokenMetadataValueDiscriminator { value: (a % 128) as u8 };
    }
}

impl pallet_process_validation::Config for Test {
    type Event = Event;
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = u32;
    type CreateProcessOrigin = system::EnsureRoot<u64>;
    type DisableProcessOrigin = system::EnsureRoot<u64>;
    type WeightInfo = ();

    type RoleKey = u32;
    type TokenMetadataKey = u32;
    type TokenMetadataValue = u128;
    type TokenMetadataValueDiscriminator = TokenMetadataValueDiscriminator;

    type MaxProcessProgramLength = ConstU32<8>;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
