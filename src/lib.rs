#![doc = include_str!("../README.md")]
#![deny(meta_variable_misuse)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::missing_const_for_fn)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(miri, feature(layout_for_ptr))]

extern crate alloc;

use self::layout::MaybeDstLayout;
pub use self::{
    cast::DstCast,
    fat_ptr::{dst_addr, dst_data, dst_len},
    layout::DstLayout,
};

pub mod cast;
#[cfg(feature = "container_unseal")]
pub mod container;
#[cfg(not(feature = "container_unseal"))]
mod container;
pub mod fat_ptr;
pub mod layout;
mod utils;

/// Just a type alias for a slice of units.
///
/// It's assumed to always be of size 0.
pub type AnyDst = [()];

/// Trait alias for every slice-like DST
///
/// Automatically implemented for every type which implements [DstCast] and
/// [DstLayout]
pub trait SliceDst: DstCast + MaybeDstLayout {}

impl<T: ?Sized + DstCast + MaybeDstLayout> SliceDst for T {}
