use core::ops::{Deref, DerefMut};

use crate::{AnyDst, DstLayout, dst_cast_impl};

/// "Fake" DST
///
/// Unlike other DSTs, this can be easily stored on the stack, since the 2nd
/// field is a ZST. Although the compiler doesn't let me set the repr to
/// `transparent`, since [`AnyDst`] is [`?Sized`].
#[derive(Debug)]
#[repr(C)]
pub struct MockDst<T>(pub T, AnyDst);

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
