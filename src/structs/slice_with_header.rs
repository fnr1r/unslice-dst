use core::{
    alloc::Layout,
    mem::offset_of,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    AllocSliceDst, DstLayout, dst_cast_impl, dst_len,
    initializers::{
        DstInit, SliceDstSaferInit, TailInit, write_fn, write_slice_copied_fn, write_slice_iter_fn,
    },
    utils::layout::{LayoutPack, repr_c},
};

/// Generic slice-like DST
#[derive(Debug)]
#[repr(C)]
pub struct SliceWithHeader<H, I> {
    pub header: H,
    pub slice: [I],
}

impl<H, I> SliceWithHeader<H, I> {
    #[inline]
    pub const fn len(&self) -> usize {
        dst_len(self)
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn fields(len: usize) -> [Layout; 2] {
        [Layout::new::<H>(), Layout::array::<I>(len).unwrap()]
    }
    fn layout_pack(len: usize) -> LayoutPack<2> {
        repr_c(Self::fields(len)).unwrap()
    }
    /// Create a new initializer
    unsafe fn new_init(header: H, init_tail: impl TailInit<I>) -> impl DstInit<Self> {
        unsafe { Self::initialize_for(write_fn(header), init_tail) }
    }
    unsafe fn from_raw_tail_init<A: AllocSliceDst<Target = Self>>(
        header: H,
        len: usize,
        init_tail: impl TailInit<I>,
    ) -> A {
        let init = unsafe { Self::new_init(header, init_tail) };
        unsafe { A::new_slice_dst(len, init) }
    }
    /// Create a new slice/header DST in a [`AllocSliceDst`] container.
    ///
    /// # Panics
    ///
    /// Panics if the items iterator incorrectly reports its length.
    #[allow(clippy::new_ret_no_self)]
    pub fn from_iter<A, J>(header: H, items: J) -> A
    where
        A: AllocSliceDst<Target = Self>,
        J: IntoIterator<Item = I>,
        J::IntoIter: ExactSizeIterator,
    {
        let items = items.into_iter();
        let len = items.len();
        let init_tail = write_slice_iter_fn(items);
        unsafe { Self::from_raw_tail_init(header, len, init_tail) }
    }
    /// Create a new slice/header DST from a slice, in a [`AllocSliceDst`] container.
    #[allow(clippy::new_ret_no_self)]
    pub fn from_slice_copy<A>(header: H, s: &[I]) -> A
    where
        A: AllocSliceDst<Target = Self>,
        I: Copy,
    {
        let len = s.len();
        let init_tail = write_slice_copied_fn(s);
        unsafe { Self::from_raw_tail_init(header, len, init_tail) }
    }
}

impl<H, I> Deref for SliceWithHeader<H, I> {
    type Target = H;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl<H, I> DerefMut for SliceWithHeader<H, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.header
    }
}

dst_cast_impl!(<H, I> for SliceWithHeader<H, I>);

unsafe impl<H, I> DstLayout for SliceWithHeader<H, I> {
    type Head = H;
    type Tail = I;
}

unsafe impl<H, I> SliceDstSaferInit for SliceWithHeader<H, I> {
    #[inline]
    fn head_offset(_this: &NonNull<Self>) -> usize {
        offset_of!(Self, header)
    }
    #[inline]
    fn tail_offset(this: &NonNull<Self>) -> usize {
        Self::layout_pack(dst_len(this.as_ptr())).1[1]
    }
}
