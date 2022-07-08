// Creating mock runtime here

use crate as pallet_simple_nft;
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

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator};

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

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
        SimpleNFT: pallet_simple_nft::{Pallet, Call, Storage, Event<T>},
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

#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, TypeInfo, Debug, Eq, Ord, PartialOrd)]
pub enum Role {
    Owner,
    NotOwner
}

impl Default for Role {
    fn default() -> Self {
        Role::Owner
    }
}

#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, TypeInfo, Debug, Eq)]
pub enum MetadataValue<TokenId> {
    File(Hash),
    Literal([u8; 1]),
    TokenId(TokenId),
    None
}

impl<T> Default for MetadataValue<T> {
    fn default() -> Self {
        MetadataValue::None
    }
}

#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, TypeInfo, Debug, Eq)]
pub enum ProcessIdentifier {
    ShouldSucceed,
    ShouldFail
}

impl Default for ProcessIdentifier {
    fn default() -> Self {
        ProcessIdentifier::ShouldSucceed
    }
}

pub struct MockProcessValidator {}

type TestProcessId = ProcessFullyQualifiedId<ProcessIdentifier, u32>;
type TestProcessIO = ProcessIO<u64, Role, u64, MetadataValue<u64>>;

impl ProcessValidator<u64, Role, u64, MetadataValue<u64>> for MockProcessValidator {
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = u32;

    fn validate_process(
        id: TestProcessId,
        _sender: &u64,
        _inputs: &Vec<TestProcessIO>,
        _outputs: &Vec<TestProcessIO>
    ) -> bool {
        id.id == ProcessIdentifier::ShouldSucceed
    }
}

impl pallet_simple_nft::Config for Test {
    type Event = Event;

    type TokenId = u64;
    type RoleKey = Role;
    type TokenMetadataKey = u64;
    type TokenMetadataValue = MetadataValue<Self::TokenId>;

    type ProcessValidator = MockProcessValidator;
    type WeightInfo = ();

    type MaxMetadataCount = ConstU32<4>;
    type MaxRoleCount = ConstU32<2>;
    type MaxInputCount = ConstU32<5>;
    type MaxOutputCount = ConstU32<5>;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
