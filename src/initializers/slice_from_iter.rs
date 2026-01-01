use core::{
    mem::{ManuallyDrop, MaybeUninit},
    ptr::{drop_in_place, slice_from_raw_parts_mut},
};

use super::{TailInit, assume_init_slice};

#[inline]
const unsafe fn maybe_uninit_assume_partial_init<T>(
    src: &mut [MaybeUninit<T>],
    len: usize,
) -> &mut [T] {
    debug_assert!(len <= src.len());
    let ptr = slice_from_raw_parts_mut(src.as_mut_ptr() as *mut T, len);
    // SAFETY: The pointer is created from a mutable reference
    unsafe { ptr.as_mut().unwrap_unchecked() }
}

struct SliceWriter<'a, T> {
    inner: &'a mut [MaybeUninit<T>],
    current_len: usize,
}

impl<T> SliceWriter<'_, T> {
    #[allow(unsafe_op_in_unsafe_fn)]
    #[inline]
    unsafe fn write_item(&mut self, item: T) {
        assert!(
            self.current_len < self.inner.len(),
            "ExactSizeIterator over-reported length"
        );
        self.inner.get_unchecked_mut(self.current_len).write(item);
        self.current_len += 1;
    }
}

impl<T> Drop for SliceWriter<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let data = unsafe { maybe_uninit_assume_partial_init(self.inner, self.current_len) };
        unsafe { drop_in_place(data) }
    }
}

#[inline]
fn slice_from_iter<T>(ptr: &mut [MaybeUninit<T>], items: impl ExactSizeIterator<Item = T>) {
    let mut writer = SliceWriter {
        inner: ptr,
        current_len: 0,
    };
    items.for_each(|e| unsafe { writer.write_item(e) });
    assert_eq!(
        writer.current_len,
        writer.inner.len(),
        "ExactSizeIterator under-reported length"
    );
    let _ = ManuallyDrop::new(writer);
}

#[inline]
fn write_slice_iter<T>(
    src: impl ExactSizeIterator<Item = T>,
    dest: &mut [MaybeUninit<T>],
) -> &mut [T] {
    slice_from_iter(dest, src);
    unsafe { assume_init_slice(dest) }
}

#[inline]
pub(crate) fn write_slice_iter_fn<T>(value: impl ExactSizeIterator<Item = T>) -> impl TailInit<T> {
    |this| write_slice_iter(value, this)
}
