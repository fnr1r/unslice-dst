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
/// This WILL fail in Miri.
#[allow(dead_code)]
pub(super) const fn layout_for_len_hack<T>(len: usize) -> Layout
where
    T: ?Sized + DstCast,
{
    let ptr = fake_ptr::<T>(len);
    #[cfg(not(miri))]
    {
        unsafe { layout_for_value_raw(ptr) }
    }
    #[cfg(miri)]
    {
        unsafe { Layout::for_value_raw(ptr) }
    }
}
