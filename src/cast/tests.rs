use super::{dst_cast_const, DstCast};

const TEST_DATA: &[u8] = &[130; 32];
const TEST_RESULT: isize = -126;

fn inner(f: impl Fn(&[u8]) -> &[i8]) {
    let data_retyped = f(TEST_DATA);
    for i in data_retyped.iter().copied() {
        assert_eq!(i as isize, TEST_RESULT);
    }
}

#[test]
fn assert_cast_works() {
    inner(|value| {
        let b = <[i8]>::cast_dst_const(value);
        // SAFETY: the pointer is created from a valid reference to `a`
        //   the types are compatible in terms of size and as-cast-able
        unsafe { b.as_ref().unwrap_unchecked() }
    });
}

#[test]
fn assert_const_cast_works() {
    inner(|value| {
        let b = dst_cast_const::<[u8], [i8]>(value);
        // SAFETY: the pointer is created from a valid reference to `a`
        //   the types are compatible in terms of size and as-cast-able
        unsafe { b.as_ref().unwrap_unchecked() }
    });
}
