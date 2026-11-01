#![expect(clippy::incompatible_msrv)]

use core::alloc::Layout;

use unslice_dst_cast::DstCast;

#[inline]
pub(super) const unsafe fn layout_for_value_raw<T: ?Sized + DstCast>(t: *const T) -> Layout {
    unsafe { Layout::for_value_raw(t) }
}
