use core::ops::{Deref, DerefMut};

use crate::{DstLayout, dst_cast_impl, dst_len};

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
