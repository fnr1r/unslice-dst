//! WTF, clippy? We are not dereferencing pointers.

use core::ptr::{NonNull, slice_from_raw_parts, slice_from_raw_parts_mut};

use super::DstCast;
use crate::utils::transmute_lax;

/// [`DstCast::cast_dst_const`] but const
///
/// # Safety
///
/// [`DstCast`] guarantees that both fat pointers have the same metadata
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
pub const fn dst_cast_const<T, U>(src: *const T) -> *const U
where
    T: ?Sized + DstCast,
    U: ?Sized + DstCast,
{
    unsafe { transmute_lax::<*const T, *const U>(src) }
}

/// [`DstCast::cast_dst_mut`] but const
///
/// # Safety
///
/// [`DstCast`] guarantees that both fat pointers have the same metadata
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
pub const fn dst_cast_mut<T, U>(src: *mut T) -> *mut U
where
    T: ?Sized + DstCast,
    U: ?Sized + DstCast,
{
    unsafe { transmute_lax::<*mut T, *mut U>(src) }
}

/// Cast a [`NonNull`] pointer from slice-like DST `T` to slice-like DST `U`
///
/// [`dst_cast_mut`] but for [`NonNull`]
///
/// # Safety
///
/// [`DstCast`] guarantees that both fat pointers have the same metadata
#[inline]
pub const fn dst_cast_nonnull<T, U>(src: NonNull<T>) -> NonNull<U>
where
    T: ?Sized + DstCast,
    U: ?Sized + DstCast,
{
    unsafe { transmute_lax::<NonNull<T>, NonNull<U>>(src) }
}

/// [`slice_from_raw_parts`] for slice-like DSTs
#[inline]
pub const fn dst_from_raw_parts<T, U>(data: *const T, len: usize) -> *const U
where
    U: ?Sized + DstCast,
{
    dst_cast_const(slice_from_raw_parts(data, len))
}

/// [`slice_from_raw_parts_mut`] for slice-like DSTs
#[inline]
pub const fn dst_from_raw_parts_mut<T, U>(data: *mut T, len: usize) -> *mut U
where
    U: ?Sized + DstCast,
{
    dst_cast_mut(slice_from_raw_parts_mut(data, len))
}

/// Create a [`NonNull`] pointer to a slice-like DST from a thin pointer and
/// length.
#[inline]
pub const fn dst_from_raw_parts_nonnull<T, U>(data: NonNull<T>, len: usize) -> NonNull<U>
where
    U: ?Sized + DstCast,
{
    use crate::utils::slice::slice_from_raw_parts_nonnull;
    dst_cast_nonnull(slice_from_raw_parts_nonnull(data, len))
}
