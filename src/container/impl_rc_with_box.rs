//! This is the official way to make an Rc pointer - by converting it from a
//! [`Box`]. Expect an optimized version.

use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::{alloc::Layout, ptr::NonNull};

use super::DstContainer;
use crate::SliceDst;

unsafe impl<T: ?Sized + SliceDst> DstContainer for Rc<T> {
    type Target = T;
    type Context = Layout;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Layout) {
        unsafe { Box::<T>::dst_allocate(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, ctx: Layout) -> Self {
        unsafe { Box::dst_finalize(ptr, ctx) }.into()
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: Layout) {
        unsafe { Box::<T>::dst_dealloc(ptr, ctx) }
    }
}

unsafe impl<T: ?Sized + SliceDst> DstContainer for Arc<T> {
    type Target = T;
    type Context = Layout;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Layout) {
        unsafe { Box::<T>::dst_allocate(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, ctx: Layout) -> Self {
        unsafe { Box::dst_finalize(ptr, ctx) }.into()
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: Layout) {
        unsafe { Box::<T>::dst_dealloc(ptr, ctx) }
    }
}
