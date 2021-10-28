# Procedural macros for MerkleDB

This crate provides several procedural macros for MerkleDB.

Overview of presented macros:

- `BinaryValue`: derive macro for `BinaryValue` trait of MerkleDB.
  Depending on codec, the implementation may use `ProtobufConvert`
  trait as base (default), or `serde` traits using `bincode`.
- `ObjectHash`: derive macro for `ObjectHash` trait of MerkleDB.
  It can be used for any type that implements `BinaryValue` trait.
- `FromAccess`: derive macro for `FromAccess` trait for schemas of
  MerkleDB indexes.
- `ServiceDispatcher`: derive macro for generating dispatching mechanisms
  of Rust Exonum services.
- `ServiceFactory`: derive macro for generating factory mechanisms
  of Rust Exonum services.
- `merkledb_interface`: attribute macro for transforming trait into interface
  of Rust Exonum service.
- `ExecutionFail`: derive macro similar to `failure::Fail`, implementing
  `ExecutionFail` trait for an enum.
- `RequireArtifact`: derive macro for `RequireArtifact` trait.

Consult [the crate docs](https://docs.rs/merkledb-derive) for more details.

## Usage

Include `merkledb-derive` as a dependency in your `Cargo.toml`:

## License

`merkledb-derive` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
