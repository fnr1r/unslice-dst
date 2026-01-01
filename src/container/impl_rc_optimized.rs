use alloc::{rc::Rc, sync::Arc};
use core::{alloc::Layout, ptr::NonNull};

use super::DstContainer;
use crate::{
    SliceDst,
    utils::nalloc::{nonnull_alloc, nonnull_dealloc},
};

#[derive(Debug, Clone, Copy)]
#[repr(C, align(2))]
struct RcHeader {
    strong: usize,
    weak: usize,
}

impl RcHeader {
    // NOTE: Maybe move to const-default
    const DEFAULT: Self = Self { strong: 1, weak: 1 };
    const LAYOUT: Layout = Layout::new::<Self>();
}

#[inline]
fn rc_info(inner_layout: Layout) -> (Layout, usize) {
    let (layout, data_offset) = RcHeader::LAYOUT.extend(inner_layout).unwrap();
    (layout.pad_to_align(), data_offset)
}

#[inline]
const unsafe fn rc_init(ptr: NonNull<()>) {
    let refcnt = ptr.cast::<RcHeader>();
    unsafe { refcnt.write(RcHeader::DEFAULT) }
}

#[inline]
unsafe fn rc_alloc(layout: Layout) -> (NonNull<()>, Layout) {
    let (rc_layout, data_offset) = rc_info(layout);
    debug_assert_ne!(layout.size(), 0, "Illegal refcount ptr of size 0");
    // SAFETY: The size of RcInner can't be 0
    let ptr = unsafe { nonnull_alloc(rc_layout) }.cast();
    unsafe { rc_init(ptr) }
    let ptr = unsafe { ptr.byte_add(data_offset) };
    (ptr, layout)
}

#[inline]
unsafe fn rc_finalize<T: ?Sized, A>(ptr: NonNull<T>, cvt: unsafe fn(*const T) -> A) -> A {
    unsafe { cvt(ptr.as_ptr()) }
}

#[inline]
unsafe fn rc_dealloc<T: ?Sized>(ptr: NonNull<T>, layout: Layout) {
    let (rc_layout, data_offset) = rc_info(layout);
    let ptr = unsafe { ptr.byte_sub(data_offset) }.cast();
    unsafe { nonnull_dealloc(ptr, rc_layout) }
}

unsafe impl<T: ?Sized + SliceDst> DstContainer for Rc<T> {
    type Target = T;
    type Context = Layout;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Self::Context) {
        unsafe { rc_alloc(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, _: Self::Context) -> Self {
        unsafe { rc_finalize(ptr, Self::from_raw) }
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: Self::Context) {
        unsafe { rc_dealloc(ptr, ctx) };
    }
}

unsafe impl<T: ?Sized + SliceDst> DstContainer for Arc<T> {
    type Target = T;
    type Context = Layout;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Self::Context) {
        unsafe { rc_alloc(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, _: Self::Context) -> Self {
        unsafe { rc_finalize(ptr, Self::from_raw) }
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: Self::Context) {
        unsafe { rc_dealloc(ptr, ctx) };
    }
}
