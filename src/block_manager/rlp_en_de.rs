// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use rlp::{EMPTY_LIST_RLP, RlpStream, Rlp, DecoderError};
use crate::common_types::{BlockId, GetBlockHeaders, BlockNumber};
use primitive_types::H256;

pub fn encode_get_block_headers(request: &GetBlockHeaders) -> Vec<u8> {
    let mut stream = RlpStream::new_list(4);
    match request.block_id {
        BlockId::Number(number) => stream.append(&number),
        BlockId::Hash(hash) => stream.append(&hash),
    };
    stream
        .append(&request.max_headers)
        .append(&request.skip)
        .append(&request.reverse);
    stream.out()
}

pub fn decode_get_block_headers(data: &Vec<u8>) -> Result<GetBlockHeaders, DecoderError> {
    let rlp = Rlp::new(data);

    let block_id_rlp = rlp.at(0)?;
    let block_id = match block_id_rlp.size() {
        32 => BlockId::Hash(H256::from_slice(block_id_rlp.data()?)),
        _ => BlockId::Number(block_id_rlp.as_val::<BlockNumber>()?)
    };

    let max_headers = rlp.at(1)?.as_val::<u64>()?;
    let skip = rlp.at(2)?.as_val::<u64>()?;
    let reverse = rlp.at(3)?.as_val::<bool>()?;

    Ok(GetBlockHeaders::new(block_id, max_headers, skip, reverse))
}

pub fn encode_block_headers() -> Vec<u8> {
    EMPTY_LIST_RLP.to_vec()
}

pub fn encode_block_bodies() -> Vec<u8> {
    EMPTY_LIST_RLP.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_get_block_headers() {
        let request = GetBlockHeaders::new(BlockId::Number(1024), 128u64, 0u64, true);
        let encoded = encode_get_block_headers(&request);
        assert_eq!(encoded, [0xc7, 0x82, 0x04, 0x00, 0x81, 0x80, 0x80, 0x01]);
    }

    #[test]
    fn test_decode_get_block_headers_with_hash_as_id() {
        let data: Vec<u8> = vec![
            228, 160, 229, 229, 95, 194, 152, 198, 135, 130, 236, 183, 27, 149, 246, 32, 35, 98,
            190, 1, 185, 199, 112, 109, 151, 50, 226, 8, 58, 130, 147, 155, 184, 73, 1, 128, 128
        ];
        let expected_hash = BlockId::Hash(H256::from_slice(&data[2..34]));
        let expected = GetBlockHeaders::new(expected_hash, 1, 0, false);
        let decoded = decode_get_block_headers(&data).unwrap();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_get_block_headers_roundtrip() {
        let test_cases = vec![
            GetBlockHeaders::new(BlockId::Number(2283397), 100, 0, false),
            GetBlockHeaders::new(BlockId::Number(2700031), 1024, 8, true),
            GetBlockHeaders::new(BlockId::Hash(H256::repeat_byte(0x22)), 10, 1, false),
        ];
        for test_case in test_cases {
            let encoded = encode_get_block_headers(&test_case.clone());
            let decoded = decode_get_block_headers(&encoded).unwrap();
            assert_eq!(test_case, decoded);
        }
    }
}