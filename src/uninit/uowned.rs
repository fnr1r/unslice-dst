use core::ptr::NonNull;

use super::UninitMut;

/// The "owning" pointer for others. Doesn't handle deallocation.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct UninitOwner<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> UninitOwner<T> {
    #[inline]
    pub(crate) const fn new(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }
    #[inline]
    pub(crate) const fn into_inner(self) -> NonNull<T> {
        self.0
    }
    #[inline]
    pub(crate) const fn as_mut(&self) -> UninitMut<'_, T> {
        unsafe { UninitMut::new(self.0) }
    }
}
