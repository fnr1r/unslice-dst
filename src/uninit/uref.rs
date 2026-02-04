use core::{
    marker::PhantomData,
    mem::{MaybeUninit, transmute},
    ops::Deref,
    ptr::NonNull,
};

use crate::cast::dst_cast_nonnull;

/// [NonNull] but valid for a set lifetime.
///
/// Not very useful. Not valid for reads (unless you can assume otherwhise) or
/// writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitRef<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T: ?Sized> Deref for UninitRef<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<'a, T> UninitRef<'a, T> {
    /// Convert self into the canonical Rust representation
    ///
    /// Possible here since T is [`Sized`].
    #[inline]
    pub const fn into_canonical(self) -> &'a MaybeUninit<T> {
        // SAFETY: The pointer is valid
        unsafe { self.0.cast().as_ref() }
    }
}

impl<'a, T> UninitRef<'a, [T]> {
    /// Convert self into the canonical Rust representation
    ///
    /// Possible here since T is a [`slice`](primitive@slice).
    #[inline]
    pub const fn into_canonical(self) -> &'a [MaybeUninit<T>] {
        // SAFETY: The pointer is valid
        unsafe { dst_cast_nonnull(self.0).as_ref() }
    }
}
