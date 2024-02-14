// Creating mock runtime here

use crate as pallet_utxo_nft;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, ConstU64, Hooks},
    weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use frame_system as system;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;
use sqnc_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator, ValidationResult};

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        UtxoNFT: pallet_utxo_nft,
    }
);
parameter_types! {
    pub const SS58Prefix: u8 = 42;
    pub BlockWeights: frame_system::limits::BlockWeights =
      frame_system::limits::BlockWeights::simple_max(
        Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
      );
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type Block = Block;
}

#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, TypeInfo, Debug, Eq, Ord, PartialOrd)]
pub enum Role {
    Owner,
    NotOwner,
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
    None,
}

impl<T> Default for MetadataValue<T> {
    fn default() -> Self {
        MetadataValue::None
    }
}

#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, TypeInfo, Debug, Eq)]
pub enum ProcessIdentifier {
    ShouldSucceed,
    ShouldFail,
}

impl Default for ProcessIdentifier {
    fn default() -> Self {
        ProcessIdentifier::ShouldSucceed
    }
}

pub struct MockProcessValidator {}

type TestProcessId = ProcessFullyQualifiedId<ProcessIdentifier, u32>;
type TestProcessIO = ProcessIO<u64, u64, Role, u64, MetadataValue<u64>>;

impl ProcessValidator<u64, u64, Role, u64, MetadataValue<u64>> for MockProcessValidator {
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = u32;
    type WeightArg = u32;
    type Weights = ();

    fn validate_process(
        id: &TestProcessId,
        _sender: &u64,
        _inputs: &Vec<TestProcessIO>,
        _outputs: &Vec<TestProcessIO>,
    ) -> ValidationResult<u32> {
        ValidationResult {
            success: id.id.clone() == ProcessIdentifier::ShouldSucceed,
            executed_len: 0u32,
        }
    }
}

impl pallet_utxo_nft::Config for Test {
    type RuntimeEvent = RuntimeEvent;

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
    type TokenTombstoneDuration = ConstU64<100u64>;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

pub fn run_to_block(n: u64, on_idle: bool) {
    while System::block_number() < n {
        System::set_block_number(System::block_number() + 1);
        if on_idle {
            UtxoNFT::on_idle(System::block_number(), BlockWeights::get().max_block);
        }
    }
}
