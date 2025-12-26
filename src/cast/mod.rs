//! Support for casting slice-like DSTs

pub use self::dst_cast::DstCast;
pub use crate::dst_cast_impl;

mod dst_cast;
mod impls;
#[cfg(not(feature = "cast_macro_tt_muncher"))]
mod macro_simple;
#[cfg(feature = "cast_macro_tt_muncher")]
mod macro_tt_muncher;
#[cfg_attr(not(feature = "cast_unseal"), doc(hidden))]
pub mod sealed;

#[cfg(test)]
mod tests;
