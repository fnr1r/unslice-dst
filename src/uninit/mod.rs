//! Uninitialized pointer type

pub(crate) use self::uowned::UninitOwner;
pub use self::{field_info::FieldInfo, field_layout::DstFieldInfo, umut::UninitMut, uref::UninitRef};

mod field_info;
mod field_layout;
mod umut;
mod uowned;
mod uref;
