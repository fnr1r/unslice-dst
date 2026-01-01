//! Private initializers API

use core::{
    mem::{MaybeUninit, transmute},
    ptr::NonNull,
};

pub(crate) use self::slice_from_iter::write_slice_iter_fn;
use crate::{DstCast, DstLayout, cast::dst_cast_nonnull};

mod slice_from_iter;

pub(crate) trait DstInit<T: ?Sized>: FnOnce(NonNull<T>) {}
pub(crate) trait HeadInit<T>: FnOnce(&mut MaybeUninit<T>) -> &mut T {}
pub(crate) trait TailInit<T>: FnOnce(&mut [MaybeUninit<T>]) -> &mut [T] {}

impl<T: ?Sized, F: FnOnce(NonNull<T>)> DstInit<T> for F {}
impl<T, F: FnOnce(&mut MaybeUninit<T>) -> &mut T> HeadInit<T> for F {}
impl<T, F: FnOnce(&mut [MaybeUninit<T>]) -> &mut [T]> TailInit<T> for F {}

#[inline]
unsafe fn initialize_for<T: ?Sized + SliceDstSaferInit>(
    init_head: impl HeadInit<T::Head>,
    init_tail: impl TailInit<T::Tail>,
) -> impl DstInit<T> {
    |mut ptr| {
        let (header, items) = unsafe { T::as_uninit(&mut ptr) };
        let _header = init_head(header);
        let _items = init_tail(items);
    }
}

pub(crate) unsafe trait SliceDstSaferInit: DstCast + DstLayout {
    fn head_offset(this: &NonNull<Self>) -> usize;
    fn tail_offset(this: &NonNull<Self>) -> usize;
    #[inline]
    unsafe fn as_uninit(
        this: &mut NonNull<Self>,
    ) -> (&mut MaybeUninit<Self::Head>, &mut [MaybeUninit<Self::Tail>]) {
        let head_offset = Self::head_offset(this);
        let tail_offset = Self::tail_offset(this);
        let tail_ptr = dst_cast_nonnull::<_, [MaybeUninit<Self::Tail>]>(*this);
        let mut tail_ptr = unsafe { tail_ptr.byte_add(tail_offset) };
        let head_ptr = this.cast::<MaybeUninit<Self::Head>>();
        let mut head_ptr = unsafe { head_ptr.byte_add(head_offset) };
        unsafe { (head_ptr.as_mut(), tail_ptr.as_mut()) }
    }
    #[inline]
    unsafe fn initialize_for(
        init_head: impl HeadInit<Self::Head>,
        init_tail: impl TailInit<Self::Tail>,
    ) -> impl DstInit<Self> {
        unsafe { initialize_for(init_head, init_tail) }
    }
}

/// Coerce a [`slice`] of `T` into an uninitialized one
///
/// # Safety
///
/// The transmute in the function is safe because the `repr` remains the same.
///
/// [`slice`]: primitive@slice
#[inline]
pub(crate) fn slice_cast_uninit<T>(value: &[T]) -> &[MaybeUninit<T>] {
    unsafe { transmute(value) }
}

#[inline]
pub(crate) unsafe fn assume_init_slice<T>(value: &mut [MaybeUninit<T>]) -> &mut [T] {
    unsafe { transmute(value) }
}

#[inline]
pub(crate) fn write_fn<T>(value: T) -> impl HeadInit<T> {
    |this| this.write(value)
}

#[inline]
pub(crate) fn write_slice_copied<'a, T: Copy>(
    src: &[T],
    dest: &'a mut [MaybeUninit<T>],
) -> &'a mut [T] {
    let src = slice_cast_uninit(src);
    dest.copy_from_slice(src);
    unsafe { assume_init_slice(dest) }
}

#[inline]
pub(crate) fn write_slice_copied_fn<T: Copy>(value: &[T]) -> impl TailInit<T> {
    |this| write_slice_copied(value, this)
}
