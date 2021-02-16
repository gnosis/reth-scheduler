// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use super::rlp_en_de::{encode_block_headers, encode_block_bodies, encode_get_block_headers};
use crate::{
    client_adapter::Blockchain,
    common_types::{BlockId, GetBlockHeaders},
    scheduler::peer_organizer::{ErrorAct, PeerId, Task},
    scheduler::protocol::{ProtocolId, MessageId, EthMessageId}
};

pub struct BlockManager {
    chain: Arc<dyn Blockchain + Send + Sync>,
}



//ALL APIs
impl BlockManager {
    pub fn new(chain: Arc<dyn Blockchain + Send + Sync>) -> Self {
        BlockManager { chain }
    }

    pub fn api_request_block_headers(&self, peer: &PeerId) -> Result<Task,ErrorAct> {
        let request = GetBlockHeaders::new(BlockId::Number(10_000_000), 1, 0, false);
        let data = encode_get_block_headers(&request);
        Ok(Task::GetBlocks(*peer, data))
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

    pub fn api_new_block(&self) {}
    pub fn api_get_receipts(&self) {}
}
