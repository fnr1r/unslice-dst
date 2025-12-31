use crate::dst_cast_impl;

dst_cast_impl!(<T> for [T]);
dst_cast_impl!(for str);

#[cfg(feature = "cast_impl_core_cstr")]
dst_cast_impl!(for core::ffi::CStr);
