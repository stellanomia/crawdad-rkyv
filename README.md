# crawdad-rkyv

[![Crates.io](https://img.shields.io/crates/v/crawdad-rkyv)](https://crates.io/crates/crawdad-rkyv)
[![Documentation](https://docs.rs/crawdad-rkyv/badge.svg)](https://docs.rs/crawdad-rkyv)

**Note: This is a fork of the original [daac-tools/crawdad](https://github.com/daac-tools/crawdad) created to provide serialization support via the `rkyv` framework.**

This fork adds `rkyv`'s `Archive`, `Serialize`, and `Deserialize` traits to the data structures within `crawdad`. This modification enables zero-copy deserialization, which is essential for projects requiring near-instantaneous loading of serialized data, such as [vibrato-rkyv](https://github.com/stellanomia/vibrato-rkyv).

All credit for the core implementation and functionality goes to the original authors of `crawdad`.

## Purpose

The primary purpose of this fork is to serve as a dependency for other `-rkyv` suffixed projects. It is published on crates.io to satisfy the dependency requirements for publishing those projects.

## Changes

*   Added `#[derive(Archive, Serialize, Deserialize)]` to `Trie` and other relevant structs.
*   Implemented necessary methods on the `Archived` versions of structs to maintain functionality (e.g., `common_prefix_search` on `ArchivedTrie`).

## Usage

You would typically not use this crate directly. Instead, it is used as a dependency by other libraries that rely on `rkyv`-serializable versions of `crawdad`'s data structures.

## Original README

For information on the core functionality, please refer to the [original README file](https://github.com/daac-tools/crawdad/blob/main/README.md).

## License

This project is licensed under the same terms as the original `crawdad` library (MIT OR Apache-2.0).