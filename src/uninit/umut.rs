use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// [NonNull] but mut and valid for a set lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitMut<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: ?Sized> Deref for UninitMut<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: ?Sized> DerefMut for UninitMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
