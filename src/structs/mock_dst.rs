use core::ops::{Deref, DerefMut};

use crate::{
    AllocSliceDst, AnyDst, DstLayout,
    cast::{dst_from_raw_parts, dst_from_raw_parts_mut},
    dst_cast_impl,
    uninit::UninitMut,
};

/// "Fake" DST
///
/// Unlike other DSTs, this can be easily stored on the stack, since the 2nd
/// field is a ZST. Although the compiler doesn't let me set the repr to
/// `transparent`, since [`AnyDst`] is [`?Sized`](Sized).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct MockDst<T>(pub T, AnyDst);

impl<T> MockDst<T> {
    #[inline]
    const fn dst_init(ptr: UninitMut<'_, Self>, value: T) {
        ptr.cast_sized::<T>().into_canonical().write(value);
    }
    /// Allocate a new fake DST
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new<A: AllocSliceDst<Target = Self>>(value: T, len: usize) -> A {
        let init = |ptr: UninitMut<'_, _>| Self::dst_init(ptr, value);
        unsafe { A::new_slice_dst(len, init) }
    }
    /// Unsize a reference to `T`
    #[inline]
    pub const fn from_ref(value: &T, len: usize) -> &Self {
        let ptr = dst_from_raw_parts::<_, Self>(value, len);
        unsafe { ptr.as_ref().unwrap_unchecked() }
    }
    /// Unsize a mutable reference to `T`
    #[inline]
    pub const fn from_mut(value: &mut T, len: usize) -> &mut Self {
        let ptr = dst_from_raw_parts_mut::<_, Self>(value, len);
        unsafe { ptr.as_mut().unwrap_unchecked() }
    }
}

impl<T> Deref for MockDst<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MockDst<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

dst_cast_impl!(<T> for MockDst<T>);

unsafe impl<T> DstLayout for MockDst<T> {
    type Head = T;
    type Tail = ();
}
