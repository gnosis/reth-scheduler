// will be extracted to separate library. Maybe in util :)

use primitive_types::H256;

pub type BlockNumber = u64;

#[derive(Clone)]
pub struct Header {
    pub number: BlockNumber
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockId {
    Number(BlockNumber),
    Hash(H256),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetBlockHeaders {
    pub block_id: BlockId,
    pub max_headers: u64,
    pub skip: u64,
    pub reverse: bool,
}

impl GetBlockHeaders {
    pub fn new(block_id: BlockId, max_headers: u64, skip: u64, reverse: bool) -> GetBlockHeaders {
        GetBlockHeaders { block_id, max_headers, skip, reverse }
    }
}
