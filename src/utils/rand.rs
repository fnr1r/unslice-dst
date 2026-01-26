//! LCG random number generator for tests
//!
//! Changes from musl:
//!
//! - Idiomatic-ish Rust API
//! - The [`next`] explicitly uses wrapping maths.
//! - Uses a [`Mutex`] for thread-safety (since tests may be multi-threaded)
//! - The seed is set to `1337_u64` by default
//!
//! [`next`]: musl_lcg_next

use std::sync::{Mutex, MutexGuard};

const MUSL_MULTIPLIER: u64 = 6364136223846793005;

const fn musl_lcg_next(value: u64) -> u64 {
    value.wrapping_mul(MUSL_MULTIPLIER).wrapping_add(1)
}

static MUSL_SEED: Mutex<u64> = Mutex::new(1337);

/// Lock and ignore poison error
fn mutex_ilock<T>(lock: &Mutex<T>) -> MutexGuard<'_, T> {
    match lock.lock() {
        Ok(res) => res,
        Err(error) => error.into_inner(),
    }
}

#[derive(Debug)]
pub(crate) struct MuslRand(MutexGuard<'static, u64>);

impl MuslRand {
    pub(crate) fn lock() -> Self {
        Self(mutex_ilock(&MUSL_SEED))
    }
    pub(crate) fn next_inf(&mut self) -> i32 {
        *self.0 = musl_lcg_next(*self.0);
        (*self.0 >> 33) as _
    }
}

pub(crate) fn musl_rand_with_iter<T>(f: impl FnOnce(&mut MuslRand) -> T) -> T {
    f(&mut MuslRand::lock())
}
