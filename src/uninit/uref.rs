use core::{
    cmp::Ordering,
    fmt::{Debug, Formatter, Pointer, Result as FmtResult},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::Deref,
    ptr::NonNull,
};

use crate::cast::dst_cast_nonnull;

/// [NonNull] but valid for a set lifetime.
///
/// Not very useful. Not valid for reads (unless you can assume otherwhise) or
/// writes.
#[repr(transparent)]
pub struct UninitRef<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T: ?Sized> Debug for UninitRef<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Pointer::fmt(&self, f)
    }
}

impl<'a, T: ?Sized> Copy for UninitRef<'a, T> {}

impl<'a, T: ?Sized> Clone for UninitRef<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized> PartialEq for UninitRef<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<'a, T: ?Sized> Eq for UninitRef<'a, T> {}

impl<'a, T: ?Sized> PartialOrd for UninitRef<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<'a, T: ?Sized> Ord for UninitRef<'a, T> {
    #[allow(ambiguous_wide_pointer_comparisons)]
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a, T: ?Sized> Deref for UninitRef<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: ?Sized> UninitRef<'a, T> {
    #[inline]
    pub(crate) const unsafe fn new(ptr: NonNull<T>) -> Self {
        Self(ptr, PhantomData)
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
