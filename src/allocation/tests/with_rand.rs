use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::ops::Deref;

use crate::{allocation::AllocSliceDst, uninit::UninitMut, utils::rand::musl_rand_with_iter};

fn acq_init_data() -> Box<[f32; 42]> {
    let mut res = Box::new([0.; 42]);
    musl_rand_with_iter(|rand| res.fill_with(|| rand.next_inf() as _));
    res
}

fn do_check_works<T: AllocSliceDst<Target = [f32]> + Deref<Target = [f32]>>() {
    let initial_data = acq_init_data();
    let init = |mut ptr: UninitMut<'_, [f32]>| {
        unsafe { ptr.as_mut() }.copy_from_slice(initial_data.as_ref());
    };
    let data = unsafe { T::new_slice_dst(42, init) };
    assert_eq!(*initial_data, *data);
}

#[test]
fn assert_works_for_box_slice() {
    do_check_works::<Box<_>>();
}

#[test]
fn assert_works_for_rc_slice() {
    do_check_works::<Rc<_>>();
}

#[test]
fn assert_works_for_arc_slice() {
    do_check_works::<Arc<_>>();
}
