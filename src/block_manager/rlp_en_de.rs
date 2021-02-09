// Copyright 2020 Gnosis Ltd.
// SPDX-License-Identifier: Apache-2.0

use rlp::EMPTY_LIST_RLP;

pub fn encode_block_headers() -> Vec<u8> {
    EMPTY_LIST_RLP.to_vec()
}

pub fn encode_block_bodies() -> Vec<u8> {
    EMPTY_LIST_RLP.to_vec()
}