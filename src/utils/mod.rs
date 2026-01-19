pub(crate) use self::transmute_lax::transmute_lax;

pub(crate) mod layout;
pub(crate) mod nalloc;
pub(crate) mod slice;
mod transmute_lax;

#[cfg(all(test, feature = "std"))]
pub(crate) mod rand;
