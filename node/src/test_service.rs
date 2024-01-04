//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
#![allow(clippy::needless_borrow)]

use futures::{channel::mpsc::Receiver, FutureExt};
use std::sync::Arc;

use sc_client_api::Backend;
use sc_consensus_manual_seal::{
    consensus::{babe::BabeConsensusDataProvider, timestamp::SlotTimestampProvider},
    EngineCommand, ManualSealParams,
};
pub use sc_executor::NativeElseWasmExecutor;
use sc_service::{error::Error as ServiceError, Configuration, PartialComponents, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_consensus_babe::AuthorityId;
use sp_core::H256;
use sp_keyring::sr25519::Keyring::Alice;

use dscp_node_runtime::{self, opaque::Block, RuntimeApi};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    /// Only enable the benchmarking host functions when we actually want to benchmark.
    #[cfg(feature = "runtime-benchmarks")]
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
    /// Otherwise we only use the default Substrate host functions.
    #[cfg(not(feature = "runtime-benchmarks"))]
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        dscp_node_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        dscp_node_runtime::native_version()
    }
}

pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
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
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            impl Fn(
                crate::rpc::DenyUnsafe,
                sc_rpc::SubscriptionTaskExecutor,
            ) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>,
            sc_consensus_babe::BabeBlockImport<Block, FullClient, FullGrandpaBlockImport>,
            sc_consensus_babe::BabeLink<Block>,
            Receiver<EngineCommand<H256>>,
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

    let executor = sc_service::new_native_or_wasm_executor(config);

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

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
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

    let (command_sink, commands_stream) = futures::channel::mpsc::channel(1000);
    let rpc_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        Box::new(move |deny_unsafe, _| {
            let deps = crate::rpc::TestDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
                command_sink: command_sink.clone(),
            };

            crate::rpc::create_test(deps).map_err(Into::into)
        })
    };

    Ok(PartialComponents {
        client,
        backend,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain,
        other: (rpc_builder, block_import, babe_link, commands_stream, telemetry),
    })
}

/// Builds a new service for a full client.
pub fn new_test(config: Configuration) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (rpc_builder, block_import, babe_link, commands_stream, mut telemetry),
    } = new_partial(&config)?;

    let net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

    let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_params: None,
            block_relay: None,
        })?;

    if config.offchain_worker.enabled {
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
                runtime_api_provider: client.clone(),
                is_validator: config.role.is_authority(),
                keystore: Some(keystore_container.keystore()),
                offchain_db: backend.offchain_storage(),
                transaction_pool: Some(OffchainTransactionPoolFactory::new(transaction_pool.clone())),
                network_provider: network.clone(),
                enable_http_requests: true,
                custom_extensions: |_| vec![],
            })
            .run(client.clone(), task_manager.spawn_handle())
            .boxed(),
        );
    }

    let role = config.role.clone();
    let prometheus_registry = config.prometheus_registry().cloned();

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
            block_import,
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
