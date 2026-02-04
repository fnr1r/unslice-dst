use core::{
    marker::PhantomData,
    mem::{MaybeUninit, transmute},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use super::FieldInfo;
use crate::{
    DstCast,
    cast::dst_cast_nonnull,
    utils::slice::{slice_as_uninit, slice_assume_init_mut},
};

/// [NonNull] but mut (invariant) and valid for a set lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitMut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Deref for UninitMut<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&Self, &NonNull<T>>(self) }
    }
}

impl<'a, T: ?Sized> DerefMut for UninitMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut Self, &mut NonNull<T>>(self) }
    }
}

impl<'a, T: ?Sized> UninitMut<'a, T> {
    pub(super) const unsafe fn new(ptr: NonNull<T>) -> Self {
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
    /// Initialize the [`slice`] from the passed [`slice`].
    ///
    /// Returns a valid reference to the newly initialized value.
    ///
    /// This is a common enough operation to warrant an implementation.
    ///
    /// [`slice`]: primitive@slice
    #[inline]
    pub fn copy_from_slice(self, src: &[T]) -> &'a mut [T]
    where
        T: Copy,
    {
        let this = self.into_canonical();
        this.copy_from_slice(slice_as_uninit(src));
        // SAFETY: this is initialized by the above call
        unsafe { slice_assume_init_mut(this) }
    }
}

impl<'a, T: ?Sized + DstCast> UninitMut<'a, T> {
    /// Transform the pointer based on the provided field metadata
    #[inline]
    pub const fn transform_sized<U>(self, info: FieldInfo<T, U>) -> UninitMut<'a, U> {
        UninitMut(unsafe { info.transform_sized(self.0) }, PhantomData)
    }
    /// Transform the pointer based on the provided field metadata
    #[inline]
    pub const fn transform_dst<U>(self, info: FieldInfo<T, U>) -> UninitMut<'a, U>
    where
        U: ?Sized + DstCast,
    {
        UninitMut(unsafe { info.transform_dst(self.0) }, PhantomData)
    }
}
