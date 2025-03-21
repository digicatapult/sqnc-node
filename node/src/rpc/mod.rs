use std::sync::Arc;

use futures::channel::mpsc::Sender;
use jsonrpsee::RpcModule;
use sc_consensus_babe::BabeWorkerHandle;
use sc_consensus_manual_seal::{rpc::ManualSeal, rpc::ManualSealApiServer, EngineCommand};
use sc_network_sync::types::SyncStatusProvider;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::KeystorePtr;
use sqnc_runtime::{opaque::Block, AccountId, Balance, Hash, Nonce};

mod sqnc;
pub use sqnc::*;

/// Extra dependencies for BABE.
pub struct BabeDeps {
    /// A handle to the BABE worker for issuing requests.
    pub babe_worker_handle: BabeWorkerHandle<Block>,
    /// The keystore that manages the keys of the node.
    pub keystore: KeystorePtr,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, SS> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// The SelectChain Strategy
    pub select_chain: SC,
    /// BABE specific dependencies.
    pub babe: BabeDeps,
    /// Sqnc specific dependencies
    pub sqnc: SqncDeps<Block, SS>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, SS>(
    deps: FullDeps<C, P, SC, SS>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BabeApi<Block>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
    SC: SelectChain<Block> + 'static,
    SS: SyncStatusProvider<Block> + Send + Sync + Clone + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use sc_consensus_babe_rpc::{Babe, BabeApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        select_chain,
        babe,
        ..
    } = deps;

    let BabeDeps {
        keystore,
        babe_worker_handle,
    } = babe;

    module.merge(System::new(client.clone(), pool).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    module.merge(Babe::new(client.clone(), babe_worker_handle, keystore, select_chain).into_rpc())?;
    module.merge(Sqnc::new(client.clone(), deps.sqnc).into_rpc())?;

    Ok(module)
}

/// Full client dependencies.
pub struct TestDeps<C, P, SS> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// A command stream to send authoring commands to manual seal consensus engine
    pub command_sink: Sender<EngineCommand<Hash>>,
    /// Sqnc specific dependencies
    pub sqnc: SqncDeps<Block, SS>,
}

// Instantiate all full RPC extensions.
pub fn create_test<C, P, SS>(
    deps: TestDeps<C, P, SS>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
    SS: SyncStatusProvider<Block> + Send + Sync + Clone + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let TestDeps {
        client,
        pool,
        command_sink,
        sqnc,
    } = deps;

    module.merge(System::new(client.clone(), pool).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    module.merge(ManualSeal::new(command_sink).into_rpc())?;
    module.merge(Sqnc::new(client.clone(), sqnc).into_rpc())?;

    Ok(module)
}
