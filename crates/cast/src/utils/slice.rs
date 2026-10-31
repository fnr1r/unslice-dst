use core::ptr::{NonNull, slice_from_raw_parts_mut};

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
