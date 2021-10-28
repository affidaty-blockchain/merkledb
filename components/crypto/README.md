# Cryptography primitives for MerkleDB

`merkle-crypto` provides a high-level API for work with various cryptography
tasks.

Capabilities of `merkledb-crypto` include:

- Calculating the hash of data;
- Generating key pairs for work with digital signatures;
- Creating and verifying of digital signatures.

The main backend for `exonum-crypto` is `sodiumoxide`, and the used algorithms are:

- SHA-256 for hashing.
- Ed25519 for digital signatures.

Consult [the crate docs](https://docs.rs/exonum-crypto) for more details.

## Examples

Signing data and verifying the signature:

```rust
merkledb_crypto::init();
let (public_key, secret_key) = merkledb_crypto::gen_keypair();
let data = [1, 2, 3];
let signature = merkledb_crypto::sign(&data, &secret_key);
assert!(merkledb_crypto::verify(&signature, &data, &public_key));
```

Hashing fixed amount of data:

```rust
merkledb_crypto::init();
let data = [1, 2, 3];
let hash = merkledb_crypto::hash(&data);
```

Hashing data by chunks:

```rust
use merkledb_crypto::HashStream;

merkledb_crypto::init();
let data: Vec<[u8; 5]> = vec![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]];
let mut hash_stream = HashStream::new();
for chunk in data {
    hash_stream = hash_stream.update(&chunk);
}
let _ = hash_stream.hash();
```

## Usage

Include `merkledb-crypto` as a dependency in your `Cargo.toml`:

## License

`merkledb-crypto` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
