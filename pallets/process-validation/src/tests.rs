// Creating mock runtime here

use crate as pallet_process_validation;
use frame_support::{derive_impl, parameter_types, traits::ConstU32};
use frame_system as system;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_runtime::BuildStorage;

mod create_process;
mod disable_process;
mod genesis;
mod validate_process;

type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        ProcessValidation: pallet_process_validation::{Pallet, Call, Storage, Event<T>},
    }
);
parameter_types! {
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type Block = Block;
}

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq, Debug, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProcessIdentifier {
    A,
    B,
}

impl Default for ProcessIdentifier {
    fn default() -> Self {
        ProcessIdentifier::A
    }
}

#[derive(Encode, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq, Debug, Default, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenMetadataValueDiscriminator {
    pub(crate) value: u8,
}
impl From<u128> for TokenMetadataValueDiscriminator {
    fn from(a: u128) -> TokenMetadataValueDiscriminator {
        return TokenMetadataValueDiscriminator { value: (a % 128) as u8 };
    }
}

impl pallet_process_validation::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = u32;
    type CreateProcessOrigin = system::EnsureRoot<u64>;
    type DisableProcessOrigin = system::EnsureRoot<u64>;
    type WeightInfo = ();

    type TokenId = u128;
    type RoleKey = u32;
    type TokenMetadataKey = u32;
    type TokenMetadataValue = u128;
    type TokenMetadataValueDiscriminator = TokenMetadataValueDiscriminator;

    type MaxProcessProgramLength = ConstU32<8>;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

pub fn new_test_ext_with_genesis(genesis: pallet_process_validation::GenesisConfig<Test>) -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
    genesis.assimilate_storage(&mut t).unwrap();
    t.into()
}
