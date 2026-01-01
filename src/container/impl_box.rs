//! The repr of [`Box`] is simple and stable. No surprises here.

use alloc::boxed::Box;
use core::{alloc::Layout, ptr::NonNull};

use super::DstContainer;
use crate::{
    SliceDst,
    utils::nalloc::{alloc_maydangle, dealloc_maydangle},
};

unsafe impl<T: ?Sized + SliceDst> DstContainer for Box<T> {
    type Target = T;
    type Context = Layout;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Layout) {
        (unsafe { alloc_maydangle(layout) }, layout)
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, _: Layout) -> Self {
        unsafe { Self::from_raw(ptr.as_ptr()) }
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, layout: Layout) {
        unsafe { dealloc_maydangle(ptr.cast(), layout) }
    }
}
