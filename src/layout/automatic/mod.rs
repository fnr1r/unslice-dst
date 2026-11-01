//! An implementation utilizing Undefined Behavior
//!
//! This requires [`cast`](crate::cast) and [`fat_ptr`](crate::fat_ptr).

use core::{alloc::Layout, ptr::null};

use unslice_dst_cast::{DstCast, dst_from_raw_parts};

use self::inner::layout_for_value_raw;

#[cfg_attr(not(any(miri, feature = "feature_layout_for_ptr")), path = "ub.rs")]
#[cfg_attr(any(miri, feature = "feature_layout_for_ptr"), path = "feature.rs")]
mod inner;

const FAKE_ADDR: *const () = null();

/// Create a fake pointer to a slice-like DST
#[inline]
const fn fake_ptr<T: ?Sized + DstCast>(len: usize) -> *const T {
    dst_from_raw_parts(FAKE_ADDR, len)
}

/// Calculate the layout for pointer to slice-like DST `T`
///
/// # Warning
///
/// When not using Miri, this function will assume that the passed pointer is
/// valid. This shouldn't be an issue though. See [`layout_for_len`].
#[inline]
pub(super) const fn layout_for_ptr<T>(ptr: *const T) -> Layout
where
    T: ?Sized + DstCast,
{
    unsafe { layout_for_value_raw(ptr) }
}

/// Calculate the layout for slice-like DST `T`
///
/// # Warning
///
/// This function utilizes the following Undefined Behavior:
///
/// - creates a NULL reference
/// - passes the NULL reference to [`Layout::for_value`]
///
/// This is justified by the fact that this reference is never read from.
///
/// Since this function is only usable for slice-like DSTs implementing
/// [`DstCast`] the Rust compiler should be able to optimize away any
/// implementations of [`Layout::for_value`] which may read from the reference.
/// This can't be guaranteed though.
///
/// # Notes
///
/// This WOULD fail in Miri if I didn't conditionally check for it and use
/// [`Layout::for_value_raw`] instead.
#[cfg_attr(not(feature = "layout_automatic"), allow(dead_code))]
#[inline]
pub(super) const fn layout_for_len<T>(len: usize) -> Layout
where
    T: ?Sized + DstCast,
{
    // SAFETY: This will create a null reference.
    layout_for_ptr(fake_ptr::<T>(len))
}
