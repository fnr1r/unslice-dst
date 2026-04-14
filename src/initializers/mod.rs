//! Private initializers API

use core::ptr::NonNull;

pub(crate) use self::{aliases::*, slice_from_iter::write_slice_iter_fn};
use crate::{
    DstCast, DstLayout,
    cast::dst_cast_nonnull,
    uninit::{UninitMut, UninitRef},
};

mod aliases;
mod slice_from_iter;

#[allow(dead_code)]
#[inline]
pub(crate) fn init_sized_upcast<T>(f: impl InitSizedCan<T>) -> impl InitSized<T> {
    |ptr| f(ptr.into_canonical())
}

#[allow(dead_code)]
#[inline]
pub(crate) fn init_slice_upcast<T>(f: impl InitSliceCan<T>) -> impl InitSlice<T> {
    |ptr| f(ptr.into_canonical())
}

#[inline]
unsafe fn initialize_for<T: ?Sized + SliceDstSaferInit>(
    init_head: impl InitHead<T>,
    init_tail: impl InitTail<T>,
) -> impl InitDst<T> {
    |ptr| {
        let (header, items) = unsafe { T::as_uninit(ptr) };
        let _header = init_head(header);
        let _items = init_tail(items);
    }
}

/// # Safety
///
/// The head and tail offsets must match what's expected by Rust.
pub(crate) unsafe trait SliceDstSaferInit: DstCast + DstLayout {
    fn head_offset(this: UninitRef<'_, Self>) -> usize;
    fn tail_offset(this: UninitRef<'_, Self>) -> usize;
    #[inline]
    unsafe fn as_uninit(
        this: UninitMut<'_, Self>,
    ) -> (UninitMut<'_, Self::Head>, UninitMut<'_, [Self::Tail]>) {
        let head = unsafe { this.ucopy().into_field(Self::head_offset, NonNull::cast) };
        let tail = unsafe { this.into_field(Self::tail_offset, dst_cast_nonnull) };
        (head, tail)
    }
    #[inline]
    unsafe fn initialize_for(
        init_head: impl InitHead<Self>,
        init_tail: impl InitTail<Self>,
    ) -> impl InitDst<Self> {
        unsafe { initialize_for(init_head, init_tail) }
    }
}

#[inline]
pub(crate) fn write_fn<T>(value: T) -> impl InitSized<T> {
    |this| this.write(value)
}

#[inline]
pub(crate) fn write_slice_copied_fn<T: Copy>(value: &[T]) -> impl InitSlice<T> {
    |this| this.copy_from_slice(value)
}
