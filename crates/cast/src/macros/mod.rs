#[cfg(not(feature = "macro_tt_muncher"))]
mod simple;
#[cfg(feature = "macro_tt_muncher")]
mod tt_muncher;
