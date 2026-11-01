//! Support for casting slice-like DSTs
//!
//! Exists only because stable Rust forbids as-casts between `?Sized` pointers
//! (except for concrete and compatible types)
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::{
    dst_cast::DstCast,
    funcs::{
        dst_cast_const, dst_cast_mut, dst_cast_nonnull, dst_from_raw_parts, dst_from_raw_parts_mut,
        dst_from_raw_parts_nonnull, dst_into_raw_parts, dst_into_raw_parts_mut,
        dst_into_raw_parts_nonnull,
    },
};

mod dst_cast;
mod funcs;
mod impls;
mod macros;
#[cfg_attr(not(feature = "unseal"), doc(hidden))]
pub mod sealed;
#[doc(hidden)]
pub mod utils;

#[cfg(test)]
mod tests;

/// Just a type alias for a slice of units.
///
/// It's assumed to always be of size 0.
pub type AnyDst = [()];
