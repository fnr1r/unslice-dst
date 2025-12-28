use core::mem::{align_of, size_of, ManuallyDrop};

union TransmuteLax<A, B> {
    a: ManuallyDrop<A>,
    b: ManuallyDrop<B>,
}

/// Reinterprets the bits of a value of one type as another type.
///
/// Both types must have the same size. Unlike in [`transmute`], this is not
/// guaranteed. We do still `assert` it is the same though.
///
/// [`transmute`] also forbids casting fat pointers, but we have a trait that
/// guarantees the fat pointer size, so... this is just unavoidable for
/// constant casts.
///
/// # Safety
///
/// See [`transmute`]
///
/// [`transmute`]: core::mem::transmute
#[inline]
pub(crate) const unsafe fn transmute_lax<A, B>(src: A) -> B {
    debug_assert!(size_of::<A>() == size_of::<B>());
    debug_assert!(align_of::<A>() == align_of::<B>());
    let a = ManuallyDrop::new(src);
    let b = unsafe { TransmuteLax::<A, B> { a }.b };
    ManuallyDrop::into_inner(b)
}
