use core::{marker::PhantomData, ops::Deref, ptr::NonNull};

/// [NonNull] but valid for a set lifetime.
///
/// Not very useful. Not valid for reads (unless you can assume otherwhise) or
/// writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct UninitRef<'a, T: ?Sized>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T: ?Sized> Deref for UninitRef<'a, T> {
    type Target = NonNull<T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
