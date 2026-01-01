use core::{alloc::Layout, ptr::NonNull};

use crate::{
    SliceDst,
    cast::dst_from_raw_parts_nonnull,
    layout::layout_for_len,
    utils::nalloc::{AllocFn, alloc_if_non_zero, nonnull_alloc},
};

#[inline]
pub(super) fn alloc_for_slice_dst_raw<T, F>(alloc_fn: F, len: usize) -> (Layout, NonNull<T>)
where
    T: ?Sized + SliceDst,
    F: AllocFn,
{
    let layout = layout_for_len::<T>(len);
    let ptr = alloc_if_non_zero(alloc_fn, layout);
    (layout, dst_from_raw_parts_nonnull(ptr, len))
}

/// [alloc_for_slice_dst] but with support for custom allocators
///
/// TODO: allocator trait support
#[inline]
fn alloc_for_slice_dst_with<T, F>(alloc_fn: F, len: usize) -> NonNull<T>
where
    T: ?Sized + SliceDst,
    F: AllocFn,
{
    alloc_for_slice_dst_raw(alloc_fn, len).1
}

/// Allocates space for a DST on the heap using the system allocator.
///
/// This function uses the [alloc] allocator, which means the memory can be
/// deallocated with [dealloc] or any smart pointer using the `System`
/// allocator.
///
/// # Notes
///
/// This function returns a [NonNull] pointer, because [MaybeUninit] does not
/// support [?Sized] types.
///
/// [MaybeUninit]: core::mem::MaybeUninit
/// [alloc]: alloc::alloc::alloc
/// [dealloc]: alloc::alloc::dealloc
#[inline]
pub fn alloc_for_slice_dst<T>(len: usize) -> NonNull<T>
where
    T: ?Sized + SliceDst,
{
    let alloc_fn = |layout| unsafe { nonnull_alloc(layout) };
    alloc_for_slice_dst_with(alloc_fn, len)
}
