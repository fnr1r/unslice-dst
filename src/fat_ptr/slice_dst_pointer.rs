use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    mem::size_of,
    ops::{Deref, DerefMut},
};

use super::PointerWith;
use crate::{utils::transmute_lax, AnyDst, DstCast};

pub(super) type Inner = PointerWith<usize>;

// Some static assertions. The layout is expected to be two usize-s
const _: () = assert!(size_of::<usize>() * 2 == size_of::<*const AnyDst>());
const _: () = assert!(size_of::<Inner>() == size_of::<*const AnyDst>());

/// Struct representation of a fat pointer to a slice-like DST.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SliceDstPointer(pub Inner);

impl SliceDstPointer {
    /// Creates a [`SliceDstPointer`] from a [`PointerWith<usize>`]
    #[inline]
    pub const fn new(inner: Inner) -> Self {
        Self(inner)
    }
    /// Creates a [`SliceDstPointer`] from an address and metadata
    #[inline]
    pub const fn from_raw_parts<T>(address: *const T, metadata: usize) -> Self {
        let address = address.cast();
        Self(PointerWith { address, metadata })
    }
    /// Extracts the inner value
    #[inline]
    pub const fn into_inner(self) -> Inner {
        self.0
    }
    /// Creates a [`SliceDstPointer`] from a raw fat pointer to a DST.
    ///
    /// # Safety
    ///
    /// The safety is guaranteed by the [`DstCast`] trait.
    ///
    /// # Panics
    ///
    /// In debug builds, panics if the size of `*const T` does not match the
    /// size of `SliceDstPointer`. This is an internal consistency check.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline]
    pub const fn from_ptr<T: ?Sized + DstCast>(ptr: *const T) -> Self {
        debug_assert!(size_of::<*const T>() == size_of::<Self>());
        // SAFETY: `DstCast` requires a fat pointer with usize metadata
        let inner = unsafe { transmute_lax::<*const T, Inner>(ptr) };
        Self::new(inner)
    }
    /// Reconstructs a raw fat pointer to a DST from this [`SliceDstPointer`].
    ///
    /// This function takes the address and metadata stored within `self` and
    /// combines them back into a raw fat pointer `*const T`.
    ///
    /// # Safety
    ///
    /// The safety is guaranteed by the [`DstCast`] trait.
    ///
    /// Also see the docs for [pointers](*const).
    ///
    /// # Panics
    ///
    /// In debug builds, panics if the size of `*const T` does not match the
    /// size of `SliceDstPointer`. This is an internal consistency check.
    #[inline]
    pub const fn into_ptr<T: ?Sized + DstCast>(self) -> *const T {
        debug_assert!(size_of::<*const T>() == size_of::<Self>());
        // SAFETY: `DstCast` requires a fat pointer with usize metadata
        unsafe { transmute_lax::<Inner, *const T>(self.into_inner()) }
    }
    /// Returns the number of elements in the slice.
    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.metadata
    }
}

impl Debug for SliceDstPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Deref for SliceDstPointer {
    type Target = PointerWith<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SliceDstPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Inner> for SliceDstPointer {
    fn from(value: Inner) -> Self {
        Self::new(value)
    }
}

impl<T: ?Sized + DstCast> From<*const T> for SliceDstPointer {
    #[inline]
    fn from(value: *const T) -> Self {
        Self::from_ptr(value)
    }
}
