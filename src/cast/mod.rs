//! Support for casting slice-like DSTs

pub use self::dst_cast::DstCast;
pub use crate::dst_cast_impl;

mod dst_cast;
mod impls;
mod auto_impl;
