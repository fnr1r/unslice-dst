//! Layout calculation trait for slice-like DSTs
//!
//! Required for allocation.
//!
//! May depend on casting

pub(crate) use self::funcs::MaybeDstLayout;
pub use self::funcs::{layout_for_len, layout_for_ptr};

mod automatic;
mod funcs;
mod impls;
mod manual;

/// Bare minimum of information required for calculating DST [`Layout`]s
///
/// Slice-like DSTs can be dumbed down to having a header, of size known at
/// compile time (constant) and an array of items, the size of which is known
/// at runtime. Both of them also have an alignment.
///
/// This is enough to calculate the layout.
///
/// # Safety
///
/// This function MUST produce a valid layout, the same as the nightly
/// [`Layout::for_value_raw`].
///
/// [`Layout`]: core::alloc::Layout
/// [`Layout::for_value_raw`]: core::alloc::Layout::for_value_raw
pub unsafe trait DstLayout {
    /// The type of the data of constant size
    type Header;
    /// The type of the item of the 'slice'
    type Item;
}
