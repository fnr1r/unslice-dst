//! Support for casting slice-like DSTs

pub use self::{
    dst_cast::DstCast,
    funcs::{dst_cast_const, dst_cast_mut, dst_cast_nonnull},
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
