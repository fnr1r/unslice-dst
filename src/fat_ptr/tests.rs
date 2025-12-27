use super::{dst_len, SliceDstPointer};

const LEN: usize = 420;
const DATA: &[u8] = &[12; LEN];

fn slice_test_data() -> SliceDstPointer {
    SliceDstPointer::from_ptr(DATA)
}

#[test]
fn assert_slice_addr() {
    assert_ne!(slice_test_data().address, core::ptr::null());
}

#[test]
fn assert_slice_len() {
    assert_eq!(slice_test_data().metadata, LEN);
}

const TEST_SLICE: [i16; 32] = [69; 32];
const TEST_STR: &str = "I'm a potato";

#[test]
fn works_with_slice() {
    assert_eq!(dst_len(TEST_SLICE.as_ref()), TEST_SLICE.len());
}

#[test]
fn works_with_str() {
    assert_eq!(dst_len(TEST_STR), TEST_STR.len());
}

#[cfg(any(feature = "cast_impl_core_cstr", feature = "cast_impl_std_cstr",))]
mod cstr {
    #[cfg(feature = "cast_impl_core_cstr")]
    use core::ffi::CStr;
    #[cfg(all(not(feature = "cast_impl_core_cstr"), feature = "cast_impl_std_cstr",))]
    use std::ffi::CStr;

    use super::*;

    const TEST_CSTR_V: &str = "uiu uiu uiu, uiu uiu aaa\0";
    const TEST_CSTR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(TEST_CSTR_V.as_bytes()) };

    #[test]
    fn works_with_cstr() {
        // NOTE: Count bytes can't be used, since we need to include the null
        assert_eq!(dst_len(TEST_CSTR), TEST_CSTR.to_bytes_with_nul().len());
    }
}
