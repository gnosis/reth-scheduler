// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use super::blockchain::Blockchain;
use crate::common_types::{BlockNumber, Header};

pub struct HeadersInMemory {
    headers: Vec<Header>,
}

impl HeadersInMemory {
    pub(crate) fn new() -> Self {
        HeadersInMemory { headers: vec![] }
    }
}

fn clone_option(from_header: Option<&Header>) -> Option<Header> {
    if let Some(header) = from_header { Some(header.clone()) } else { None }
}

impl Blockchain for HeadersInMemory {
    fn block_header(&self, number: BlockNumber) -> Option<Header> {
        clone_option(self.headers.get(number as usize))
    }

    fn block_body(&self) {
        unimplemented!()
    }

    fn block_receipt(&self) {
        unimplemented!()
    }

    fn best_block_header(&self) -> Option<Header> {
        clone_option(self.headers.last())
    }

    fn import_block(&mut self, header: &Header) {
        if header.number as usize == self.headers.len() {
            self.headers.push(header.clone());
        }
    }

    fn import_old_block(&self) {
        unimplemented!()
    }
}