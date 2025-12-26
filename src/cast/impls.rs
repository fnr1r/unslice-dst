use crate::dst_cast_impl;

dst_cast_impl!(<T> for [T]);
dst_cast_impl!(for str);

#[cfg(feature = "cast_impl_core_cstr")]
dst_cast_impl!(for core::ffi::CStr);

#[cfg(feature = "std")]
dst_cast_impl! {
    for std::ffi::OsStr;
    for std::path::Path;
}
