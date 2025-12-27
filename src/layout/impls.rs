use super::DstLayout;

unsafe impl<T> DstLayout for [T] {
    type Head = ();
    type Tail = T;
}

unsafe impl DstLayout for str {
    type Head = ();
    type Tail = u8;
}
