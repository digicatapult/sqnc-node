use log::warn;
use sc_consensus::{BlockCheckParams, BlockImport, BlockImportParams};
use sc_utils::mpsc::{TracingUnboundedReceiver, TracingUnboundedSender};
use sp_runtime::traits::Block as BlockT;

const LOG_TARGET: &str = "observable_block_import";

pub struct ObservableBlockImport<Header, BI> {
    base_import: BI,
    observer: TracingUnboundedSender<Header>,
}

impl<Header, BI> ObservableBlockImport<Header, BI> {
    pub fn new(base_import: BI) -> (Self, TracingUnboundedReceiver<Header>) {
        let (sender, receiver) = sc_utils::mpsc::tracing_unbounded::<Header>("ImportProposedBlock", 100);

        (
            Self {
                base_import,
                observer: sender,
            },
            receiver,
        )
    }
}

#[async_trait::async_trait]
impl<Block, BI> BlockImport<Block> for ObservableBlockImport<Block::Header, BI>
where
    Block: BlockT,
    BI: BlockImport<Block, Error = sp_consensus::Error> + Send + Sync + 'static,
{
    type Error = sp_consensus::Error;

    async fn check_block(&self, block: BlockCheckParams<Block>) -> Result<sc_consensus::ImportResult, Self::Error> {
        self.base_import.check_block(block).await
    }

    async fn import_block(&self, block: BlockImportParams<Block>) -> Result<sc_consensus::ImportResult, Self::Error> {
        let import_header = block.post_header();
        let result = self.base_import.import_block(block).await;

        match &result {
            Ok(sc_consensus::ImportResult::Imported(_)) => {
                self.observer.unbounded_send(import_header).unwrap_or_else(
                    |e| warn!(target: LOG_TARGET, "😬 Error reporting new block import to observer: {}", e),
                );
            }
            _ => {}
        };

        result
    }
}
