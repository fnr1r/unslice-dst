//! Uninitialized pointer type

#![allow(clippy::non_canonical_partial_ord_impl)]

pub use self::{umut::UninitMut, uref::UninitRef};

mod umut;
mod uref;
