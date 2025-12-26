use super::DstCast;

#[test]
fn assert_cast_works() {
    let a = [69i8; 32];
    let b = <[u8]>::cast_dst_const(a.as_ref());
    // SAFETY: the pointer is created from a valid reference to `a`
    //   the types are compatible in terms of size and as-cast-able
    let b = unsafe { b.as_ref().unwrap_unchecked() };
    for (c, d) in a.iter().zip(b) {
        assert_eq!(Ok(*c), i8::try_from(*d));
    }
}
