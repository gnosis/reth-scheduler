// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use super::blockchain::Blockchain;
use crate::common_types::{BlockNumber, BlockHeader};

pub struct HeadersInMemory {
    headers: HashMap<BlockNumber, BlockHeader>,
}

impl HeadersInMemory {
    pub fn new() -> Self {
        HeadersInMemory { headers: HashMap::new() }
    }
}

fn clone_option(from_header: Option<&BlockHeader>) -> Option<BlockHeader> {
    if let Some(header) = from_header { Some(header.clone()) } else { None }
}

impl Blockchain for HeadersInMemory {
    fn block_header(&self, number: BlockNumber) -> Option<BlockHeader> {
        clone_option(self.headers.get(&number))
    }

    fn block_body(&self) {
        unimplemented!()
    }

    fn block_receipt(&self) {
        unimplemented!()
    }

    fn best_block_header(&self) -> Option<BlockHeader> { unimplemented!() }

    fn import_block(&mut self, header: &BlockHeader) {
        self.headers.insert(header.number, header.clone());
    }

    fn import_old_block(&self) {
        unimplemented!()
    }
}