//! Fat pointer utils for slice-like DSTs
//!
//! Depends on the casting module.

use self::slice_dst_pointer::Inner;
pub use self::{pointer_with::PointerWith, slice_dst_pointer::SliceDstPointer};
use crate::DstCast;

mod pointer_with;
mod slice_dst_pointer;

/// Extracts the data from a fat pointer to `T`
///
/// See [`SliceDstPointer::from_ptr`] for more info.
#[inline]
pub const fn dst_data<T: ?Sized + DstCast>(ptr: *const T) -> Inner {
    SliceDstPointer::from_ptr(ptr).into_inner()
}

/// Extracts the address from a fat pointer to `T`
///
/// See [`SliceDstPointer::from_ptr`] for more info.
///
/// This function is just here for feature parity with as-casting.
#[inline]
pub const fn dst_addr<T: ?Sized + DstCast>(ptr: *const T) -> *const () {
    dst_data(ptr).address
}

/// Extracts the length from a fat pointer to `T`
///
/// See [`SliceDstPointer::from_ptr`] for more info.
#[inline]
pub const fn dst_len<T: ?Sized + DstCast>(ptr: *const T) -> usize {
    dst_data(ptr).metadata
}
