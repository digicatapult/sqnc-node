use dscp_node_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, MembershipConfig, NodeAuthorizationConfig,
    Signature, SudoConfig, SystemConfig, WASM_BINARY
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::OpaquePeerId; // A struct wraps Vec<u8>, represents as our `PeerId`.
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
const DEFAULT_PROTOCOL_ID: &str = "dscp";

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                NodeAuthorizationConfig {
                    nodes: vec![(
                        //0000000000000000000000000000000000000000000000000000000000000001
                        OpaquePeerId(
                            bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                                .into_vec()
                                .unwrap()
                        ),
                        get_account_id_from_seed::<sr25519::Public>("Alice")
                    )]
                },
                true
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // fork id
        None,
        // Properties
        None,
        // Extensions
        None
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                    authority_keys_from_seed("Charlie"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                NodeAuthorizationConfig {
                    nodes: vec![
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000001
                            OpaquePeerId(
                                bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                                    .into_vec()
                                    .unwrap()
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Alice")
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000002
                            OpaquePeerId(
                                bs58::decode("12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD")
                                    .into_vec()
                                    .unwrap()
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Bob")
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000003
                            OpaquePeerId(
                                bs58::decode("12D3KooWSCufgHzV4fCwRijfH2k3abrpAJxTKxEvN1FDuRXA2U9x")
                                    .into_vec()
                                    .unwrap()
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Charlie")
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000004
                            OpaquePeerId(
                                bs58::decode("12D3KooWSsChzF81YDUKpe9Uk5AHV5oqAaXAcWNSPYgoLauUk4st")
                                    .into_vec()
                                    .unwrap()
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Eve")
                        ),
                    ]
                },
                true
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // fork id
        None,
        // Properties
        None,
        // Extensions
        None
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    technical_committee_accounts: Vec<AccountId>,
    node_authorization_config: NodeAuthorizationConfig,
    _enable_println: bool
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec()
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect()
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect()
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect()
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key)
        },
        node_authorization: node_authorization_config,
        membership: MembershipConfig {
            members: technical_committee_accounts.try_into().unwrap(),
            ..Default::default()
        },
        technical_committee: Default::default()
    }
}
