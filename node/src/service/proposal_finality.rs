use std::sync::Arc;

use futures::StreamExt;
use sp_runtime::traits::Header;

use crate::rpc::ProposalFinalityRequest;

const LOG_TARGET: &str = "proposal_finality_service";

struct ProposalFinalityState<B>
where
    B: sp_runtime::traits::Block,
{
    /// list of blocks proposed by the node
    proposed_blocks: Vec<B::Header>,
    /// the latest finalised block observed. Needed because manual seal imports blocks after marking it as finalised
    latest_finalised: Option<B::Header>,
    // latest block finalised by us. Updated either when importing a block that's last been finalised or when a finalised block is observed that was authored by us
    latest_finalised_by_us: Option<B::Header>,
}

impl<B> ProposalFinalityState<B>
where
    B: sp_runtime::traits::Block,
{
    pub fn new() -> Self {
        Self {
            proposed_blocks: Vec::new(),
            latest_finalised: None,
            latest_finalised_by_us: None,
        }
    }

    fn handle_imported_block(&mut self, header: Option<B::Header>) {
        match header {
            Some(header) => {
                let block_number = header.number();
                let block_hash = header.hash();
                log::debug!("🏗 Proposed block: {} ({})", block_number, block_hash);

                self.proposed_blocks.push(header.clone());

                let last_finalised_block = self.latest_finalised.as_ref();
                if Some(header.hash()) == last_finalised_block.map(|h| h.hash()) {
                    self.handle_finalised_block_by_us(header);
                }
            }
            None => log::warn!(target: LOG_TARGET, "😬 Proposed block receiver did not contain header"),
        }
    }

    fn handle_finalised_block(&mut self, header: Option<B::Header>) {
        if let None = header {
            log::warn!(target: LOG_TARGET, "😬 Finalised block receiver did not contain header");
            return;
        }
        let header = header.unwrap();
        self.latest_finalised = Some(header.clone());

        let block_number = header.number();
        let block_hash = header.hash();

        log::debug!(target: LOG_TARGET, "🏗 Finalised block: {} ({})", block_number, block_hash);

        let maybe_proposed_block = self
            .proposed_blocks
            .iter()
            .find(|&proposed_header| block_hash.eq(&proposed_header.hash()));

        match maybe_proposed_block {
            Some(_) => {
                self.handle_finalised_block_by_us(header);
            }
            None => {
                log::debug!(
                  target: LOG_TARGET,
                     "🏗 Finalised block {} ({}) was not authored by us",
                    block_number,
                    block_hash
                );
                self.proposed_blocks
                    .retain(|proposed_header| proposed_header.number() > block_number);
            }
        }
    }

    fn handle_finalised_block_by_us(&mut self, header: B::Header) {
        let block_number = header.number();
        let block_hash = header.hash();

        log::info!(target: LOG_TARGET, "🍾 Proposed block was finalised: {} ({})", block_number, block_hash);
        self.proposed_blocks
            .retain(|proposed_header| proposed_header.number() > block_number);
        self.latest_finalised_by_us = Some(header.clone());
    }

    fn handle_request(&self, request: Option<ProposalFinalityRequest<B>>) {
        match request {
            Some(ProposalFinalityRequest::GetLatestFinalisedBlockByLocal(sender)) => {
                if let Err(_) = sender.send(self.latest_finalised_by_us.clone()) {
                    log::warn!(target: LOG_TARGET, "😬 Failed to send latest finalised by us");
                }
            }
            _ => log::warn!(target: LOG_TARGET, "😬 Request did not contain a message"),
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

#[cfg(test)]
mod tests {
    use sp_runtime::Digest;

    use super::*;

    #[test]
    fn state_new() {
        let state = ProposalFinalityState::<sqnc_runtime::Block>::new();

        assert_eq!(state.proposed_blocks, Vec::new());
        assert_eq!(state.latest_finalised_by_us, None);
    }

    #[test]
    fn state_handle_imported_block_success() {
        let hash = sqnc_runtime::Hash::random();
        let header: sqnc_runtime::Header = Header::new(42u32, hash, hash, hash, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header.clone()));

        assert_eq!(state.proposed_blocks, vec![header]);
        assert_eq!(state.latest_finalised_by_us, None);
    }

    #[test]
    fn state_handle_imported_block_none() {
        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(None);

        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, None);
        assert_eq!(state.latest_finalised_by_us, None);
    }

    #[test]
    fn state_handle_imported_block_after_finalised() {
        let hash1 = sqnc_runtime::Hash::random();
        let hash2 = sqnc_runtime::Hash::random();
        let header1: sqnc_runtime::Header = Header::new(1u32, hash1, hash1, hash1, Digest { logs: vec![] });
        let header2: sqnc_runtime::Header = Header::new(2u32, hash2, hash2, hash2, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header1.clone()));
        state.handle_finalised_block(Some(header2.clone()));

        state.handle_imported_block(Some(header2.clone()));

        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, Some(header2.clone()));
        assert_eq!(state.latest_finalised_by_us, Some(header2.clone()));
    }

    #[test]
    fn state_handle_finalised_block_by_us_1() {
        let hash1 = sqnc_runtime::Hash::random();
        let hash2 = sqnc_runtime::Hash::random();
        let header1: sqnc_runtime::Header = Header::new(1u32, hash1, hash1, hash1, Digest { logs: vec![] });
        let header2: sqnc_runtime::Header = Header::new(2u32, hash2, hash2, hash2, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header1.clone()));
        state.handle_imported_block(Some(header2.clone()));

        state.handle_finalised_block(Some(header1.clone()));

        assert_eq!(state.proposed_blocks, vec![header2]);
        assert_eq!(state.latest_finalised, Some(header1.clone()));
        assert_eq!(state.latest_finalised_by_us, Some(header1));
    }

    #[test]
    fn state_handle_finalised_block_by_us_2() {
        let hash1 = sqnc_runtime::Hash::random();
        let hash2 = sqnc_runtime::Hash::random();
        let header1: sqnc_runtime::Header = Header::new(1u32, hash1, hash1, hash1, Digest { logs: vec![] });
        let header2: sqnc_runtime::Header = Header::new(2u32, hash2, hash2, hash2, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header1.clone()));
        state.handle_imported_block(Some(header2.clone()));

        state.handle_finalised_block(Some(header2.clone()));

        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, Some(header2.clone()));
        assert_eq!(state.latest_finalised_by_us, Some(header2));
    }

    #[test]
    fn state_handle_finalised_block_by_not_us() {
        let hash1 = sqnc_runtime::Hash::random();
        let hash2 = sqnc_runtime::Hash::random();
        let header1: sqnc_runtime::Header = Header::new(1u32, hash1, hash1, hash1, Digest { logs: vec![] });
        let header2: sqnc_runtime::Header = Header::new(2u32, hash2, hash2, hash2, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header1.clone()));
        state.handle_finalised_block(Some(header2.clone()));

        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, Some(header2.clone()));
        assert_eq!(state.latest_finalised_by_us, None);
    }

    #[test]
    fn state_handle_finalised_block_none() {
        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_finalised_block(None);

        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, None);
        assert_eq!(state.latest_finalised_by_us, None);
    }

    #[tokio::test]
    async fn state_handle_request_get_latest_finalised_block_by_local() {
        let hash = sqnc_runtime::Hash::random();
        let header: sqnc_runtime::Header = Header::new(2u32, hash, hash, hash, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header.clone()));
        state.handle_finalised_block(Some(header.clone()));

        let (s, r) = futures::channel::oneshot::channel();
        state.handle_request(Some(ProposalFinalityRequest::GetLatestFinalisedBlockByLocal(s)));

        let result = r.await.unwrap();
        assert_eq!(result, Some(header.clone()));
        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, Some(header.clone()));
        assert_eq!(state.latest_finalised_by_us, Some(header));
    }

    #[tokio::test]
    async fn state_handle_request_get_latest_finalised_block_by_local_none() {
        let hash = sqnc_runtime::Hash::random();
        let header: sqnc_runtime::Header = Header::new(2u32, hash, hash, hash, Digest { logs: vec![] });

        let mut state = ProposalFinalityState::<sqnc_runtime::Block>::new();
        state.handle_imported_block(Some(header.clone()));
        state.handle_finalised_block(Some(header.clone()));

        state.handle_request(None);
        assert_eq!(state.proposed_blocks, vec![]);
        assert_eq!(state.latest_finalised, Some(header.clone()));
        assert_eq!(state.latest_finalised_by_us, Some(header));
    }
}
