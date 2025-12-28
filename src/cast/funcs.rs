//! WTF, clippy? We are not dereferencing pointers.

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
