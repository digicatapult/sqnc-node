#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use frame_support::{
    derive_impl,
    traits::{ConstU128, ConstU32, ConstU64, EitherOfDiverse, EqualPrivilegeOnly, InstanceFilter},
};
use frame_system::EnsureRoot;
use pallet_grandpa::AuthorityId as GrandpaId;
use scale_info::prelude::string::String;
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT, NumberFor};
use sp_runtime::{
    generic, impl_opaque_keys,
    traits::OpaqueKeys,
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, RuntimeDebug,
};
use sp_std::borrow::Cow;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::genesis_builder_helper::{build_state, get_preset};

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime,
    dispatch::{DispatchInfo, GetDispatchInfo},
    parameter_types,
    traits::{ChangeMembers, InitializeMembers, KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment_free::CurrencyAdapter;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub use sqnc_pallet_traits::ValidateProcessWeights;

pub use sqnc_runtime_types::*;

pub mod constants;
mod utils;
pub mod weights;

use crate::constants::time::*;
use utils::MembersNotifyBoth;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
        }
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: Cow::Borrowed("sqnc"),
    impl_name: Cow::Borrowed("sqnc"),
    authoring_version: 1,
    spec_version: 1140,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: PRIMARY_PROBABILITY,
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
    };

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

type MoreThanHalfMembers = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionMoreThan<AccountId, GovernanceCollective, 1, 2>,
>;

type MoreThanTwoMembers =
    EitherOfDiverse<EnsureRoot<AccountId>, pallet_collective::EnsureMembers<AccountId, GovernanceCollective, 2>>;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: frame_system::limits::BlockWeights =
    frame_system::limits::BlockWeights::with_sensible_defaults(
        Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
        NORMAL_DISPATCH_RATIO,
    );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The block type for the runtime.
    type Block = Block;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = constants::weights::RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// The maximum number of consumers allowed on a single account.
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    // Weights for system extrinsics
    type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
}

parameter_types! {
    // NOTE: Currently it is not possible to change the epoch duration after the chain has started.
    //       Attempting to do so will brick block production.
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    // pub const ReportLongevity: u64 =
    // 	BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type DisabledValidators = Session;
    type WeightInfo = (); // not using actual as benchmark does not produce valid WeightInfo
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type WeightInfo = (); // not actual local as benchmark does not produce valid WeightInfo
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type MaxNominators = ConstU32<0>;

    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type AccountStore = System;
    type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl pallet_transaction_payment_free::Config for Runtime {
    type OnFreeTransaction = CurrencyAdapter<Balances, ()>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
}

impl pallet_doas::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Call = RuntimeCall;
    type DoasOrigin = MoreThanHalfMembers;
}

impl pallet_node_authorization::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxWellKnownNodes = ConstU32<16>;
    type MaxPeerIdLength = ConstU32<128>;
    type AddOrigin = MoreThanHalfMembers;
    type RemoveOrigin = MoreThanHalfMembers;
    type SwapOrigin = MoreThanHalfMembers;
    type ResetOrigin = MoreThanHalfMembers;
    type WeightInfo = (); // no benchmarks defined in pallet!
}

parameter_types! {
    pub const ProxyDepositBase: Balance = 0; // 0 indicates no cost to creating proxies - may be prone to spam
    pub const ProxyDepositFactor: Balance = 0;
    pub const AnnouncementDepositBase: Balance = 0;
    pub const AnnouncementDepositFactor: Balance = 0;

}

/// The type used to represent the kinds of proxying allowed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum ProxyType {
    RunProcess,
    Any,
    Governance,
}
impl Default for ProxyType {
    fn default() -> Self {
        Self::Any
    }
}

// Determines which calls a proxy type can execute.
impl InstanceFilter<RuntimeCall> for ProxyType {
    fn filter(&self, c: &RuntimeCall) -> bool {
        match self {
            ProxyType::Any => true,
            ProxyType::RunProcess => matches!(c, RuntimeCall::UtxoNFT(..)),
            ProxyType::Governance => matches!(c, RuntimeCall::TechnicalCommittee(..)),
        }
    }
    fn is_superset(&self, o: &Self) -> bool {
        match (self, o) {
            (x, y) if x == y => true,
            (ProxyType::Any, _) => true,
            (_, ProxyType::Any) => false,
            _ => false,
        }
    }
}

impl pallet_proxy::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type Currency = Balances;
    type ProxyType = ProxyType; // Custom Proxy Types
    type ProxyDepositBase = ProxyDepositBase;
    type ProxyDepositFactor = ProxyDepositFactor;
    type MaxProxies = ConstU32<32>;
    type MaxPending = ConstU32<32>;
    type CallHasher = BlakeTwo256;
    type AnnouncementDepositBase = AnnouncementDepositBase;
    type AnnouncementDepositFactor = AnnouncementDepositFactor;
    type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        BlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
    type Preimages = Preimage;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = ConstU32<50>;
    type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = ();
}

