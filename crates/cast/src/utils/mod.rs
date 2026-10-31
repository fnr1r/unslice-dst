//! HEY! This is only used internally to avoid code duplication.

pub use self::transmute_lax::transmute_lax;

pub(crate) mod slice;
mod transmute_lax;
