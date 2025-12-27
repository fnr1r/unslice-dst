use core::alloc::Layout;

use super::{DstLayout, layout_for_len};

#[derive(Debug)]
#[repr(C)]
struct ExampleDst<const N: usize> {
    f1: u8,
    f2: (),
    f3: [f64; N],
    f4: [i16],
}

unsafe impl<const N: usize> DstLayout for ExampleDst<N> {
    type Head = (u8, (), [f64; N]);
    type Tail = i16;
}

const CORRECT_LAYOUTS_N0: &[(usize, Layout)] = &[
    (0, unsafe { Layout::from_size_align_unchecked(0x8, 0x8) }),
    (1, unsafe { Layout::from_size_align_unchecked(0x10, 0x8) }),
    (2, unsafe { Layout::from_size_align_unchecked(0x10, 0x8) }),
];

#[test]
fn assert_layouts_correct_n0() {
    for (len, layout) in CORRECT_LAYOUTS_N0 {
        let res = layout_for_len::<ExampleDst<0>>(*len);
        assert_eq!(&res, layout, "len={} layout invalid", len);
    }
}

const CORRECT_LAYOUTS_N1: &[(usize, Layout)] = &[
    (0, unsafe { Layout::from_size_align_unchecked(0x10, 0x8) }),
    (1, unsafe { Layout::from_size_align_unchecked(0x18, 0x8) }),
    (2, unsafe { Layout::from_size_align_unchecked(0x18, 0x8) }),
];

#[test]
fn assert_layouts_correct_n1() {
    for (len, layout) in CORRECT_LAYOUTS_N1 {
        let res = layout_for_len::<ExampleDst<1>>(*len);
        assert_eq!(&res, layout, "len={} layout invalid", len);
    }
}
