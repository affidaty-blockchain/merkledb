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

//! Cryptography related types, constants, traits and functions. The functions
//! in this library are used for key generation, hashing, signing and signature
//! verification.
//!
//! The Crypto library makes it possible to potentially change the type of
//! cryptography applied in the system and add abstractions best
//! suited for Exonum.

#[cfg(feature = "sodiumoxide-crypto")]
mod sodiumoxide;

#[doc(inline)]
pub use crate::crypto::crypto_impl::HASH_SIZE;

use hex::{encode as encode_hex, FromHex, FromHexError};
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    Serialize, Serializer,
};

use std::{
    default::Default,
    fmt::{self, Debug},
    ops::{Index, Range, RangeFrom, RangeFull, RangeTo},
};

// A way to set an active cryptographic backend is to export it as `crypto_impl`.
#[cfg(feature = "sodiumoxide-crypto")]
use crate::crypto::sodiumoxide as crypto_impl;

#[macro_use]
mod macros;

/// The size to crop the string in debug messages.
const BYTES_IN_DEBUG: usize = 4;
/// The size of ellipsis in debug messages.
const BYTES_IN_ELLIPSIS: usize = 3;

fn write_short_hex(f: &mut impl fmt::Write, slice: &[u8]) -> fmt::Result {
    for byte in slice.iter().take(BYTES_IN_DEBUG) {
        write!(f, "{:02x}", byte)?;
    }
    if slice.len() > BYTES_IN_DEBUG {
        write!(f, "...")?;
    }
    Ok(())
}

/// Calculates a hash of a bytes slice.
///
/// Type of a hash depends on a chosen crypto backend (via `...-crypto` cargo feature).
///
/// # Examples
///
/// The example below calculates the hash of the indicated data.
///
/// ```
/// # merkledb::crypto::init();
/// let data = [1, 2, 3];
/// let hash = merkledb::crypto::hash(&data);
/// ```
pub fn hash(data: &[u8]) -> Hash {
    let dig = crypto_impl::hash(data);
    Hash(dig)
}

/// Initializes the cryptographic backend.
///
/// # Panics
///
/// Panics if backend initialization is failed.
///
/// # Examples
///
/// ```
/// merkledb::crypto::init();
/// ```
pub fn init() {
    if !crypto_impl::init() {
        panic!("Cryptographic library initialization failed.");
    }
}

/// This structure provides a possibility to calculate a hash digest
/// for a stream of data. Unlike the
/// [`Hash` structure](struct.Hash.html),
/// the given structure lets the code process several data chunks without
/// the need to copy them into a single buffer.
///
/// # Examples
///
/// The example below indicates the data the code is working with; runs the
/// system hash update as many times as required to process all the data chunks
/// and calculates the resulting hash of the system.
///
/// ```rust
/// use merkledb::crypto::HashStream;
///
/// let data: Vec<[u8; 5]> = vec![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]];
/// let mut hash_stream = HashStream::new();
/// for chunk in data {
///     hash_stream = hash_stream.update(&chunk);
/// }
/// let _ = hash_stream.hash();
/// ```
#[derive(Default)]
pub struct HashStream(crypto_impl::HashState);

impl Debug for HashStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hash_sha256 state")
    }
}

impl HashStream {
    /// Creates a new instance of `HashStream`.
    pub fn new() -> Self {
        Self(crypto_impl::HashState::new())
    }

    /// Processes a chunk of stream and returns a `HashStream` with the updated internal state.
    pub fn update(mut self, chunk: &[u8]) -> Self {
        self.0.update(chunk);
        self
    }

    /// Returns the resulting hash of the system calculated upon the commit
    /// of currently supplied data.
    pub fn hash(self) -> Hash {
        let dig = self.0.finalize();
        Hash(dig)
    }
}

implement_public_crypto_wrapper! { struct Hash, HASH_SIZE }

implement_serde! { Hash }
implement_index_traits! { Hash }

#[cfg(test)]
mod tests {
    use super::{fmt, hash, Hash, HashStream, Serialize, HASH_SIZE};

    use hex::FromHex;
    use serde::de::DeserializeOwned;

    use std::str::FromStr;

    #[test]
    fn to_from_hex_hash() {
        let original = hash(&[]);
        let from_hex = Hash::from_hex(original.to_hex()).unwrap();
        assert_eq!(original, from_hex);
    }

    #[test]
    fn to_from_string_hash() {
        let original = hash(&[]);
        let from_hex = Hash::from_str(&original.to_string()).unwrap();
        assert_eq!(original, from_hex);
    }

    #[test]
    fn zero_hash() {
        let hash = Hash::zero();
        assert_eq!(hash.as_ref(), [0; HASH_SIZE]);
    }

    #[test]
    fn serialize_deserialize_hash() {
        assert_serialize_deserialize(&Hash::new([207; HASH_SIZE]));
    }

    #[test]
    fn debug_format() {
        // Check zero padding.
        let hash = Hash::new([1; HASH_SIZE]);
        assert_eq!(format!("{:?}", &hash), "Hash(\"01010101...\")");

        // Check no padding.
        let hash = Hash::new([128; HASH_SIZE]);
        assert_eq!(format!("{:?}", &hash), "Hash(\"80808080...\")");
    }

    // Note that only public values have Display impl.
    #[test]
    fn display_format() {
        // Check zero padding.
        let hash = Hash::new([1; HASH_SIZE]);
        assert_eq!(
            format!("{}", &hash),
            "0101010101010101010101010101010101010101010101010101010101010101"
        );

        // Check no padding.
        let hash = Hash::new([128; HASH_SIZE]);
        assert_eq!(
            format!("{}", &hash),
            "8080808080808080808080808080808080808080808080808080808080808080"
        );
    }

    #[test]
    fn hash_streaming_zero() {
        let h1 = hash(&[]);
        let state = HashStream::new();
        let h2 = state.update(&[]).hash();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_streaming_chunks() {
        let data: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let h1 = hash(&data);
        let state = HashStream::new();
        let h2 = state.update(&data[..5]).update(&data[5..]).hash();
        assert_eq!(h1, h2);
    }

    fn assert_serialize_deserialize<T>(original_value: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + fmt::Debug,
    {
        let json = serde_json::to_string(original_value).unwrap();
        let deserialized_value: T = serde_json::from_str(&json).unwrap();
        assert_eq!(*original_value, deserialized_value);
    }
}
