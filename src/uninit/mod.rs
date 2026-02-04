//! Uninitialized pointer type

pub(crate) use self::uowned::UninitOwner;
pub use self::{umut::UninitMut, uref::UninitRef};

mod umut;
mod uowned;
mod uref;
