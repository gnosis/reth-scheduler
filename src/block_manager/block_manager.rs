use std::sync::Arc;
use super::rlp_en_de::{encode_block_headers, encode_block_bodies};
use crate::{client_adapter::Blockchain, scheduler::peer_organizer::{ErrorAct, Task}};
use crate::scheduler::peer_organizer::PeerId;
use crate::scheduler::protocol::{ProtocolId, MessageId, EthMessageId};
// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0





pub struct BlockManager {
    // chain: Arc<dyn Blockchain>,
}



//ALL APIs
impl BlockManager {
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
