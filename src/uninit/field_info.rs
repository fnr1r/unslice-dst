use core::{marker::PhantomData, ptr::NonNull};

use crate::{DstCast, cast::dst_cast_nonnull};

/// Field metadata for parent type `T`
///
/// # TODO
///
/// - How to handle DSTs with different offsets per length? Unless I add an
///   variant ref I can't enforce the usage of this type for the allocation.
#[derive(Debug)]
pub struct FieldInfo<T: ?Sized, U: ?Sized> {
    offset: usize,
    parent: PhantomData<T>,
    child: PhantomData<U>,
}

impl<T: ?Sized, U: ?Sized> Copy for FieldInfo<T, U> {}

impl<T: ?Sized, U: ?Sized> Clone for FieldInfo<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized, U: ?Sized> FieldInfo<T, U> {
    /// Create a new blah blah blah TODO
    ///
    /// # Safety
    ///
    /// The offset must be a valid offset of the field.
    pub const unsafe fn new(offset: usize) -> Self {
        Self {
            offset,
            parent: PhantomData,
            child: PhantomData,
        }
    }
}

impl<T: ?Sized + DstCast, U> FieldInfo<T, U> {
    /// Transform a valid pointer to type T to its specified field
    pub(super) const unsafe fn transform_sized(&self, ptr: NonNull<T>) -> NonNull<U> {
        let ptr = ptr.cast::<U>().as_ptr();
        unsafe { NonNull::new_unchecked(ptr.byte_add(self.offset)) }
    }
}

impl<T: ?Sized + DstCast, U: ?Sized + DstCast> FieldInfo<T, U> {
    /// Transform a valid pointer to type T to its specified field
    pub(super) const unsafe fn transform_dst(&self, ptr: NonNull<T>) -> NonNull<U> {
        let ptr = dst_cast_nonnull::<T, U>(ptr).as_ptr();
        unsafe { NonNull::new_unchecked(ptr.byte_add(self.offset)) }
    }
}
