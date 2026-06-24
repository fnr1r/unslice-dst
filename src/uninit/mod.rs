//! Uninitialized pointer type
//!
//! Exists only because [`MaybeUninit`] requires `Sized`
//!
//! [`MaybeUninit`]: core::mem::MaybeUninit

#![allow(clippy::non_canonical_partial_ord_impl)]

pub use self::{umut::UninitMut, uref::UninitRef};

mod umut;
mod uref;
