// Copyright 2020 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module implements cryptographic backend based
//! on [Sodium library](https://github.com/jedisct1/libsodium)
//! through [sodiumoxide rust bindings](https://github.com/dnaq/sodiumoxide).
//! The constants in this module are imported from Sodium.
//!
//! The SHA-256 function applied in this backend splits the input data into blocks
//! and runs each block through a cycle of 64 iterations. The result of the
//! function is a cryptographic hash 256 bits or 32 bytes in length. This
//! hash can later be used to verify the integrity of data without accessing the
//! data itself.
//!
//! This backend also makes use of Ed25519 keys. Ed25519 is a signature system that ensures
//! fast signing and key generation, as well as security and collision
//! resilience.

// spell-checker:ignore DIGESTBYTES, PUBLICKEYBYTES, SECRETKEYBYTES, SEEDBYTES, SIGNATUREBYTES

//use exonum_sodiumoxide as sodiumoxide;
pub use sodiumoxide::crypto::hash::sha256;

/// Digest type for sodiumoxide-based implementation.
//pub use sha256::Digest as Hash;
pub use self::sha256::Digest as Hash;

/// Contains the state for multi-part (streaming) hash computations
/// for sodiumoxide-based implementation.
//pub use sha256::State as HashState;
pub use self::sha256::State as HashState;

/// Number of bytes in a `Hash`.
//pub const HASH_SIZE: usize = sha256::DIGESTBYTES;
pub const HASH_SIZE: usize = self::sha256::DIGESTBYTES;

/// Initializes the sodium library and automatically selects faster versions
/// of the primitives, if possible.
pub fn init() -> bool {
    sodiumoxide::init().is_ok()
}

/// Calculates hash of a bytes slice.
pub fn hash(data: &[u8]) -> Hash {
    sha256::hash(data)
}
