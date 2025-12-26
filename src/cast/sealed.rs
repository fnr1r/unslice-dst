//! Sealed type for [`DstCast`]
//!
//! # WARNING
//!
//! DO NOT IMPLEMENT [`DstCast`] ON YOUR OWN!
//!
//! [`DstCast`]: super::DstCast

use core::{marker::PhantomData, mem::size_of};

use crate::AnyDst;

/// Seal type. Guarantees that the pointer size of `T` matches [`AnyDst`].
///
/// Can only be constructed with [`assert_fat_ptr_matches_size_dst`].
#[derive(Debug)]
pub struct Sealed<T: ?Sized>(PhantomData<T>);

/// Checks if the size of the pointer to any type T matches [`AnyDst`]
pub const fn fat_ptr_matches_slice_dst<T: ?Sized>() -> bool {
    size_of::<*const T>() == size_of::<*const AnyDst>()
}

/// Asserts that the size of the pointer to any type T matches [`AnyDst`]
///
/// Panics if [`fat_ptr_matches_slice_dst`] returned false.
pub const fn assert_fat_ptr_matches_size_dst<T: ?Sized>() -> Sealed<T> {
    // TODO: Add assertion message once core::any::type_name::<T>() is stable
    assert!(fat_ptr_matches_slice_dst::<T>());
    Sealed::<T>(PhantomData)
}
