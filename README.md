# MerkleDB

**MerkleDB** is a document-oriented persistent storage which provides APIs to
work with merkelized data structures.  Under the hood, MerkleDB uses RocksDB as
a key-value storage.

**MerkleDB** borns as a fork of Exonum (MerkleDB)[https://github.com/exonum/exonum]
to better isolate a useful component that is often required by other blockchains
implementation.

## Features

- Supports list, map and set collections (aka *indexes*),
  as well as singular elements.
  Further, indexes can be organized into groups, allowing to create
  hierarchies of documents with arbitrary nesting.
- Automated state aggregation of top-level indexes into a single
  *state hash*, which reflects the entire database state.
- Ability to define data layouts in an intuitive, declarative format.
- Basic support of transactions: changes to the storage can be
  aggregated into a fork and then merged to the database atomically.
- Access control leveraging the Rust type system, allowing to precisely
  define access privileges for different actors.
- First-class support of long-running, fault-tolerant data migrations
  running concurrently with other I/O to the storage.

## Usage

Include `merkledb` as a dependency in your `Cargo.toml`:

See [the description in Exonum docs][docs:merkledb] for a more detailed overview,
and the [examples](examples) for the examples of usage.

## License

`merkledb` is licensed under the Apache License (Version 2.0).  See
[LICENSE](LICENSE) for details.

[docs:merkledb]: https://exonum.com/doc/version/latest/architecture/merkledb/
