use core::{marker::PhantomData, mem::ManuallyDrop, ptr::NonNull};

use crate::{cast::dst_from_raw_parts_nonnull, container::DstContainer, layout::layout_for_len};

#[inline]
const unsafe fn unwrap_invariant<T>(value: Option<T>, s: &str) -> T {
    if cfg!(debug_assertions) {
        value.expect(s)
    } else {
        unsafe { value.unwrap_unchecked() }
    }
}

#[derive(Debug)]
#[must_use]
pub(super) struct UninitBoxInner<T: DstContainer> {
    ptr: NonNull<T::Target>,
    context: T::Context,
    container: PhantomData<T>,
}

impl<T: DstContainer> UninitBoxInner<T> {
    #[inline]
    const fn new(ptr: NonNull<T::Target>, context: T::Context) -> Self {
        Self {
            ptr,
            context,
            container: PhantomData,
        }
    }
    #[inline]
    fn alloc_for_len(len: usize) -> Self {
        let layout = layout_for_len::<T::Target>(len);
        let (ptr, ctx) = unsafe { T::dst_allocate(layout) };
        let ptr = dst_from_raw_parts_nonnull(ptr, len);
        Self::new(ptr, ctx)
    }
    #[inline]
    fn finalize(self) -> T {
        unsafe { T::dst_finalize(self.ptr, self.context) }
    }
    #[inline]
    fn destroy(self) {
        unsafe { T::dst_dealloc(self.ptr, self.context) }
    }
}

pub(super) struct UninitBox<T: DstContainer> {
    /// Invariant: This is always [`Some`]. It's only ever invalidated in
    /// [`Drop::drop`] and [`Self::finalize`].
    inner: Option<UninitBoxInner<T>>,
}

impl<T: DstContainer> UninitBox<T> {
    #[inline]
    const fn new(inner: UninitBoxInner<T>) -> Self {
        Self { inner: Some(inner) }
    }
    #[inline]
    pub(super) fn alloc_for_len(len: usize) -> Self {
        Self::new(UninitBoxInner::<T>::alloc_for_len(len))
    }
    #[inline]
    const fn borrow(&self) -> &UninitBoxInner<T> {
        unsafe { unwrap_invariant(self.inner.as_ref(), "invariants violated") }
    }
    #[inline]
    pub(super) const fn as_ptr(&self) -> NonNull<T::Target> {
        self.borrow().ptr
    }
    #[inline]
    const fn take_once(&mut self) -> UninitBoxInner<T> {
        unsafe { unwrap_invariant(self.inner.take(), "invariants violated") }
    }
    #[inline]
    pub(super) fn finalize(self) -> T {
        ManuallyDrop::new(self).take_once().finalize()
    }
}

impl<T: DstContainer> Drop for UninitBox<T> {
    #[inline]
    fn drop(&mut self) {
        self.take_once().destroy();
    }
}
