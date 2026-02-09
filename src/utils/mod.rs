pub(crate) use self::transmute_lax::transmute_lax;

pub(crate) mod nalloc;
pub(crate) mod slice;
mod transmute_lax;

#[cfg(test)]
pub(crate) mod rand;