parameter_types! {
    pub const TokenTombstoneDuration: BlockNumber = 7 * DAYS;
}

impl pallet_utxo_nft::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenId = TokenId;
    type RoleKey = Role;
    type TokenMetadataKey = TokenMetadataKey;
    type TokenMetadataValue = TokenMetadataValue;
    type ProcessValidator = ProcessValidation;
    type WeightInfo = weights::pallet_utxo_nft::WeightInfo<Runtime>;
    type MaxMetadataCount = ConstU32<64>;
    type MaxRoleCount = ConstU32<16>;
    type MaxInputCount = ConstU32<64>;
    type MaxOutputCount = ConstU32<64>;
    type TokenTombstoneDuration = TokenTombstoneDuration;
}

impl pallet_process_validation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = ProcessVersion;
    type CreateProcessOrigin = MoreThanTwoMembers;
    type DisableProcessOrigin = MoreThanTwoMembers;
    type WeightInfo = weights::pallet_process_validation::WeightInfo<Runtime>;
    type TokenId = TokenId;
    type RoleKey = Role;
    type TokenMetadataKey = TokenMetadataKey;
    type TokenMetadataValue = TokenMetadataValue;
    type TokenMetadataValueDiscriminator = MetadataValueType;
    type MaxProcessProgramLength = MaxProcessProgramLength;
}

impl pallet_organisation_data::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OrgDataKey = OrgDataKey;
    type OrgDataValue = OrgDataValue;
    type MaxOrgMemberEntries = ConstU32<ORG_DATA_KEY_COUNT>;
    type WeightInfo = weights::pallet_organisation_data::WeightInfo<Runtime>;
}

parameter_types! {
    pub const GovernanceMotionDuration: BlockNumber = 7 * DAYS;
    pub const GovernanceMaxProposals: u32 = 100;
    pub const GovernanceMaxMembers: u32 = 100;
    pub MaxProposalWeight: Weight = sp_runtime::Perbill::from_percent(50) * BlockWeights::get().max_block;
}

type GovernanceCollective = pallet_collective::Instance1;
impl pallet_collective::Config<GovernanceCollective> for Runtime {
    type RuntimeOrigin = RuntimeOrigin;
    type Proposal = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type MotionDuration = GovernanceMotionDuration;
    type MaxProposals = GovernanceMaxProposals;
    type MaxMembers = GovernanceMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
    type SetMembersOrigin = MoreThanHalfMembers;
    type MaxProposalWeight = MaxProposalWeight;
    type DisapproveOrigin = EnsureRoot<AccountId>; // Only root can disapprove proposals without penalty
    type KillOrigin = EnsureRoot<AccountId>; // Only root can kill potentially malicious proposals
    type Consideration = ();
}

type GovernanceMembershipInstance = pallet_membership::Instance1;
impl pallet_membership::Config<GovernanceMembershipInstance> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AddOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type SwapOrigin = MoreThanHalfMembers;
    type ResetOrigin = MoreThanHalfMembers;
    type PrimeOrigin = EnsureRoot<AccountId>;
    type MembershipInitialized = MembersNotifyBoth<TechnicalCommittee, OrganisationData>;
    type MembershipChanged = MembersNotifyBoth<TechnicalCommittee, OrganisationData>;
    type MaxMembers = ConstU32<100>;
    type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

parameter_types! {
    pub const KeyLength: u32 = 32;
    pub const RefreshPeriod: u32 = 7 * DAYS;
}

impl pallet_symmetric_key::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type KeyLength = KeyLength;
    type RefreshPeriod = RefreshPeriod;
    type RuntimeCall = RuntimeCall;
    type UpdateOrigin = MoreThanHalfMembers;
    type RotateOrigin = MoreThanTwoMembers;
    type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
    type PalletsOrigin = OriginCaller;
    type Scheduler = Scheduler;
    type WeightInfo = weights::pallet_symmetric_key::WeightInfo<Runtime>;
    type Preimages = Preimage;
}

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = pallet_validator_set::ValidatorOf<Self>;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = ValidatorSet;
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = opaque::SessionKeys;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinAuthorities: u32 = 2;
}

impl pallet_validator_set::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AddRemoveOrigin = MoreThanHalfMembers;
    type MinAuthorities = MinAuthorities;
    type WeightInfo = pallet_validator_set::weights::SubstrateWeight<Runtime>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        ValidatorSet: pallet_validator_set,
        Session: pallet_session,
        Babe: pallet_babe,
        Grandpa: pallet_grandpa,
        TransactionPaymentFree: pallet_transaction_payment_free,
        Sudo: pallet_sudo,
        UtxoNFT: pallet_utxo_nft,
        ProcessValidation: pallet_process_validation,
        NodeAuthorization: pallet_node_authorization,
        Preimage: pallet_preimage,
        Scheduler: pallet_scheduler,
        IpfsKey: pallet_symmetric_key,
        Membership: pallet_membership::<Instance1>,
        TechnicalCommittee: pallet_collective::<Instance1>,
        OrganisationData: pallet_organisation_data,
        Doas: pallet_doas,
        Proxy: pallet_proxy
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment_free::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

