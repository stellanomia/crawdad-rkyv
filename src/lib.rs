//! 🦞 Crawdad: ChaRActer-Wise Double-Array Dictionary
//!
//! Crawdad is a library of natural language dictionaries using character-wise double-array tries.
//! The implementation is optimized for strings of multibyte-characters,
//! and you can enjoy fast text processing on such strings such as Japanese or Chinese.
//!
//! # Data structures
//!
//! Crawdad contains the two trie implementations:
//!
//! - [`Trie`] is a standard trie form that often provides the fastest queries.
//! - [`MpTrie`] is a minimal-prefix trie form that is memory-efficient for long strings.
//!
//! # Examples
//!
//! ## Looking up an input key
//!
//! To get a value associated with an input key, use [`Trie::exact_match()`].
//!
//! ```
//! use crawdad_rkyv::Trie;
//!
//! let keys = vec!["世界", "世界中", "国民"];
//! let trie = Trie::from_keys(&keys).unwrap();
//!
//! assert_eq!(trie.exact_match("世界中".chars()), Some(1));
//! assert_eq!(trie.exact_match("日本中".chars()), None);
//! ```
//!
//! ## Finding all occurrences of keys in an input text
//!
//! To search for all occurrences of registered keys in an input text,
//! use [`Trie::common_prefix_search()`] for all starting positions in the text.
//!
//! ```
//! use crawdad_rkyv::Trie;
//!
//! let keys = vec!["世界", "世界中", "国民"];
//! let trie = Trie::from_keys(&keys).unwrap();
//!
//! let haystack: Vec<char> = "国民が世界中にて".chars().collect();
//! let mut matches = vec![];
//!
//! for i in 0..haystack.len() {
//!     for (v, j) in trie.common_prefix_search(haystack[i..].iter().copied()) {
//!         matches.push((v, i..i + j));
//!     }
//! }
//!
//! assert_eq!(
//!     matches,
//!     vec![(2, 0..2), (0, 3..5), (1, 3..6)]
//! );
//! ```
//!
//! ## Serializing and deserializing the data structure
//!
//! To serialize/deserialize the data structure into/from a byte sequence,
//! use [`Trie::serialize_to_vec()`]/[`Trie::deserialize_from_slice()`].
//!
//! ```
//! use crawdad_rkyv::Trie;
//!
//! let keys = vec!["世界", "世界中", "国民"];
//! let trie = Trie::from_keys(&keys).unwrap();
//!
//! let bytes = trie.serialize_to_vec();
//! let (other, _) = Trie::deserialize_from_slice(&bytes);
//!
//! assert_eq!(trie.io_bytes(), other.io_bytes());
//! ```
#![deny(missing_docs)]
#![no_std]

#[cfg(target_pointer_width = "16")]
compile_error!("`target_pointer_width` must be larger than or equal to 32");

#[cfg(not(feature = "alloc"))]
compile_error!("`alloc` feature is currently required to build this crate");

#[macro_use]
extern crate alloc;

mod builder;
pub mod errors;
mod mapper;
pub mod mptrie;
pub mod trie;
mod utils;

pub(crate) const OFFSET_MASK: u32 = 0x7fff_ffff;
pub(crate) const INVALID_IDX: u32 = 0xffff_ffff;
pub(crate) const MAX_VALUE: u32 = OFFSET_MASK;
pub(crate) const END_CODE: u32 = 0;

/// Special terminator, which must not be contained in keys.
pub const END_MARKER: char = '\u{ffff}';

pub use mptrie::MpTrie;
use rkyv::{Archive, Deserialize, Serialize};
pub use trie::Trie;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
struct Node {
    base: u32,
    check: u32,
}

impl Node {
    #[inline(always)]
    pub const fn get_base(&self) -> u32 {
        self.base & OFFSET_MASK
    }

    #[inline(always)]
    pub const fn get_check(&self) -> u32 {
        self.check & OFFSET_MASK
    }

    #[inline(always)]
    pub const fn is_leaf(&self) -> bool {
        self.base & !OFFSET_MASK != 0
    }

    #[inline(always)]
    pub const fn has_leaf(&self) -> bool {
        self.check & !OFFSET_MASK != 0
    }

    #[inline(always)]
    pub const fn is_vacant(&self) -> bool {
        self.base == OFFSET_MASK && self.check == OFFSET_MASK
    }

    pub const fn io_bytes() -> usize {
        8
    }

    #[inline(always)]
    fn serialize(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        bytes[0..4].copy_from_slice(&self.base.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.check.to_le_bytes());
        bytes
    }

    #[inline(always)]
    fn deserialize(bytes: [u8; 8]) -> Self {
        Self {
            base: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            check: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        }
    }
}

impl ArchivedNode {
    #[inline(always)]
    pub fn get_base(&self) -> u32 {
        self.base & OFFSET_MASK
    }

    #[inline(always)]
    pub fn get_check(&self) -> u32 {
        self.check & OFFSET_MASK
    }

    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.base & !OFFSET_MASK != 0
    }

    #[inline(always)]
    pub fn has_leaf(&self) -> bool {
        self.check & !OFFSET_MASK != 0
    }

    #[inline(always)]
    pub fn is_vacant(&self) -> bool {
        self.base == OFFSET_MASK && self.check == OFFSET_MASK
    }
}
