use core::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Pointer, Result as FmtResult},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    cast::dst_cast_nonnull,
    uninit::UninitRef,
    utils::slice::{slice_as_uninit, slice_assume_init_mut},
};

/// [NonNull] but mut and valid for a set lifetime.
#[repr(transparent)]
pub struct UninitMut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Debug for UninitMut<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&self, f)
    }
}

impl<'a, T: ?Sized> Copy for UninitMut<'a, T> {}

impl<'a, T: ?Sized> Clone for UninitMut<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> PartialEq for UninitMut<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<'a, T: ?Sized> Eq for UninitMut<'a, T> {}

impl<'a, T: ?Sized> PartialOrd for UninitMut<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<'a, T: ?Sized> Ord for UninitMut<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

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
    #[inline]
    pub(crate) const fn into_ref(self) -> UninitRef<'a, T> {
        unsafe { UninitRef::new(self.0) }
    }
    /// TODO: move this out
    #[inline]
    pub(crate) const fn cast<U>(self) -> UninitMut<'a, U> {
        unsafe { UninitMut::new(self.0.cast()) }
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
    #[inline]
    pub(crate) const fn write(self, val: T) -> &'a mut T {
        self.into_canonical().write(val)
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
