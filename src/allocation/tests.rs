use core::{alloc::Layout, ptr::NonNull};

use super::funcs::alloc_for_slice_dst_raw;
use crate::{
    SliceDst,
    utils::nalloc::{dealloc_maydangle, nonnull_alloc},
};

#[inline]
pub(super) fn alloc_for_slice_dst_wl<T>(len: usize) -> (Layout, NonNull<T>)
where
    T: ?Sized + SliceDst,
{
    let alloc_fn = |layout| unsafe { nonnull_alloc(layout) };
    alloc_for_slice_dst_raw(alloc_fn, len)
}

fn with_dst<T: ?Sized + SliceDst, R>(len: usize, f: impl FnOnce(NonNull<T>, &Layout) -> R) -> R {
    let (layout, ptr) = alloc_for_slice_dst_wl::<T>(len);
    let res = f(ptr, &layout);
    unsafe { dealloc_maydangle(ptr.cast(), layout) }
    res
}

fn ignore_layout_fn<T: ?Sized + SliceDst, R>(
    f: impl FnOnce(NonNull<T>) -> R,
) -> impl FnOnce(NonNull<T>, &Layout) -> R {
    |ptr, _layout| f(ptr)
}

#[test]
fn assert_works_for_zst() {
    let f = ignore_layout_fn(|ptr: NonNull<[()]>| ptr.addr());
    assert_eq!(with_dst(24, f).get(), 1);
}
