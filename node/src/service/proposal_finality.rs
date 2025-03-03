use std::sync::Arc;

use futures::StreamExt;
use sp_runtime::traits::Header;

use crate::rpc::ProposalFinalityRequest;

const LOG_TARGET: &str = "proposal_finality_service";

struct ProposalFinalityState<B>
where
    B: sp_runtime::traits::Block,
{
    proposed_blocks: Vec<B::Header>,
    latest_finalised_by_us: Option<B::Header>,
}

impl<B> ProposalFinalityState<B>
where
    B: sp_runtime::traits::Block,
{
    pub fn new() -> Self {
        Self {
            proposed_blocks: Vec::new(),
            latest_finalised_by_us: None,
        }
    }

    fn handle_imported_block(&mut self, header: Option<B::Header>) {
        match header {
            Some(header) => {
                log::debug!("Proposed block: {} ({})", header.number(), header.hash());
                self.proposed_blocks.push(header);
            }
            None => log::warn!(target: LOG_TARGET, "Proposed block receiver did not contain header"),
        }
    }

    fn handle_finalised_block(&mut self, header: Option<B::Header>) {
        if let None = header {
            log::warn!(target: LOG_TARGET, "Finalised block receiver did not contain header");
            return;
        }
        let header = header.unwrap();
        let block_number = header.number();
        let block_hash = header.hash();

        log::debug!(target: LOG_TARGET, "Finalised block: {} ({})", block_number, block_hash);

        let maybe_proposed_block = self
            .proposed_blocks
            .iter()
            .find(|&proposed_header| header.hash().eq(&proposed_header.hash()));

        match maybe_proposed_block {
            Some(_) => {
                log::info!(target: LOG_TARGET, "🍾 Proposed block was finalised: {} ({})", block_number, block_hash);
                self.proposed_blocks
                    .retain(|proposed_header| proposed_header.number() > block_number);
                self.latest_finalised_by_us = Some(header);
            }
            None => log::debug!(
              target: LOG_TARGET,
                "Finalised block {} ({}) was not authored by us",
                block_number,
                block_hash
            ),
        }
    }

    fn handle_request(&self, request: Option<ProposalFinalityRequest<B>>) {
        match request {
            Some(ProposalFinalityRequest::GetLatestFinalisedBlockByLocal(sender)) => {
                if let Err(_) = sender.send(self.latest_finalised_by_us.clone()) {
                    log::warn!(target: LOG_TARGET, "Failed to send latest finalised by us");
                }
            }
            _ => log::warn!(target: LOG_TARGET, "Request did not contain a message"),
        }
    }
}

pub struct ProposalFinality<B, C, R1, R2>
where
    B: sp_runtime::traits::Block,
{
    client: Arc<C>,
    import_block_receiver: R1,
    proposal_finality_request_receiver: R2,
    state: ProposalFinalityState<B>,
}

impl<B, C, R1, R2> ProposalFinality<B, C, R1, R2>
where
    B: sp_runtime::traits::Block,
    C: sc_client_api::BlockchainEvents<B>,
    R1: futures::stream::Stream<Item = B::Header> + Unpin + futures::stream::FusedStream,
    R2: futures::stream::Stream<Item = ProposalFinalityRequest<B>> + Unpin + futures::stream::FusedStream,
{
    pub fn new(client: Arc<C>, import_block_receiver: R1, request_receiver: R2) -> Self {
        Self {
            client,
            import_block_receiver,
            proposal_finality_request_receiver: request_receiver,
            state: ProposalFinalityState::new(),
        }
    }

    pub async fn start_proposal_log(self) {
        let mut import_stream = self.import_block_receiver;
        let mut proposal_finality_request_receiver = self.proposal_finality_request_receiver;
        let mut finality_stream = self.client.finality_notification_stream();
        let mut state = self.state;

        loop {
            futures::select! {
                header = import_stream.next() => {
                    state.handle_imported_block(header);
                },
                finality_notification = finality_stream.next() => {
                    let header = finality_notification.map(|n| n.header);
                    state.handle_finalised_block(header);
                },
                request = proposal_finality_request_receiver.next() => {
                    state.handle_request(request);
                }
            };
        }
    }
}
