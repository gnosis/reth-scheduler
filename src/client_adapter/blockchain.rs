// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::common_types::{BlockNumber, Header};

pub trait Blockchain {
    fn block_header(&self, number: BlockNumber) -> Option<Header>;
    fn block_body(&self);
    fn block_receipt(&self);
    fn best_block_header(&self) -> Option<Header>;

    
    fn import_block(&mut self, header: &Header);
    fn import_old_block(&self);
}

