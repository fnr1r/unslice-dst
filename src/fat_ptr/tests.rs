use super::{SliceDstPointer, dst_len};
use crate::cast::{dst_cast_const, dst_cast_impl};

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

#[cfg(any(feature = "core_ffi_cstr_impl", feature = "std_ffi_cstr_impl"))]
mod cstr {
    #[cfg(feature = "core_ffi_cstr_impl")]
    use core::ffi::CStr;
    #[cfg(all(not(feature = "core_ffi_cstr_impl"), feature = "std_ffi_cstr_impl"))]
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

const EMPTY_ALIGN_FIX: [i32; 0] = [];
#[rustfmt::skip]
const TEST_DATA_M: [u8; 12] = [
    // n: i32 LE
    0x80, 0x00, 0x00, 0x00,
    // text: str
    b'h', b'e', b'l', b'l', b'o',
    // _padding
    0x00, 0x00, 0x00,
];
const TEST_DATA: &([i32; 0], [u8; 12]) = &(EMPTY_ALIGN_FIX, TEST_DATA_M);

#[derive(Debug)]
struct TestStruct {
    n: [u8; 4],
    text: str,
}

dst_cast_impl!(for TestStruct);

#[test]
fn assert_complex_const_dst_works() {
    let slice = TEST_DATA.1.as_ref();
    let data = dst_cast_const::<_, TestStruct>(slice);
    let mut data = SliceDstPointer::from_ptr(data);
    data.metadata = 5;
    let ptr = data.into_ptr::<TestStruct>();
    let data = unsafe { ptr.as_ref().unwrap_unchecked() };
    assert_eq!(i32::from_be_bytes(data.n), i32::MIN);
    assert_eq!(&data.text, "hello");
}
