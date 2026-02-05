//! Uninitialized pointer type

pub use self::{umut::UninitMut, uref::UninitRef};

mod umut;
mod uref;
