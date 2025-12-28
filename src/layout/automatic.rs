//! An implementation utilizing Undefined Behaviour
//!
//! This requires casting and fat_ptr.

use core::alloc::Layout;

#[cfg(not(miri))]
use crate::utils::transmute_lax;
use crate::{DstCast, cast::dst_from_raw_parts};

const FAKE_ADDR: *const () = core::ptr::null();

/// Create a fake pointer to a slice-like DST
const fn fake_ptr<T: ?Sized + DstCast>(len: usize) -> *const T {
    dst_from_raw_parts(FAKE_ADDR, len)
}

#[cfg(not(miri))]
const unsafe fn raw_pointer_to_reference_ignoring_ub<'a, T>(v: *const T) -> &'a T
where
    T: ?Sized + DstCast,
{
    unsafe { transmute_lax::<*const T, &T>(v) }
}

#[cfg(not(miri))]
const unsafe fn layout_for_value_raw<T: ?Sized + DstCast>(t: *const T) -> Layout {
    // SAFETY: This is NOT safe. This is UB.
    let fake_ref = unsafe { raw_pointer_to_reference_ignoring_ub(t) };
    Layout::for_value(fake_ref)
}

/// Calculate the layout for pointer to slice-like DST `T`
///
/// # Warning
///
/// When not using Miri, this function will assume that the passed pointer is
/// valid. This shouldn't be an issue though. See [`layout_for_len`].
pub(super) const fn layout_for_ptr<T>(ptr: *const T) -> Layout
where
    T: ?Sized + DstCast,
{
    #[cfg(not(miri))]
    {
        unsafe { layout_for_value_raw(ptr) }
    }
    #[cfg(miri)]
    {
        unsafe { Layout::for_value_raw(ptr) }
    }
}

/// Calculate the layout for slice-like DST `T`
///
/// # Warning
///
/// This function utilizes the following Undefined Behaviour:
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
pub(super) const fn layout_for_len<T>(len: usize) -> Layout
where
    T: ?Sized + DstCast,
{
    // SAFETY: This will create a null reference.
    layout_for_ptr(fake_ptr::<T>(len))
}