type Migrations = (
    pallet_symmetric_key::migrations::v1::MigrateToV1<Runtime>,
    pallet_organisation_data::migrations::v1::MigrateToV1<Runtime, GovernanceMembershipInstance>,
);

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_benchmarking, BaselineBench::<Runtime>]
        [frame_system, SystemBench::<Runtime>]
        [pallet_babe, Babe]
        [pallet_balances, Balances]
        [pallet_collective, TechnicalCommittee]
        [pallet_grandpa, Grandpa]
        [pallet_membership, Membership]
        [pallet_preimage, Preimage]
        [pallet_process_validation, ProcessValidation]
        [pallet_scheduler, Scheduler]
        [pallet_sudo, Sudo]
        [pallet_symmetric_key, IpfsKey]
        [pallet_timestamp, Timestamp]
        [pallet_utxo_nft, UtxoNFT]
        [pallet_proxy, Proxy]
        [pallet_organisation_data, OrganisationData]
    );
}

#[cfg(feature = "runtime-benchmarks")]
impl frame_system_benchmarking::Config for Runtime {}

#[cfg(feature = "runtime-benchmarks")]
impl frame_benchmarking::baseline::Config for Runtime {}

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeConfiguration {
            let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
            sp_consensus_babe::BabeConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDuration::get(),
                c: epoch_config.c,
                authorities: Babe::authorities().to_vec(),
                randomness: Babe::randomness(),
                allowed_slots: epoch_config.allowed_slots,
            }
        }

        fn current_epoch_start() -> sp_consensus_babe::Slot {
            Babe::current_epoch_start()
        }

        fn current_epoch() -> sp_consensus_babe::Epoch {
            Babe::current_epoch()
        }

        fn next_epoch() -> sp_consensus_babe::Epoch {
            Babe::next_epoch()
        }

        fn generate_key_ownership_proof(
            _slot: sp_consensus_babe::Slot,
            _authority_id: sp_consensus_babe::AuthorityId,
        ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
            None
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
            key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Babe::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    // Certain elements of the polkadot UI assume the presence of the payment RPC API
    // Because we do not have transaction fees we implement the API but returning zero fees
    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            _len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            let dispatch_info = frame_support::dispatch::GetDispatchInfo::get_dispatch_info(&uxt);
            let DispatchInfo { call_weight, class, .. } = dispatch_info;

            pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo {
                weight: call_weight,
                class,
                partial_fee: 0u32.into()
            }
        }

        fn query_fee_details(
            _uxt: <Block as BlockT>::Extrinsic,
            _len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::FeeDetails<Balance> {
            pallet_transaction_payment_rpc_runtime_api::FeeDetails {
                inclusion_fee: None,
                tip: 0u32.into()
            }
        }

        fn query_weight_to_fee(_: Weight) -> Balance { 0 }
        fn query_length_to_fee(_: u32) -> Balance { 0 }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn benchmark_metadata(extra: bool) -> (
            Vec<frame_benchmarking::BenchmarkList>,
            Vec<frame_support::traits::StorageInfo>,
        ) {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
            use frame_support::traits::StorageInfoTrait;
            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            let mut list = Vec::<BenchmarkList>::new();
            list_benchmarks!(list, extra);

            let storage_info = AllPalletsWithSystem::storage_info();

            (list, storage_info)
        }

        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, String> {
            use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
            use sp_storage::TrackedStorageKey;

            use frame_system_benchmarking::Pallet as SystemBench;
            use baseline::Pallet as BaselineBench;

            use frame_support::traits::WhitelistedStorageKeys;
            let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);
            add_benchmarks!(params, batches);

            Ok(batches)
        }
    }

    #[cfg(feature = "try-runtime")]
    impl frame_try_runtime::TryRuntime<Block> for Runtime {
        fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here. If any of the pre/post migration checks fail, we shall stop
            // right here and right now.
            let weight = Executive::try_runtime_upgrade(checks).unwrap();
            (weight, BlockWeights::get().max_block)
        }

        fn execute_block(
            block: Block,
            state_root_check: bool,
            signature_check: bool,
            select: frame_try_runtime::TryStateSelect
        ) -> Weight {
            // NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
            // have a backtrace here.
            Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::traits::WhitelistedStorageKeys;
    use sp_core::hexdisplay::HexDisplay;
    use std::collections::HashSet;

    #[test]
    fn check_whitelist() {
        let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
            .iter()
            .map(|e| HexDisplay::from(&e.key).to_string())
            .collect();

        // Block Number
        assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac"));
        // Total Issuance
        assert!(whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80"));
        // Execution Phase
        assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a"));
        // Event Count
        assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850"));
        // System Events
        assert!(whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"));
    }
}
