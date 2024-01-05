use dscp_node_runtime::{RuntimeGenesisConfig, WASM_BINARY};
use dscp_runtime_types::{AccountId, RuntimeExpressionSymbol, RuntimeRestriction, Signature};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

const DEFAULT_PROTOCOL_ID: &str = "dscp";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

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
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an authority key.
pub fn authority_keys_from_seed(s: &str) -> (BabeId, GrandpaId) {
    (get_from_seed::<BabeId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Development")
    .with_id("dev")
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
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
        vec![(
            //0000000000000000000000000000000000000000000000000000000000000001
            bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                .into_vec()
                .unwrap(),
            get_account_id_from_seed::<sr25519::Public>("Alice"),
        )],
    ))
    .build())
}

pub fn l3_prod_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/prod-l3-azure.json")[..])
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_protocol_id(DEFAULT_PROTOCOL_ID)
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(testnet_genesis(
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
        vec![
            (
                // 0000000000000000000000000000000000000000000000000000000000000001
                bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                    .into_vec()
                    .unwrap(),
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            ),
            (
                // 0000000000000000000000000000000000000000000000000000000000000002
                bs58::decode("12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD")
                    .into_vec()
                    .unwrap(),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
            ),
            (
                // 0000000000000000000000000000000000000000000000000000000000000003
                bs58::decode("12D3KooWSCufgHzV4fCwRijfH2k3abrpAJxTKxEvN1FDuRXA2U9x")
                    .into_vec()
                    .unwrap(),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
            ),
            (
                // 0000000000000000000000000000000000000000000000000000000000000004
                bs58::decode("12D3KooWSsChzF81YDUKpe9Uk5AHV5oqAaXAcWNSPYgoLauUk4st")
                    .into_vec()
                    .unwrap(),
                get_account_id_from_seed::<sr25519::Public>("Eve"),
            ),
        ],
    ))
    .build())
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    initial_authorities: Vec<(BabeId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    technical_committee_accounts: Vec<AccountId>,
    authorized_nodes: Vec<(Vec<u8>, AccountId)>,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1i64 << 60)).collect::<Vec<_>>(),
        },
        "babe": {
            "authorities": initial_authorities.iter().map(|x| (x.0.clone(), 1)).collect::<Vec<_>>(),
            "epochConfig": Some(dscp_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        "grandpa": {
            "authorities": initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
        },
        "sudo": {
            "key": Some(root_key),
        },
        "nodeAuthorization": {
            "nodes": authorized_nodes,
        },
        "membership": {
            "members": technical_committee_accounts,
        },
        "technicalCommittee": {
            "members": Vec::<AccountId>::new()
        },
        "processValidation": {
            "processes": vec![(
                "default".as_bytes().to_vec(),
                vec![RuntimeExpressionSymbol::Restriction(RuntimeRestriction::None)],
            )],
        },
    })
}
