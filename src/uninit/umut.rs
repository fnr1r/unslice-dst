use core::{
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// [NonNull] but mut (invariant) and valid for a set lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitMut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Deref for UninitMut<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute::<&Self, &NonNull<T>>(self) }
    }
}

impl<'a, T: ?Sized> DerefMut for UninitMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute::<&mut Self, &mut NonNull<T>>(self) }
    }
}

impl<'a, T: ?Sized> UninitMut<'a, T> {
    pub(super) const unsafe fn new(ptr: NonNull<T>) -> Self {
        Self(ptr, PhantomData)
    }
}
