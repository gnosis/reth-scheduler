use std::sync::Arc;
use crate::{client_adapter::Blockchain, scheduler::peer_organizer::{ErrorAct, Task}};
// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0





pub struct BlockManager {
    chain: Arc<dyn Blockchain>,
}



//ALL APIs
impl BlockManager {
    pub fn api_new_block_hashes(&self) -> Result<Task,ErrorAct> {
        //Task::InsertPeer()
        ErrorAct::new_kick("TEST".into()).map(|_| Task::None)
    }
    pub fn api_get_block_headers(&self) {}
    pub fn api_get_block_bodies(&self) {}
    pub fn api_new_block(&self) {}
    pub fn api_get_receipts(&self) {}
}