use core::mem::offset_of;

use crate::{
    AllocSliceDst, DstLayout, dst_cast_impl,
    initializers::{
        InitDst, InitTail, SliceDstSaferInit, write_fn, write_slice_copied_fn, write_slice_iter_fn,
    },
    uninit::UninitRef,
    utils::const_utils::{size_round_up, usize_max},
};

/// Generic slice-like DST
#[allow(missing_docs)]
#[derive(Debug)]
#[repr(C)]
pub struct SliceWithHeader<H, I> {
    pub header: H,
    pub slice: [I],
}

impl<H, I> SliceWithHeader<H, I> {
    const HEAD_OFFSET: usize = offset_of!(Self, header);
    const TAIL_OFFSET: usize = {
        let align = usize_max(align_of::<H>(), align_of::<I>());
        size_round_up(size_of::<H>(), align)
    };
}

impl<H, I> SliceWithHeader<H, I> {
    /// Create a new initializer
    unsafe fn new_init(header: H, init_tail: impl InitTail<Self>) -> impl InitDst<Self> {
        unsafe { Self::initialize_for(write_fn(header), init_tail) }
    }
    unsafe fn from_raw_tail_init<A: AllocSliceDst<Target = Self>>(
        header: H,
        len: usize,
        init_tail: impl InitTail<Self>,
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

dst_cast_impl!(<H, I> for SliceWithHeader<H, I>);

unsafe impl<H, I> DstLayout for SliceWithHeader<H, I> {
    type Head = H;
    type Tail = I;
}

unsafe impl<H, I> SliceDstSaferInit for SliceWithHeader<H, I> {
    #[inline]
    fn head_offset(_this: UninitRef<'_, Self>) -> usize {
        Self::HEAD_OFFSET
    }
    #[inline]
    fn tail_offset(_this: UninitRef<'_, Self>) -> usize {
        Self::TAIL_OFFSET
    }
}
