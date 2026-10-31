#[cfg(not(feature = "cast_macro_tt_muncher"))]
mod simple;
#[cfg(feature = "cast_macro_tt_muncher")]
mod tt_muncher;
