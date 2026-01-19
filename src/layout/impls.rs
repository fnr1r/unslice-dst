use super::DstLayout;

unsafe impl<T> DstLayout for [T] {
    type Head = ();
    type Tail = T;
}

unsafe impl DstLayout for str {
    type Head = ();
    type Tail = u8;
}

#[cfg(feature = "core_ffi_cstr_impl")]
unsafe impl DstLayout for core::ffi::CStr {
    type Head = ();
    type Tail = u8;
}

#[cfg(feature = "std")]
mod for_std {
    use std::ffi::OsStr;

    use super::*;

    /// # Implementation notes
    ///
    /// - On Windows, it's Wtf8, backed by slice of u8
    /// - On Motor, it's Utf8, which doesn't matter for us
    /// - Otherwise, it's just a slice of u8
    unsafe impl DstLayout for OsStr {
        type Head = ();
        type Tail = u8;
    }

    unsafe impl DstLayout for std::path::Path {
        type Head = ();
        type Tail = <OsStr as DstLayout>::Tail;
    }
}

#[cfg(all(not(feature = "core_ffi_cstr_impl"), feature = "std_ffi_cstr_impl"))]
unsafe impl DstLayout for std::ffi::CStr {
    type Head = ();
    type Tail = u8;
}
