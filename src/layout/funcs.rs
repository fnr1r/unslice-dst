use core::alloc::{Layout, LayoutError};

use super::DstLayout;

const fn layout_try_for_len<T: ?Sized + DstLayout>(len: usize) -> Result<Layout, LayoutError> {
    let hlayout = Layout::new::<T::Header>();
    let ilayout = match Layout::array::<T::Item>(len) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let layout = match hlayout.extend(ilayout) {
        Ok((res, _)) => res,
        Err(e) => return Err(e),
    };
    Ok(layout.pad_to_align())
}

/// Produces layout describing a record that could be used to allocate backing
/// structure for slice-like DST `T`.
///
/// See [`Layout::for_value_raw`]
#[allow(unused)]
pub const fn layout_for_len<T: ?Sized + DstLayout>(len: usize) -> Layout {
    match layout_try_for_len::<T>(len) {
        Ok(res) => res,
        Err(_) => panic!("layout doesn't fit"),
    }
}
