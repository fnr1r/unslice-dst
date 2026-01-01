use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::{alloc::Layout, ptr::NonNull};

pub(crate) trait AllocFn: FnOnce(Layout) -> NonNull<u8> {}

impl<F: FnOnce(Layout) -> NonNull<u8>> AllocFn for F {}

#[inline]
unsafe fn nonnull_alloc_raw(layout: Layout) -> Option<NonNull<u8>> {
    NonNull::new(unsafe { alloc(layout) })
}

#[inline]
pub(crate) unsafe fn nonnull_alloc(layout: Layout) -> NonNull<u8> {
    unsafe { nonnull_alloc_raw(layout) }.unwrap_or_else(|| handle_alloc_error(layout))
}

#[inline]
pub(crate) fn alloc_if_non_zero(alloc_fn: impl AllocFn, layout: Layout) -> NonNull<()> {
    if layout.size() == 0 {
        NonNull::dangling()
    } else {
        alloc_fn(layout).cast()
    }
}

/// # Safety
///
/// Although the UB is handled, this function is unsafe to match the others.
#[inline]
pub(crate) unsafe fn alloc_maydangle(layout: Layout) -> NonNull<()> {
    alloc_if_non_zero(|layout| unsafe { nonnull_alloc(layout) }, layout)
}

#[inline]
unsafe fn nonnull_dealloc(ptr: NonNull<u8>, layout: Layout) {
    unsafe { dealloc(ptr.as_ptr(), layout) }
}

#[inline]
pub(crate) unsafe fn dealloc_maydangle(ptr: NonNull<u8>, layout: Layout) {
    if layout.size() == 0 {
        return;
    }
    unsafe { nonnull_dealloc(ptr.cast(), layout) }
}
