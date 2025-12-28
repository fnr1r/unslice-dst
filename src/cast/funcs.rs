//! WTF, clippy? We are not dereferencing pointers.

use core::ptr::NonNull;

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
