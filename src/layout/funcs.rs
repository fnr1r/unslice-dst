use core::alloc::Layout;

#[cfg(not(feature = "layout_automatic"))]
use super::DstLayout;
#[cfg(feature = "layout_automatic")]
use crate::DstCast;
use crate::SliceDst;

#[cfg(feature = "layout_automatic")]
pub trait MaybeDstLayout: DstCast {}
#[cfg(not(feature = "layout_automatic"))]
pub trait MaybeDstLayout: DstLayout {}

#[cfg(feature = "layout_automatic")]
impl<T: ?Sized + DstCast> MaybeDstLayout for T {}
#[cfg(not(feature = "layout_automatic"))]
impl<T: ?Sized + DstLayout> MaybeDstLayout for T {}

/// Produces layout describing a record that could be used to allocate backing
/// structure for slice-like DST `T`.
///
/// # Note
///
/// Depending on feature flags, it may use a different implementation.
#[inline]
pub const fn layout_for_len<T>(len: usize) -> Layout
where
    T: ?Sized + MaybeDstLayout,
{
    #[cfg(feature = "layout_automatic")]
    use super::automatic::layout_for_len as inner;
    #[cfg(not(feature = "layout_automatic"))]
    use super::manual::layout_for_len as inner;
    inner::<T>(len)
}

/// Produces layout describing a record that could be used to allocate backing
/// structure for slice-like DST `T`.
///
/// (but for pointers)
#[inline]
pub const fn layout_for_ptr<T>(ptr: *const T) -> Layout
where
    T: ?Sized + SliceDst,
{
    #[cfg(feature = "layout_automatic")]
    use super::automatic::layout_for_ptr as inner;
    #[cfg(not(feature = "layout_automatic"))]
    use super::manual::layout_for_ptr as inner;
    inner::<T>(ptr)
}
