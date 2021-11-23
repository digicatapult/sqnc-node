use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::OpaquePeerId; // A struct wraps Vec<u8>, represents as our `PeerId`.
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use vitalam_node_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, NodeAuthorizationConfig, MembershipConfig, Signature,
    SudoConfig, SystemConfig, WASM_BINARY,
};
const DEFAULT_PROTOCOL_ID: &str = "vam";

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
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
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
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                Some(NodeAuthorizationConfig {
                    nodes: vec![(
                        //0000000000000000000000000000000000000000000000000000000000000001
                        OpaquePeerId(
                            bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                                .into_vec()
                                .unwrap(),
                        ),
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                    )],
                }),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn vitalam_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/vitalam.json")[..])
}

pub fn vitalam_staging_testnet_config() -> Result<ChainSpec, String> {
    use hex_literal::hex;
    use sp_core::crypto::UncheckedInto;

    let wasm_binary = WASM_BINARY.ok_or_else(|| "VITALam development wasm binary not available".to_string())?;

    let boot_nodes = vec![];

    let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
        (
            // 5E7Qn3hDL4qXps6FsczGnYdmQ9t6BV1UjW19nEdpve6EWc3W
            hex!["5a88b53191215d2bef4e3002ff84908f919672340c9cb0aa40c6b3320409a927"].unchecked_into(),
            // 5CMz6kYDQFy8B9K8J5Cakzw5TwYx8v5jLPhUfVn9v5uMwWiZ
            hex!["0d2dd99bae72c439e6ddd731d069c1d5ea6aa0383e6fb579363d5b48b8c34ff5"].unchecked_into(),
        ),
        (
            // 5EkgBhtK7JiaeC6PDP82hVeqncLDhQM8fqK6tvNUe6nuMYWh
            hex!["76f4a7c6f6690a2a196125ffcc8d6e3af2be7379a74531cd4e0eafa394313d01"].unchecked_into(),
            // 5HQBtVLoJvLA6nK2suz47jZT6db15jFbh96iD4yWQVJnoKtU
            hex!["ec0931286eadc24545a14e3503a6b15c79e642f35f2cae44b726d9fb0e9d97ef"].unchecked_into(),
        ),
        (
            // 5Cm8ya7njhAXPCdP2P3rGjjV5av48jPzfrCBFbc5JB5x9aJo
            hex!["1ed65fd3e37014b99db31b2975d05560a1f2d188f442f0de6d89b0e9054c504e"].unchecked_into(),
            // 5DZbNy4aaqvRPQ1xNLuNrV51fiSjwWHra1UMcxhSKrSHNBub
            hex!["42448b6b08a29f85c44e5f03c37382783c6ce12e9b70e8a1017614c2eda47dc3"].unchecked_into(),
        ),
        (
            // 5Evf8up8C1Qh488H2KBzDkuLn4LmLLtcYQhunbweJkU1FavH
            hex!["7e9194b87f1e67ae97e7c0458f1c51ab7127422bb157b2078c2533b15fc3f86d"].unchecked_into(),
            // 5CkpVgQ83AB6vahdBqtyKrUxK491yEHd6RmRAGNET4DRMTsh
            hex!["1e98297f74ad007f0309081342072b11df59fbcd098b99370f166f7d3be6376a"].unchecked_into(),
        ),
        (
            // 5CwVqgwFmsgKC3xjYWrRGu7CJEpALSq1MLs6exgaHDSvW2sG
            hex!["26bd102c6d449f21d036579f383fbf492030e9ddb07d429cfd78fae3419eba02"].unchecked_into(),
            // 5EvHnoTpkVm6zwsthkYaXhHCXsccB6EQEgWDzENYpwSec7cH
            hex!["7e49b8b3a48fc928e94b3b0616f1a2714cb1ed4ca73abbf4edc25aece106dfe9"].unchecked_into(),
        ),
        (
            // 5CSHAna9aDyUFErZhshNbVdRtTWDNt2F9pJsZzZYkP4PypjY
            hex!["10744afe5e300a98cc23e03a365333426402c735f5bd14b6c845dc281a511271"].unchecked_into(),
            // 5GTbtzp228gMZ8CveKaBvUFDA6HfGLgVGq2NDo67Xaxmwckj
            hex!["c26783f623ee652ee1bfb8d61b01484550d2e96832c7d1de835f25ed4695abac"].unchecked_into(),
        ),
        (
            // 5ERAwVtfGy7KFc1mXr3ivRFzGr4zBjkNhQdaWc9SKJs93Gye
            hex!["68148ba1c62e57d4bf08dbe63075bc12bd0bff3ef53851d602c8430bb5907f3b"].unchecked_into(),
            // 5EGAgksz71AXxspnsavbGUcqFQneEmVXNrzTJ7tbBcvaYiTf
            hex!["61367b30095778e2436f9057d58cf9326dac79375f051c2b789fb58abfe032d6"].unchecked_into(),
        ),
        (
            // 5HmkH1TTo5n7nxnzRRmPm7cpjZymheAtN7WnDxWqFVTLDnJ4
            hex!["fc7a5fc6a9fdf9fabf9189002adae2d4632824da636f2fc586f27a59a3c9a969"].unchecked_into(),
            // 5DBAYkZyVCMSugXiRRsrVgePrbMvE9dmgTZ9MfDbW7yCFXBC
            hex!["31298e5c802bc8ad6d1c9218866ff8650a08ceafc01344ce172c6106ecbf6f5b"].unchecked_into(),
        ),
        (
            // 5GHBPvaw2qLimoKZZiMu3f3xbnoFtf35naKdrLazW5EisuVE
            hex!["ba749535b22617bd888e85e295c57e4efb974663709048086dee3d006da7d928"].unchecked_into(),
            // 5GgehjSu4oJaDxi5iSzExReUEWD4FSPVCyk3M3p8kkqpDsb6
            hex!["cc5b269c2e1bf25d14b9b24454fd59e3b5c3bff629434ce57939f297cc80595d"].unchecked_into(),
        ),
    ];

    let endowed_accounts: Vec<AccountId> = vec![
        // 5Fpk38Xk2eixKwtNRnUn1YTcUuKbgSmF8G2wgRoqGUmgtG1o
        hex!["a64ad684438ae298d875f66e24e4f98f2d4689e008d431534f5c6adffd1ea26b"].into(),
        // 5DtQJtJurJbniRA1LaFu9JgoL5Myi4C29EBfqtoJpcoEydLp
        hex!["509cf0e9e9c49855cf888f9af6bd481174399d6fdbe4d43041e6ed8a960b6a2c"].into(),
        // 5H8iVfXZ89B8eByqEdwdRYuVJReEeETnC24GdVXj4LjSg27z
        hex!["e03c4bcf92683e004bb95bfffef863e4ecf1c9a1fc661c5807ac48e22b060b20"].into(),
    ];

    let sudo_account: AccountId = hex!["ce175f5d1247802ec06857659a5753d52b8e80ae38a470bc97a10bd285596e79"].into();

    Ok(ChainSpec::from_genesis(
        // Name
        "VITALam Staging Testnet",
        // ID
        "vitalam_staging_testnet",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                initial_authorities.iter().map(|k| k.clone()).collect(),
                // Sudo account
                sudo_account.clone(),
                // Pre-funded accounts
                endowed_accounts.iter().map(|k| k.clone()).collect(),
                Some(NodeAuthorizationConfig {
                    nodes: vec![
                        (
                            // stage bootnode
                            OpaquePeerId(
                                bs58::decode("12D3KooWCVhZ8Hm496zf4sugFekVyZtuE98b1XuySQUxztNvAQY6")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[0].clone(),
                        ),
                        (
                            // stage red
                            OpaquePeerId(
                                bs58::decode("12D3KooWRSxrnHyBjePLXcNyCaJE31oWFpoya5PrsYb3DUHP5QW7")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[0].clone(),
                        ),
                        (
                            // stage green
                            OpaquePeerId(
                                bs58::decode("12D3KooWKZGsMiJvispaQgVk2PDjECbT1FFx7VVoEF91wzquSho7")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[0].clone(),
                        ),
                        (
                            // stage blue
                            OpaquePeerId(
                                bs58::decode("12D3KooWSbedmDP2M8odLSFEMshWzAu6Rc8Hq24rw9xq1s1eCJzy")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[0].clone(),
                        ),
                        (
                            // stage2 red
                            OpaquePeerId(
                                bs58::decode("12D3KooWDW6qHkgpnxzb5FPbSwRU3paUirLeiEbTHRBQ7VwfcciV")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[1].clone(),
                        ),
                        (
                            // stage2 green
                            OpaquePeerId(
                                bs58::decode("12D3KooWQ2ignqpwNmXM8gCBaesg7VCupLHkXkcrKge2yzvyiKL4")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[1].clone(),
                        ),
                        (
                            // stage2 blue
                            OpaquePeerId(
                                bs58::decode("12D3KooWFkrb3zMP5Gijk6MYWUShPMRTdXeox3z5ppr4HzKztjDe")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[1].clone(),
                        ),
                        (
                            // stage3 red
                            OpaquePeerId(
                                bs58::decode("12D3KooWG4N23D8Ny4ZPyqv4zBwJN3YGUtrhURP8ZUAkJTip14hK")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[2].clone(),
                        ),
                        (
                            // stage3 green
                            OpaquePeerId(
                                bs58::decode("12D3KooWN48NLTwN4gyjbaPYX1hzgmewGKN3iYAphh24NaZb5TWF")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[2].clone(),
                        ),
                        (
                            // stage3 blue
                            OpaquePeerId(
                                bs58::decode("12D3KooWHxeSevn6pdXCfaz13eEKPEgubZbt3W2mg3Lwaaa5HRpP")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            endowed_accounts[2].clone(),
                        ),
                    ],
                }),
                true,
            )
        },
        // Bootnodes
        boot_nodes,
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // Properties
        None,
        // Extensions
        None,
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
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                Some(NodeAuthorizationConfig {
                    nodes: vec![
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000001
                            OpaquePeerId(
                                bs58::decode("12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Alice"),
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000002
                            OpaquePeerId(
                                bs58::decode("12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Bob"),
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000003
                            OpaquePeerId(
                                bs58::decode("12D3KooWSCufgHzV4fCwRijfH2k3abrpAJxTKxEvN1FDuRXA2U9x")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        ),
                        (
                            // 0000000000000000000000000000000000000000000000000000000000000004
                            OpaquePeerId(
                                bs58::decode("12D3KooWSsChzF81YDUKpe9Uk5AHV5oqAaXAcWNSPYgoLauUk4st")
                                    .into_vec()
                                    .unwrap(),
                            ),
                            get_account_id_from_seed::<sr25519::Public>("Eve"),
                        ),
                    ],
                }),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some(DEFAULT_PROTOCOL_ID),
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    node_authorization_config: Option<NodeAuthorizationConfig>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
        }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
        pallet_node_authorization: node_authorization_config,
        pallet_membership: Some(MembershipConfig {
             members: endowed_accounts.iter().map(|k| k.clone()).collect(),
             .. Default::default()
        }),
    }
}
