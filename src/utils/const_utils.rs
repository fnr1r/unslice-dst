//! const utilities (minimal)

use core::{mem::ManuallyDrop, ptr};

/// Create a reference to T from a [`ManuallyDrop`] `T`
///
/// from `const-util` 2.2.0
#[inline]
const fn manually_drop_as_ref<T>(this: &ManuallyDrop<T>) -> &T {
    // SAFETY: repr(transparent)
    unsafe { ptr::from_ref(this).cast::<T>().as_ref().unwrap_unchecked() }
}

/// [`Result::expect`] but const
///
/// from `const-util` 2.2.0
#[inline]
#[track_caller]
pub(crate) const fn expect_ok<T, E>(this: Result<T, E>, msg: &str) -> T {
    let this = ManuallyDrop::new(this);
    let Ok(this) = manually_drop_as_ref(&this) else {
        panic!("{}", msg);
    };
    // SAFETY: ManuallyDrop
    unsafe { ptr::read(this) }
}

/// [`Result::unwrap`] but const
///
/// from `const-util` 2.2.0
#[inline]
#[track_caller]
pub(crate) const fn unwrap_ok<T, E>(this: Result<T, E>) -> T {
    expect_ok(this, "Attempted to call `unwrap_ok` on an `Err` variant")
}

/// Question mark operator. But const.
#[doc(hidden)]
#[macro_export]
macro_rules! ctry {
    ($($arg:tt)*) => {
        match ($($arg)*) {
            Ok(res) => res,
            Err(e) => return Err(e),
        }
    };
}
