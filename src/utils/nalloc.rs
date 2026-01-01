use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::{alloc::Layout, ptr::NonNull};

#[inline]
unsafe fn nonnull_alloc_raw(layout: Layout) -> Option<NonNull<u8>> {
    NonNull::new(unsafe { alloc(layout) })
}

#[inline]
unsafe fn nonnull_alloc(layout: Layout) -> NonNull<u8> {
    unsafe { nonnull_alloc_raw(layout) }.unwrap_or_else(|| handle_alloc_error(layout))
}

/// # Safety
///
/// Although the UB is handled, this function is unsafe to match the others.
#[inline]
pub(crate) unsafe fn alloc_maydangle(layout: Layout) -> NonNull<()> {
    if layout.size() == 0 {
        NonNull::dangling()
    } else {
        unsafe { nonnull_alloc(layout) }.cast()
    }
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
