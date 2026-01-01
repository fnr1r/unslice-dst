//! Fat pointer utils for slice-like DSTs
//!
//! Depends on the casting, fat_ptr and layout module.

use core::{convert::Infallible, ptr::NonNull};

use self::uninit_box::UninitBox;
use crate::container::DstContainer;

mod uninit_box;

/// Types that can allocate a custom slice DST within them,
/// given a fallible initialization function.
///
/// # TODO
///
/// Fallible allocation APIs... someday.
///
/// # Safety
///
/// Must be implemented as described and may be relied upon by generic code.
pub unsafe trait AllocSliceDst: DstContainer {
    /// Create a new custom slice DST with a fallible initialization function.
    ///
    /// # Safety
    ///
    /// `init` must properly initialize the object behind the pointer.
    /// `init` receives a fully uninitialized pointer and must not read anything before writing.
    ///
    /// If the initialization closure panics or returns an error,
    /// the allocated place will be deallocated but not dropped.
    /// To clean up the partially initialized type, we suggest
    /// proxying creation through scope guarding types.
    #[inline]
    unsafe fn try_new_slice_dst<I, E>(len: usize, init: I) -> Result<Self, E>
    where
        I: FnOnce(NonNull<Self::Target>) -> Result<(), E>,
    {
        let ptr = UninitBox::alloc_for_len(len);
        init(ptr.as_ptr())?;
        Ok(ptr.finalize())
    }
    /// Create a new custom slice DST.
    ///
    /// # Safety
    ///
    /// `init` must properly initialize the object behind the pointer.
    /// `init` receives a fully uninitialized pointer and must not read anything before writing.
    #[inline]
    unsafe fn new_slice_dst<I>(len: usize, init: I) -> Self
    where
        I: FnOnce(NonNull<Self::Target>),
    {
        let init = |ptr| {
            init(ptr);
            Ok::<(), Infallible>(())
        };
        unsafe { Self::try_new_slice_dst(len, init).unwrap_unchecked() }
    }
}

unsafe impl<T: DstContainer> AllocSliceDst for T {}
