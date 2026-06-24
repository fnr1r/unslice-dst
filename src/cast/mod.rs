//! Support for casting slice-like DSTs
//!
//! Exists only because stable Rust forbids as-casts between `?Sized` pointers
//! (except for concrete and compatible types)

pub use self::{
    dst_cast::DstCast,
    funcs::{
        dst_cast_const, dst_cast_mut, dst_cast_nonnull, dst_from_raw_parts, dst_from_raw_parts_mut,
        dst_from_raw_parts_nonnull, dst_into_raw_parts, dst_into_raw_parts_mut,
        dst_into_raw_parts_nonnull,
    },
};
pub use crate::dst_cast_impl;

mod dst_cast;
mod funcs;
mod impls;
#[cfg(not(feature = "cast_macro_tt_muncher"))]
mod macro_simple;
#[cfg(feature = "cast_macro_tt_muncher")]
mod macro_tt_muncher;
#[cfg_attr(not(feature = "cast_unseal"), doc(hidden))]
pub mod sealed;

#[cfg(test)]
mod tests;
