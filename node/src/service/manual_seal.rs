//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
#![allow(clippy::needless_borrow)]

use futures::FutureExt;
use std::sync::Arc;

use sc_client_api::Backend;
use sc_consensus_manual_seal::{
    consensus::{babe::BabeConsensusDataProvider, timestamp::SlotTimestampProvider},
    ManualSealParams,
};
use sc_service::{error::Error as ServiceError, Configuration, PartialComponents, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_consensus_babe::AuthorityId;
use sp_keyring::sr25519::Keyring::Alice;

use sqnc_runtime::{self, opaque::Block, RuntimeApi};

use crate::{
    rpc::SqncDeps,
    service::{ObservableBlockImport, ProposalFinality},
};

pub type HostFunctions = (
    sp_io::SubstrateHostFunctions,
    sp_statement_store::runtime_api::HostFunctions,
);

pub type RuntimeExecutor = sc_executor::WasmExecutor<HostFunctions>;
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, RuntimeExecutor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport = sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

/// The minimum period of blocks on which justifications will be
/// imported and generated.
const GRANDPA_JUSTIFICATION_PERIOD: u32 = 512;

/// Returns most parts of a service. Not enough to run a full chain,
/// But enough to perform chain operations like purge-chain
fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
        (
            sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
            sc_consensus_babe::BabeLink<Block>,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = sc_service::new_wasm_executor(&config.executor);

    let (client, backend, keystore_container, task_manager) = sc_service::new_full_parts::<Block, RuntimeApi, _>(
        config,
        telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
        executor,
    )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager.spawn_handle().spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());
    let transaction_pool = Arc::from(
        sc_transaction_pool::Builder::new(
            task_manager.spawn_essential_handle(),
            client.clone(),
            config.role.is_authority().into(),
        )
        .with_options(config.transaction_pool.clone())
        .with_prometheus(config.prometheus_registry())
        .build(),
    );

    let (grandpa_block_import, ..) = sc_consensus_grandpa::block_import(
        client.clone(),
        GRANDPA_JUSTIFICATION_PERIOD,
        &client.clone(),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let (block_import, babe_link) = sc_consensus_babe::block_import(
        sc_consensus_babe::configuration(&*client)?,
        grandpa_block_import,
        client.clone(),
    )?;

    let import_queue = sc_consensus_manual_seal::import_queue(
        Box::new(block_import.clone()),
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    Ok(PartialComponents {
        client,
        backend,
        task_manager,
        keystore_container,
        select_chain,
        import_queue,
        transaction_pool,
        other: (block_import, babe_link, telemetry),
    })
}

/// Builds a new service for a full client.
pub fn new_full<N: sc_network::NetworkBackend<Block, <Block as sp_runtime::traits::Block>::Hash>>(
    config: Configuration,
) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, babe_link, mut telemetry),
    } = new_partial(&config)?;

    let net_config =
        sc_network::config::FullNetworkConfiguration::<Block, <Block as sp_runtime::traits::Block>::Hash, N>::new(
            &config.network,
            config.prometheus_registry().cloned(),
        );
    let metrics = N::register_notification_metrics(config.prometheus_registry());

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: None,
            block_relay: None,
            metrics,
        })?;

    if config.offchain_worker.enabled {
        let offchain_workers = sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
            runtime_api_provider: client.clone(),
            keystore: Some(keystore_container.keystore()),
            offchain_db: backend.offchain_storage(),
            transaction_pool: Some(OffchainTransactionPoolFactory::new(transaction_pool.clone())),
            network_provider: Arc::new(network.clone()),
            is_validator: config.role.is_authority(),
            enable_http_requests: true,
            custom_extensions: |_| vec![],
        })?;

        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            offchain_workers
                .run(client.clone(), task_manager.spawn_handle())
                .boxed(),
        );
    }

    let role = config.role.clone();
    let prometheus_registry = config.prometheus_registry().cloned();

    let (sqnc_deps, proposal_finality_request_handler) = SqncDeps::new(sync_service.clone());
    let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);
    let rpc_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let sqnc_deps = sqnc_deps.clone();

        Box::new(move |_| {
            let deps = crate::rpc::TestDeps {
                client: client.clone(),
                pool: pool.clone(),
                command_sink: command_sink.clone(),
                sqnc: sqnc_deps.clone(),
            };

            crate::rpc::create_test(deps).map_err(Into::into)
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        config,
        backend,
        client: client.clone(),
        keystore: keystore_container.keystore(),
        network: network.clone(),
        rpc_builder: Box::new(rpc_builder),
        transaction_pool: transaction_pool.clone(),
        task_manager: &mut task_manager,
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        telemetry: telemetry.as_mut(),
    })?;

    let (import_authored_block, receiver) = ObservableBlockImport::new(block_import);
    let proposal_log = ProposalFinality::new(client.clone(), receiver, proposal_finality_request_handler);

    task_manager.spawn_handle().spawn_blocking(
        "import-block-proposal",
        Some("block-authoring"),
        proposal_log.start_proposal_log(),
    );

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(Telemetry::handle),
        );

        let babe_data_provider = BabeConsensusDataProvider::new(
            client.clone(),
            keystore_container.keystore(),
            babe_link.epoch_changes().clone(),
            vec![(AuthorityId::from(Alice.public()), 1000)],
        )
        .expect("");

        // Background authorship future.
        let authorship_future = sc_consensus_manual_seal::run_manual_seal(ManualSealParams {
            block_import: import_authored_block,
            env: proposer,
            client: client.clone(),
            pool: transaction_pool.clone(),
            commands_stream,
            select_chain,
            consensus_data_provider: Some(Box::new(babe_data_provider)),
            create_inherent_data_providers: move |_, ()| {
                let client = client.clone();
                async move {
                    // Ok(sp_timestamp::InherentDataProvider::from_system_time())
                    let timestamp =
                        SlotTimestampProvider::new_babe(client.clone()).map_err(|err| format!("{:?}", err))?;
                    let babe = sp_consensus_babe::inherents::InherentDataProvider::new(timestamp.slot());
                    Ok((timestamp, babe))
                }
            },
        });

        // we spawn the future on a background thread managed by service.
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("manual-seal", Some("block-authoring"), authorship_future);
    };

    network_starter.start_network();
    Ok(task_manager)
}
