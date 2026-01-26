use alloc::boxed::Box;
use core::ptr::NonNull;

use crate::{allocation::AllocSliceDst, utils::rand::musl_rand_with_iter};

fn acq_init_data() -> Box<[f32; 42]> {
    let mut res = Box::new([0.; 42]);
    musl_rand_with_iter(|rand| res.fill_with(|| rand.next_inf() as _));
    res
}

#[test]
fn assert_works_for_slice() {
    let initial_data = acq_init_data();
    let init = |mut ptr: NonNull<[f32]>| {
        unsafe { ptr.as_mut() }.copy_from_slice(initial_data.as_ref());
    };
    let data = unsafe { <Box<[f32]>>::new_slice_dst(42, init) };
    assert_eq!(*initial_data, *data);
}
