use core::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::cast::dst_cast_nonnull;

/// [NonNull] but mut and valid for a set lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitMut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Deref for UninitMut<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: ?Sized> DerefMut for UninitMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: ?Sized> UninitMut<'a, T> {
    #[inline]
    pub(crate) const unsafe fn new(ptr: NonNull<T>) -> Self {
        Self(ptr, PhantomData)
    }
}

impl<'a, T> UninitMut<'a, T> {
    /// Convert self into the canonical Rust representation
    ///
    /// Possible here since T is [`Sized`].
    #[inline]
    pub const fn into_canonical(self) -> &'a mut MaybeUninit<T> {
        // SAFETY: The pointer is valid
        unsafe { self.0.cast().as_mut() }
    }
}

impl<'a, T> UninitMut<'a, [T]> {
    /// Convert self into the canonical Rust representation
    ///
    /// Possible here since T is a [`slice`](primitive@slice).
    #[inline]
    pub const fn into_canonical(self) -> &'a mut [MaybeUninit<T>] {
        // SAFETY: The pointer is valid
        unsafe { dst_cast_nonnull(self.0).as_mut() }
    }
}
