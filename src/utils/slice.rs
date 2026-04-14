use core::{
    mem::{MaybeUninit, transmute},
    ptr::{NonNull, from_mut, slice_from_raw_parts_mut},
};

#[inline]
pub(crate) const fn slice_from_raw_parts_nonnull<T>(data: NonNull<T>, len: usize) -> NonNull<[T]> {
    let ptr = slice_from_raw_parts_mut(data.as_ptr(), len);
    // SAFETY: `ptr` is guarenteed to remain NonNull
    unsafe { NonNull::new_unchecked(ptr) }
}

#[inline]
pub(crate) const fn slice_into_raw_parts<T>(ptr: *const [T]) -> (*const T, usize) {
    (ptr.cast(), ptr.len())
}

#[inline]
pub(crate) const fn slice_into_raw_parts_mut<T>(ptr: *mut [T]) -> (*mut T, usize) {
    (ptr.cast(), ptr.len())
}

#[inline]
pub(crate) const fn slice_into_raw_parts_nonnull<T>(ptr: NonNull<[T]>) -> (NonNull<T>, usize) {
    (ptr.cast(), ptr.len())
}

/// Coerce a [`slice`] of `T` into an uninitialized one
///
/// # Safety
///
/// The transmute in the function is safe because the `repr` remains the same.
///
/// [`slice`]: primitive@slice
#[inline]
pub(crate) const fn slice_as_uninit<T>(this: &[T]) -> &[MaybeUninit<T>] {
    // SAFETY: MaybeUninit is repr(transparent) and never written to
    unsafe { transmute(this) }
}

#[inline]
pub(crate) const unsafe fn slice_assume_init_mut<T>(this: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(from_mut(this) as *mut [T]) }
}
