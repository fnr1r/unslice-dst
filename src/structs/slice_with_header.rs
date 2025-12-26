use crate::{DstLayout, dst_cast_impl};

/// Generic slice-like DST
#[allow(missing_docs)]
#[derive(Debug)]
#[repr(C)]
pub struct SliceWithHeader<H, I> {
    pub header: H,
    pub slice: [I],
}

dst_cast_impl!(<H, I> for SliceWithHeader<H, I>);

unsafe impl<H, I> DstLayout for SliceWithHeader<H, I> {
    type Head = H;
    type Tail = I;
}
