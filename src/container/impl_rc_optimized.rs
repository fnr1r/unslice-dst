use alloc::{rc::Rc, sync::Arc};
use core::{alloc::Layout, ptr::NonNull};

use super::DstContainer;
use crate::{
    SliceDst,
    utils::nalloc::{nonnull_alloc, nonnull_dealloc},
};

#[derive(Debug)]
#[repr(C)]
struct RcHeader {
    strong: usize,
    weak: usize,
}

impl RcHeader {
    // NOTE: Maybe move to const-default
    const DEFAULT: Self = Self { strong: 1, weak: 1 };
    const LAYOUT: Layout = Layout::new::<Self>();
}

/// This needs to be public because it's used in the trait impl. Don't rely on
/// this.
#[allow(missing_copy_implementations, unreachable_pub)]
#[derive(Debug)]
pub struct RcAllocContext {
    layout: Layout,
    data_offset: usize,
}

#[inline]
fn rc_context(layout: Layout) -> RcAllocContext {
    let (layout, data_offset) = RcHeader::LAYOUT.extend(layout).unwrap();
    RcAllocContext {
        layout: layout.pad_to_align(),
        data_offset,
    }
}

#[inline]
const unsafe fn rc_init(ptr: NonNull<()>) {
    let refcnt = ptr.cast::<RcHeader>();
    unsafe { refcnt.write(RcHeader::DEFAULT) }
}

#[inline]
unsafe fn rc_alloc(layout: Layout) -> (NonNull<()>, RcAllocContext) {
    let ctx = rc_context(layout);
    // SAFETY: The size of RcInner can't be 0
    let ptr = unsafe { nonnull_alloc(ctx.layout) }.cast();
    unsafe { rc_init(ptr) }
    let ptr = unsafe { ptr.byte_add(ctx.data_offset) };
    (ptr, ctx)
}

#[inline]
unsafe fn rc_finalize<T: ?Sized, A>(ptr: NonNull<T>, cvt: unsafe fn(*const T) -> A) -> A {
    unsafe { cvt(ptr.as_ptr()) }
}

#[inline]
unsafe fn rc_dealloc<T: ?Sized>(ptr: NonNull<T>, ctx: RcAllocContext) {
    let ptr = unsafe { ptr.byte_sub(ctx.data_offset) }.cast();
    unsafe { nonnull_dealloc(ptr, ctx.layout) }
}

unsafe impl<T: ?Sized + SliceDst> DstContainer for Rc<T> {
    type Target = T;
    type Context = RcAllocContext;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, RcAllocContext) {
        unsafe { rc_alloc(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, _: RcAllocContext) -> Self {
        unsafe { rc_finalize(ptr, Self::from_raw) }
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: RcAllocContext) {
        unsafe { rc_dealloc(ptr, ctx) };
    }
}

unsafe impl<T: ?Sized + SliceDst> DstContainer for Arc<T> {
    type Target = T;
    type Context = RcAllocContext;
    #[inline]
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, RcAllocContext) {
        unsafe { rc_alloc(layout) }
    }
    #[inline]
    unsafe fn dst_finalize(ptr: NonNull<T>, _: RcAllocContext) -> Self {
        unsafe { rc_finalize(ptr, Self::from_raw) }
    }
    #[inline]
    unsafe fn dst_dealloc(ptr: NonNull<T>, ctx: RcAllocContext) {
        unsafe { rc_dealloc(ptr, ctx) };
    }
}
