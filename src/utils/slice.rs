use core::ptr::{NonNull, slice_from_raw_parts_mut};

#[inline]
pub(crate) const fn slice_from_raw_parts_nonnull<T>(data: NonNull<T>, len: usize) -> NonNull<[T]> {
    let ptr = slice_from_raw_parts_mut(data.as_ptr(), len);
    // SAFETY: `ptr` is guarenteed to remain NonNull
    unsafe { NonNull::new_unchecked(ptr) }
}
