// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// A test that does a slice to slice copy where the underlying storage of the target slice is
// not the same length as the source slice.

pub fn to_bytes(r: [u8; 4]) -> [u8; 8] {
    let mut signature_bytes: [u8; 8] = [0u8; 8];

    signature_bytes[..4].copy_from_slice(&r[..]);
    signature_bytes
}

pub fn main() {}
