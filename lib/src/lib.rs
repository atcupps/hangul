//! lib/src/lib.rs
//! A library for working with Hangul (Korean script) at the jamo, block,
//! word, and string levels.

/// A module for working with Hangul syllable blocks.
pub mod block;

/// A module for working with Hangul jamo characters.
pub mod jamo;

/// A module for working with strings mixing Hangul and non-Hangul characters.
pub mod string;

/// A module for working with Hangul words.
pub mod word;
