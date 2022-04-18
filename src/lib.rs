//! 32-bit hashing machinery
//!
//! # Why?
//!
//! Because 32-bit architectures are a thing (e.g. ARM Cortex-M) and you don't want your hashing
//! function to pull in a bunch of slow 64-bit compiler intrinsics (software implementations of
//! 64-bit operations).
//!
//! # Relationship to `core::hash`
//!
//! This crate exposes the same interfaces you'll find in [`core::hash`]: `Hash`, `Hasher`,
//! `BuildHasher` and `BuildHasherDefault`. The main difference is that `hash32::Hasher::finish`
//! returns a `u32` instead of `u64`, and the contract of `hash32::Hasher` forbids the implementer
//! from performing 64-bit (or 128-bit) operations while computing the hash.
//!
//! [`core::hash`]: https://doc.rust-lang.org/std/hash/index.html
//!
//! # `#[derive(Hash32)]`
//!
//! The easiest way to implement `hash32::Hash` for a `struct` is to use the `#[derive(Hash32)]`.
//!
//! Note that you need to *explicitly* depend on both `hash32` *and* `hash32_derive`; both crates
//! must appear in your `Cargo.toml`.
//!
//! ``` ignore
//! use hash32_derive::Hash32;
//!
//! #[derive(Hash32)]
//! struct Ipv4Addr([u8; 4]);
//!
//! # fn main() {}
//!
//! ```
//! # Hashers
//!
//! This crate provides implementations of the following 32-bit hashing algorithms:
//!
//! - [Fowler-Noll-Vo](struct.FnvHasher.html)
//! - [MurmurHash3](struct.Murmur3Hasher.html)
//!
//! # MSRV
//!
//! This crate is guaranteed to compile on latest stable Rust. It *might* compile on older
//! versions but that may change in any new patch release.
//!
//! # Future
//!
//! In the future we'd like to deprecate this crate in favor of making `core::hash::Hasher` generic
//! over the size of the computed hash. Below is shown the planned change (but it doesn't work due
//! to limitations in the `associated_type_defaults` feature):
//!
//! ``` ignore
//! #![feature(associated_type_defaults)]
//!
//! trait Hasher {
//!     type Hash = u64; // default type for backwards compatibility
//!
//!     fn finish(&self) -> Self::Hash; // changed
//!     fn write(&mut self, bytes: &[u8]);
//! }
//! ```
//!
//! With this change a single `#[derive(Hash)]` would enough to make a type hashable with 32-bit and
//! 64-bit hashers.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate byteorder;

pub use fnv::Hasher as FnvHasher;
pub use murmur3::Hasher as Murmur3Hasher;

mod fnv;
mod murmur3;

/// An extension of [core::hash::Hasher][0] for hashers which use 32 bits.
///
/// Typically, the standard `finish` method will just return a zero-extended version of the result
/// of `finish32`.
///
/// [0]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
///
/// # Contract
///
/// Implementers of this trait must *not* perform any 64-bit (or 128-bit) operation while computing
/// the hash.
pub trait Hasher: core::hash::Hasher {
    /// See [`core::hash::Hasher.finish`][0]
    ///
    /// [0]: https://doc.rust-lang.org/std/hash/trait.Hasher.html#tymethod.finish
    fn finish32(&self) -> u32;
}
