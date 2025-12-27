//! See [`SliceWithHeader`]

use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    marker::PhantomData,
    mem::offset_of,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "plain")]
use plain::Plain;

pub use self::config::*;
use crate::{
    AllocSliceDst, DstLayout, dst_cast_impl,
    initializers::{
        InitDst, InitTail, SliceDstSaferInit, write_fn, write_slice_copied_fn, write_slice_iter_fn,
    },
    uninit::UninitRef,
    utils::const_utils::{size_round_up, usize_max},
};

mod config;

/// Generic slice-like DST
#[allow(missing_docs)]
#[repr(C)]
pub struct SliceWithHeader<H, I, C = ()> {
    _config: PhantomData<C>,
    pub header: H,
    pub slice: [I],
}

impl<H: Debug, I: Debug, C> Debug for SliceWithHeader<H, I, C> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SliceWithHeader")
            .field("_config", &self._config)
            .field("header", &self.header)
            .field("slice", &&self.slice)
            .finish()
    }
}

impl<H, I, C> SliceWithHeader<H, I, C> {
    const HEAD_OFFSET: usize = offset_of!(Self, header);
    const TAIL_OFFSET: usize = {
        let align = usize_max(align_of::<H>(), align_of::<I>());
        size_round_up(size_of::<H>(), align)
    };
}

impl<H, I, C> SliceWithHeader<H, I, C> {
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
    /// Create a new slice/header DST from a slice, in a [`AllocSliceDst`]
    /// container.
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

impl<H, I, C: SliceWithHeaderDerefEnable> Deref for SliceWithHeader<H, I, C> {
    type Target = H;
    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl<H, I, C: SliceWithHeaderDerefEnable> DerefMut for SliceWithHeader<H, I, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.header
    }
}

dst_cast_impl!(<H, I, C> for SliceWithHeader<H, I, C>);

unsafe impl<H, I, C> DstLayout for SliceWithHeader<H, I, C> {
    type Head = H;
    type Tail = I;
}

unsafe impl<H, I, C> SliceDstSaferInit for SliceWithHeader<H, I, C> {
    #[inline]
    fn head_offset(_this: UninitRef<'_, Self>) -> usize {
        Self::HEAD_OFFSET
    }
    #[inline]
    fn tail_offset(_this: UninitRef<'_, Self>) -> usize {
        Self::TAIL_OFFSET
    }
}

#[cfg(all(test, miri, feature = "std"))]
mod tests {
    use std::panic::catch_unwind;

    use super::*;
    use crate::uninit::UninitMut;

    #[test]
    fn leak_test() {
        let init = |_: UninitMut<'_, [u8]>| panic!();
        let _ = catch_unwind(|| unsafe { Box::new_slice_dst(32, init) });
        // MIRI should catch any leaks here
    }
}

#[cfg(feature = "plain")]
unsafe impl<H: Plain, I: Plain, C> Plain for SliceWithHeader<H, I, C> {}
