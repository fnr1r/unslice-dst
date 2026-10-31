#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use self::layout::MaybeDstLayout;
pub use self::{
    allocation::{AllocSliceDst, alloc_for_slice_dst},
    cast::DstCast,
    fat_ptr::{dst_addr, dst_data, dst_len},
    layout::DstLayout,
    structs::{MockDst, SliceWithHeader},
};

pub mod allocation;
pub mod cast;
#[cfg(feature = "container_unseal")]
pub mod container;
#[cfg(not(feature = "container_unseal"))]
mod container;
pub mod fat_ptr;
pub(crate) mod initializers;
pub mod layout;
pub mod structs;
pub mod uninit;
mod utils;

/// Just a type alias for a slice of units.
///
/// It's assumed to always be of size 0.
pub type AnyDst = [()];

/// Trait alias for every slice-like DST
///
/// Automatically implemented for every type which implements [`DstCast`] and
/// [`DstLayout`]
pub trait SliceDst: DstCast + MaybeDstLayout {}

impl<T: ?Sized + DstCast + MaybeDstLayout> SliceDst for T {}
