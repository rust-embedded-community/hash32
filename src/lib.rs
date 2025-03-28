//! 32-bit hashing algorithms
//!
//! # Why?
//!
//! Because 32-bit architectures are a thing (e.g. ARM Cortex-M) and you don't want your hashing
//! function to pull in a bunch of slow 64-bit compiler intrinsics (software implementations of
//! 64-bit operations).
//!
//! # Relationship to `core::hash`
//!
//! This crate extends [`core::hash`] with a 32-bit version of `Hasher`, which extends
//! `core::hash::Hasher`. It requires that the hasher only performs 32-bit operations when computing
//! the hash, and adds [`finish32`] to get the hasher's result as a `u32`. The standard `finish`
//! method should just zero-extend this result.
//!
//! Since it extends `core::hash::Hasher`, `Hasher` can be used with any type which implements the
//! standard `Hash` trait.
//!
//! [`core::hash`]: https://doc.rust-lang.org/std/hash/index.html
//! [`finish32`]: crate::Hasher::finish32
//!
//! # Hashers
//!
//! This crate provides implementations of the following 32-bit hashing algorithms:
//!
//! - [`FnvHasher`] Fowler-Noll-Vo 1a
//! - [`Murmur3Hasher`] `MurmurHash3`
//!
//! # Generic code
//!
//! In generic code, the trait bound `H: core::hash::Hasher` accepts *both* 64-bit hashers like
//! `std::collections::hash_map::DefaultHasher`; and 32-bit hashers like the ones defined in this
//! crate (`hash32::FnvHasher` and `hash32::Murmur3Hasher`)
//!
//! The trait bound `H: hash32::Hasher` is *more* restrictive as it only accepts 32-bit hashers.
//!
//! # MSRV
//!
//! This crate is guaranteed to compile on latest stable Rust. It *might* compile on older
//! versions but that may change in any new patch release.

#![warn(
    missing_docs,
    clippy::use_self,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::trivially_copy_pass_by_ref
)]
#![no_std]

pub use crate::fnv::Hasher as FnvHasher;
pub use crate::murmur3::Hasher as Murmur3Hasher;

mod fnv;
mod murmur3;

/// An extension of [core::hash::Hasher][0] for hashers which use 32 bits.
///
/// For hashers which implement this trait, the standard `finish` method should just return a
/// zero-extended version of the result of `finish32`.
///
/// [0]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
///
/// # Contract
///
/// Implementers of this trait must *not* perform any 64-bit (or 128-bit) operation while computing
/// the hash.
pub trait Hasher: core::hash::Hasher {
    /// The equivalent of [`core::hash::Hasher.finish`][0] for 32-bit hashers.
    ///
    /// This returns the hash directly; `finish` zero-extends it to 64 bits for compatibility.
    ///
    /// [0]: https://doc.rust-lang.org/std/hash/trait.Hasher.html#tymethod.finish
    fn finish32(&self) -> u32;
}
