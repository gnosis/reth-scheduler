// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use primitive_types::{H160, H256};
use rlp::{EMPTY_LIST_RLP, RlpStream, Rlp, DecoderError};
use crate::common_types::{BlockHeader, BlockId, BlockNumber, GetBlockHeaders};

pub fn encode_get_block_headers(request: &GetBlockHeaders) -> Vec<u8> {
    let mut stream = RlpStream::new_list(4);

    match request.block_id {
        BlockId::Number(number) => stream.append(&number),
        BlockId::Hash(hash) => stream.append(&hash),
    };

    stream
        .append(&request.max_headers)
        .append(&request.skip);

    if request.reverse {
        stream.append(&1u8);
    } else {
        stream.append_empty_data();
    }

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

pub fn decode_block_headers(data: &[u8]) -> Result<Vec<BlockHeader>, DecoderError> {
    let encoded_headers = Rlp::new(data);
    let mut decoded_headers = vec![];

    for header in encoded_headers.iter() {
        decoded_headers.push(BlockHeader {
            parent_hash: H256::from_slice(header.at(0)?.data()?),
            ommers_hash: H256::from_slice(header.at(1)?.data()?),
            beneficiary_address: H160::from_slice(header.at(2)?.data()?),
            state_root: H256::from_slice(header.at(3)?.data()?),
            transactions_root: H256::from_slice(header.at(4)?.data()?),
            receipts_root: H256::from_slice(header.at(5)?.data()?),
            logs_bloom: header.val_at(6)?,
            difficulty: header.val_at(7)?,
            number: header.val_at(8)?,
            gas_limit: header.val_at(9)?,
            gas_used: header.val_at(10)?,
            timestamp: header.val_at(11)?,
            extra_data: header.val_at(12)?,
            mix_hash: H256::from_slice(header.at(13)?.data()?),
            nonce: header.val_at(14)?,
        });
    }

    Ok(decoded_headers)
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
        let request = GetBlockHeaders::new(BlockId::Number(4096), 1u64, 10, false);
        let encoded = encode_get_block_headers(&request);
        assert_eq!(encoded, [0xc6, 0x82, 0x10, 0x00, 0x01, 0x0a, 0x80]);
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

    #[test]
    fn test_decode_block_header() {
        let header = vec![249, 2, 26, 249, 2, 23, 160, 150, 107, 246, 132, 157, 169, 47, 242, 160, 227, 219, 154, 55, 31, 91, 159, 7, 221, 96, 1, 226, 119, 10, 66, 105, 165, 193, 52, 241, 191, 156, 76, 160, 29, 204, 77, 232, 222, 199, 93, 122, 171, 133, 181, 103, 182, 204, 212, 26, 211, 18, 69, 27, 148, 138, 116, 19, 240, 161, 66, 253, 64, 212, 147, 71, 148, 234, 103, 79, 221, 231, 20, 253, 151, 157, 227, 237, 240, 245, 106, 169, 113, 107, 137, 142, 200, 160, 116, 71, 126, 170, 190, 206, 107, 206, 0, 195, 70, 220, 18, 39, 91, 46, 215, 78, 201, 214, 199, 88, 196, 2, 60, 32, 64, 186, 14, 114, 224, 93, 160, 20, 230, 203, 133, 194, 42, 226, 253, 119, 79, 24, 204, 214, 103, 211, 254, 150, 125, 110, 57, 235, 197, 34, 70, 131, 127, 35, 15, 2, 248, 69, 221, 160, 195, 99, 51, 64, 229, 167, 39, 232, 170, 29, 41, 163, 175, 206, 149, 210, 126, 85, 90, 49, 167, 176, 151, 41, 103, 47, 55, 108, 47, 63, 78, 46, 185, 1, 0, 136, 100, 128, 192, 2, 0, 98, 13, 132, 24, 13, 4, 112, 0, 12, 80, 48, 129, 22, 0, 68, 208, 80, 21, 128, 128, 3, 116, 1, 16, 112, 96, 18, 0, 64, 16, 82, 129, 16, 1, 0, 16, 69, 0, 65, 66, 3, 4, 10, 32, 128, 3, 72, 20, 32, 6, 16, 218, 18, 8, 166, 56, 209, 110, 68, 12, 2, 72, 128, 128, 3, 1, 225, 0, 76, 43, 2, 40, 80, 96, 32, 0, 8, 76, 50, 73, 160, 192, 132, 86, 156, 144, 194, 0, 32, 1, 88, 98, 65, 4, 30, 128, 4, 3, 90, 68, 0, 160, 16, 9, 56, 0, 30, 4, 17, 128, 8, 49, 128, 176, 52, 6, 97, 55, 32, 96, 64, 20, 40, 192, 32, 8, 116, 16, 64, 43, 148, 132, 2, 129, 0, 4, 148, 129, 144, 12, 8, 3, 72, 100, 49, 70, 136, 208, 1, 84, 140, 48, 0, 130, 142, 84, 34, 132, 24, 2, 128, 0, 100, 2, 162, 138, 2, 100, 218, 0, 172, 34, 48, 4, 0, 98, 9, 96, 152, 50, 6, 96, 50, 0, 8, 64, 64, 18, 42, 71, 57, 8, 5, 1, 37, 21, 66, 8, 32, 32, 164, 8, 124, 0, 2, 129, 192, 136, 0, 137, 141, 9, 0, 2, 64, 71, 56, 0, 0, 18, 112, 56, 9, 142, 9, 8, 1, 8, 0, 0, 66, 144, 200, 66, 1, 102, 16, 64, 32, 2, 1, 192, 0, 75, 132, 144, 173, 88, 136, 4, 135, 8, 121, 44, 111, 71, 247, 15, 131, 152, 150, 128, 131, 152, 112, 92, 131, 152, 36, 179, 132, 94, 176, 23, 5, 150, 80, 80, 89, 69, 45, 101, 116, 104, 101, 114, 109, 105, 110, 101, 45, 97, 115, 105, 97, 49, 45, 49, 160, 55, 253, 227, 17, 117, 254, 24, 3, 70, 68, 77, 21, 180, 223, 198, 169, 218, 59, 43, 65, 238, 34, 152, 206, 236, 202, 248, 136, 178, 212, 93, 244, 136, 47, 105, 35, 248, 4, 38, 241, 87];
        let decoded = decode_block_headers(&header);
        assert!(decoded.is_ok(), "Error: {}", decoded.err().unwrap());
    }
}