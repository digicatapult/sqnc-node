use async_trait::async_trait;
use futures::channel::oneshot;
use jsonrpsee::proc_macros::rpc;
use sc_network_sync::types::SyncStatusProvider;
pub use sc_rpc_api::system::Error;
use sc_utils::mpsc::{TracingUnboundedReceiver, TracingUnboundedSender};
use serde::{Deserialize, Serialize};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, Header};
use std::sync::Arc;

const RPC_INTERNAL_ERROR: &str = "Error getting extended sync state";

pub enum ProposalFinalityRequest<B>
where
    B: BlockT,
{
    GetLatestFinalisedBlockByLocal(oneshot::Sender<Option<B::Header>>),
}

type ProposalFinalityRequestHandler<B> = TracingUnboundedReceiver<ProposalFinalityRequest<B>>;

#[derive(Clone)]
pub struct SqncDeps<B, SS>
where
    B: BlockT,
{
    request: TracingUnboundedSender<ProposalFinalityRequest<B>>,
    sync_service: Arc<SS>,
}

impl<B, SS> SqncDeps<B, SS>
where
    B: BlockT,
{
    pub fn new(sync_service: Arc<SS>) -> (Self, ProposalFinalityRequestHandler<B>) {
        let (sender, receiver) =
            sc_utils::mpsc::tracing_unbounded::<ProposalFinalityRequest<B>>("ProposalFinalityRequestHandler", 128);

        (
            Self {
                request: sender,
                sync_service,
            },
            receiver,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedSyncState<Number> {
    /// Height of the block at which syncing started.
    pub starting_block: Number,
    /// Height of the current best block of the node.
    pub current_block: Number,
    /// Height of the highest block in the network.
    pub highest_block: Number,
    /// Height of the highest finalised block that this node authored in the network.
    pub last_authored_finalised_block: Number,
}

#[rpc(client, server)]
pub trait SqncApi<Number> {
    #[method(name = "sqnc_syncStateExtended")]
    async fn sqnc_sync_state_extended(&self) -> Result<ExtendedSyncState<Number>, Error>;
}

pub struct Sqnc<C, SS, Block>
where
    Block: BlockT,
{
    client: Arc<C>,
    sync_service: Arc<SS>,
    request: TracingUnboundedSender<ProposalFinalityRequest<Block>>,
    starting_block: <Block::Header as sp_runtime::traits::Header>::Number,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, SS, Block> Sqnc<C, SS, Block>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
{
    pub fn new(client: Arc<C>, deps: SqncDeps<Block, SS>) -> Self {
        let client = client.clone();
        let info = client.info();
        Self {
            client,
            sync_service: deps.sync_service,
            request: deps.request,
            starting_block: info.best_number,
            _marker: Default::default(),
        }
    }
}

#[async_trait]
impl<C, SS, Block> SqncApiServer<<Block::Header as sp_runtime::traits::Header>::Number> for Sqnc<C, SS, Block>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
    SS: SyncStatusProvider<Block> + Send + Sync + Clone + 'static,
{
    async fn sqnc_sync_state_extended(
        &self,
    ) -> Result<ExtendedSyncState<<Block::Header as sp_runtime::traits::Header>::Number>, Error> {
        let client = self.client.clone();
        let info = client.info();
        let (sender, receiver) = oneshot::channel::<Option<Block::Header>>();

        if let Err(err) = self
            .request
            .unbounded_send(ProposalFinalityRequest::GetLatestFinalisedBlockByLocal(sender))
        {
            log::warn!("Failed to send proposal finality request: {:?}", err);
            return Err(Error::Internal(RPC_INTERNAL_ERROR.into()));
        }

        let (best_seen_block, last_authored_finalised_block) = futures::join!(self.sync_service.status(), receiver);

        let best_seen_block = best_seen_block.map(|status| status.best_seen_block);
        let last_authored_finalised_block = last_authored_finalised_block
            .map(|header| header.map(|header| header.number().clone()).unwrap_or_default());

        match (best_seen_block, last_authored_finalised_block) {
            (Ok(best_seen_block), Ok(last_authored_finalised_block)) => Ok(ExtendedSyncState {
                starting_block: self.starting_block,
                current_block: info.best_number,
                highest_block: best_seen_block.unwrap_or(info.best_number),
                last_authored_finalised_block,
            }),
            (_, Err(err)) => {
                log::warn!("Error getting last authored finalised block: {:?}", err);
                Err(Error::Internal(RPC_INTERNAL_ERROR.into()))
            }
            _ => {
                log::warn!("Unknown error getting sync state");
                Err(Error::Internal(RPC_INTERNAL_ERROR.into()))
            }
        }
    }
}
