use core::alloc::Layout;

use unslice_dst_cast::{DstCast, utils::transmute_lax};

#[inline]
const unsafe fn raw_pointer_to_reference_ignoring_ub<'a, T>(v: *const T) -> &'a T
where
    T: ?Sized + DstCast,
{
    unsafe { transmute_lax::<*const T, &T>(v) }
}

#[inline]
pub(super) const unsafe fn layout_for_value_raw<T: ?Sized + DstCast>(t: *const T) -> Layout {
    // SAFETY: This is NOT safe. This is UB.
    let fake_ref = unsafe { raw_pointer_to_reference_ignoring_ub(t) };
    Layout::for_value(fake_ref)
}
