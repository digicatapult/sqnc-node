//! Mock helpers for Validator Set pallet.

#![cfg(test)]

use crate as pallet_organisation_data;

use frame_support::derive_impl;
use frame_system::{config_preludes::TestDefaultConfig, DefaultConfig, EnsureRoot};
use sp_core::ConstU32;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        OrganisationData: pallet_organisation_data,
        Membership: pallet_membership
    }
);

#[derive_impl(TestDefaultConfig as DefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

type AccountId = <TestDefaultConfig as DefaultConfig>::AccountId;
impl pallet_membership::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AddOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = EnsureRoot<AccountId>;
    type ResetOrigin = EnsureRoot<AccountId>;
    type PrimeOrigin = EnsureRoot<AccountId>;
    type MembershipInitialized = ();
    type MembershipChanged = ();
    type MaxMembers = ConstU32<100>;
    type WeightInfo = ();
}

impl pallet_organisation_data::Config for Test {
    type RuntimeEvent = RuntimeEvent;

    type OrgDataKey = u8;
    type OrgDataValue = u32;

    type MaxOrgMemberEntries = ConstU32<2u32>;

    type WeightInfo = ();
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
