// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::common_types::{BlockNumber, BlockHeader};

pub trait Blockchain {
    fn block_header(&self, number: BlockNumber) -> Option<BlockHeader>;
    fn block_body(&self);
    fn block_receipt(&self);
    fn best_block_header(&self) -> Option<BlockHeader>;

    
    fn import_block(&mut self, header: &BlockHeader);
    fn import_old_block(&self);
}

