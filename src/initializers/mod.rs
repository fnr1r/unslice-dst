//! Private initializers API

pub(crate) use self::aliases::*;
use crate::{
    DstCast, DstLayout,
    cast::dst_cast_nonnull,
    uninit::{UninitMut, UninitRef},
};

mod aliases;

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
        let head_offset = Self::head_offset(this.into_ref());
        let tail_offset = Self::tail_offset(this.into_ref());
        let head_ptr = this.cast::<Self::Head>();
        let head_ptr = unsafe { head_ptr.byte_add(head_offset) };
        let tail_ptr = dst_cast_nonnull::<_, [Self::Tail]>(*this);
        let tail_ptr = unsafe { tail_ptr.byte_add(tail_offset) };
        unsafe { (UninitMut::new(head_ptr), UninitMut::new(tail_ptr)) }
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
