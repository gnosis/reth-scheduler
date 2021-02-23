// will be extracted to separate library. Maybe in util :)

use primitive_types::{H160, H256};

pub type BlockNumber = u64;

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

#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary_address: H160,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: Vec<u8>,
    pub difficulty: u64,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: Vec<u8>,
    pub mix_hash: H256,
    pub nonce: u64,
}
