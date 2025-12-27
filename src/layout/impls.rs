use super::DstLayout;

unsafe impl<T> DstLayout for [T] {
    type Header = ();
    type Item = T;
}

unsafe impl DstLayout for str {
    type Header = ();
    type Item = u8;
}

unsafe impl DstLayout for core::ffi::CStr {
    type Header = ();
    type Item = u8;
}

#[cfg(all(feature = "std", unix))]
mod for_std {
    use std::ffi::OsStr;

    use super::*;

    unsafe impl DstLayout for OsStr {
        type Header = ();
        type Item = u8;
    }

    unsafe impl DstLayout for std::path::Path {
        type Header = ();
        type Item = u8;
    }
}
