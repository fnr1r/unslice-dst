//! LCG random number generator for tests
//!
//! Changes from musl:
//!
//! - Idiomatic-ish Rust API
//! - The [`next`] function explicitly uses wrapping maths.
//! - The RNG is instanced. No statics or shared state.
//! - The seed is set to `1337_u64` by default
//!
//! TODO: Maybe use a different seed for different tests?
//!
//! [`next`]: musl_lcg_next

const MUSL_MULTIPLIER: u64 = 6364136223846793005;

const fn musl_lcg_next(value: u64) -> u64 {
    value.wrapping_mul(MUSL_MULTIPLIER).wrapping_add(1)
}

const MUSL_SEED: u64 = 1337;

#[derive(Debug)]
pub(crate) struct MuslRand(u64);

impl MuslRand {
    pub(crate) fn next_inf(&mut self) -> i32 {
        self.0 = musl_lcg_next(self.0);
        (self.0 >> 33) as _
    }
}

pub(crate) fn musl_rand_with_iter<T>(f: impl FnOnce(&mut MuslRand) -> T) -> T {
    f(&mut MuslRand(MUSL_SEED))
}
