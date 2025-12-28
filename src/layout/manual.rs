use core::alloc::{Layout, LayoutError};

use super::DstLayout;
use crate::{DstCast, dst_len};

const fn layout_try_for_len<T: ?Sized + DstLayout>(len: usize) -> Result<Layout, LayoutError> {
    let hlayout = Layout::new::<T::Head>();
    let ilayout = match Layout::array::<T::Tail>(len) {
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
pub(super) const fn layout_for_len<T: ?Sized + DstLayout>(len: usize) -> Layout {
    match layout_try_for_len::<T>(len) {
        Ok(res) => res,
        Err(_) => panic!("layout doesn't fit"),
    }
}

#[cfg_attr(feature = "layout_automatic", allow(dead_code))]
pub(super) const fn layout_for_ptr<T>(ptr: *const T) -> Layout
where
    T: ?Sized + DstCast + DstLayout,
{
    layout_for_len::<T>(dst_len(ptr))
}

#[test]
fn assert_slice() {
    type A = i16;
    type B = [A];
    let len = 19;
    assert_eq!(layout_for_len::<B>(len), Layout::array::<A>(len).unwrap());
}
