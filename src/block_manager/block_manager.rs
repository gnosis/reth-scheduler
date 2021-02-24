// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};
use super::rlp_en_de::{
    decode_block_headers,
    encode_block_headers,
    encode_block_bodies,
    encode_get_block_headers
};
use crate::{
    client_adapter::Blockchain,
    common_types::{BlockId, GetBlockHeaders},
    scheduler::PeerOrganizer,
    scheduler::peer_organizer::{ErrorAct, PeerId, Task},
    scheduler::protocol::{ProtocolId, MessageId, EthMessageId}
};

pub struct BlockManager {
    chain: Arc<Mutex<dyn Blockchain + Send + Sync>>,
    peer_organizer: Arc<Mutex<PeerOrganizer>>
}



//ALL APIs
impl BlockManager {
    pub fn new(
        chain: Arc<Mutex<dyn Blockchain + Send + Sync>>,
        peer_organizer: Arc<Mutex<PeerOrganizer>>
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(BlockManager { chain, peer_organizer }))
    }

    fn request_block_headers(&self, peer: &PeerId) -> Task {
        let request = GetBlockHeaders::new(BlockId::Number(10_000_000), 100, 0, false);
        let data = encode_get_block_headers(&request);
        Task::GetBlocks(*peer, data)
    }

    pub fn sync(&self) {
        // TODO implement sync instead of this test request
        if self.is_syncing() {
            let mut org = self.peer_organizer.lock().unwrap();
            if let Some(ref peer) = org.random_peer() {
                info!("Sync: Scheduling task.");
                org.schedule(self.request_block_headers(peer));
            } else {
                warn!("No peer for syncing found.");
            }
        }
    }

    pub fn is_syncing(&self) -> bool {
        true // TODO
    }

    pub fn api_new_block_hashes(&self) -> Result<Task,ErrorAct> {
        //Task::InsertPeer()
        ErrorAct::new_kick("TEST".into()).map(|_| Task::None)
    }

    pub fn api_get_block_headers(&self, peer: &PeerId) -> Result<Task, ErrorAct> {
        Ok(Task::Responde(
            *peer,
            ProtocolId::Eth,
            MessageId::Eth(EthMessageId::BlockHeaders),
            encode_block_headers(),
        ))
    }

    pub fn api_get_block_bodies(&self, peer: &PeerId) -> Result<Task, ErrorAct> {
        Ok(Task::Responde(
            *peer,
            ProtocolId::Eth,
            MessageId::Eth(EthMessageId::BlockBodies),
            encode_block_bodies(),
        ))
    }

    pub fn process_block_headers(&self, data: &[u8]) {
        let decoded = decode_block_headers(&data);
        match decoded {
            Ok(headers) => {
                info!("Decoded block headers: {:?}", headers);
                for ref header in headers {
                    self.chain.lock().unwrap().import_block(header);
                }
            },
            Err(err) => error!("Could not decode block header: {}", err),
        }
    }

    pub fn api_new_block(&self) {}
    pub fn api_get_receipts(&self) {}
}
