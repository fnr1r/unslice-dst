#![doc = include_str!("../README.md")]
#![deny(meta_variable_misuse)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::missing_const_for_fn)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::cast::DstCast;

pub mod cast;

/// Just a type alias for a slice of units.
///
/// It's assumed to always be of size 0.
pub type AnyDst = [()];
