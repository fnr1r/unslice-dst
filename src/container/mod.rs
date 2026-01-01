//! Smart pointer compatibility trait
//!
//! Depends on `alloc` (crate).
//!
//! See [`DstContainer`] for more.

use core::{alloc::Layout, ptr::NonNull};

use crate::SliceDst;

mod impl_box;

/// Any smart pointer which can contain a DST
///
/// Avoid using this outside of the `alloc` module. This is supposed to be a
/// part of the life cycle of creating a DST.
///
/// [`Self`] must be a smart pointer to type T, possibly with some header.
///
/// See implementation for [`Box`].
///
/// [`Box`]: alloc::boxed::Box
#[allow(clippy::missing_safety_doc)]
#[cfg_attr(not(feature = "container_unseal"), allow(dead_code, unreachable_pub))]
pub unsafe trait DstContainer: Sized {
    /// The data behind the smart pointer.
    ///
    /// Restricted to only [`SliceDst`] types since it avoids ugly trait bounds.
    type Target: ?Sized + SliceDst;
    /// Additional info about the allocation which may be needed for
    /// deallocation.
    ///
    /// Also passed into finalize for future proofing.
    ///
    /// Usually, as per the Rust convention, it's [`Layout`].
    type Context;
    /// Allocate `T` and return a [`NonNull`] pointer to it.
    ///
    /// We can't return [`Self`] here because the data inside the smart pointer
    /// is uninitialized and [`MaybeUninit`] doesn't support `?Sized` types.
    ///
    /// For [`Box`] it means just a cast. For `(A)Rc` though it means skipping
    /// the header which contains the reference count.
    ///
    /// `layout` is allowed to be of size 0.
    ///
    /// The [`NonNull`] pointer is guaranteed to be valid, but uninitialized.
    ///
    /// [`MaybeUninit`]: core::mem::MaybeUninit
    unsafe fn dst_allocate(layout: Layout) -> (NonNull<()>, Self::Context);
    /// Turn the [`NonNull`] pointer into [`Self`].
    ///
    /// This is the "success" branch.
    ///
    /// The [`NonNull`] pointer must be one returned from [`dst_allocate`],
    /// valid and initialized.
    ///
    /// [`dst_allocate`]: Self::dst_allocate
    unsafe fn dst_finalize(ptr: NonNull<Self::Target>, ctx: Self::Context) -> Self;
    /// Consume the [`NonNull`] pointer and deallocate it.
    ///
    /// This is the "failure" branch.
    ///
    /// The [`NonNull`] pointer must be one returned from [`dst_allocate`],
    /// valid and uninitialized (likely partially [`drop`]ped).
    ///
    /// [`dst_allocate`]: Self::dst_allocate
    unsafe fn dst_dealloc(ptr: NonNull<Self::Target>, ctx: Self::Context);
}
