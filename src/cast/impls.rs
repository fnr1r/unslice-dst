use crate::dst_cast_impl;

dst_cast_impl!(<T> for [T]);
dst_cast_impl!(for str);
// NOTE: not adding an impl for CStr
//   1. It bumps the MSRV for cast to 1.64
//   2. Its layout is implementation detail
// Maybe put this behind a feature flag.
