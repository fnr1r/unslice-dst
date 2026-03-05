#[cfg(doc)]
use core::ops::Deref;

#[cfg(doc)]
use super::SliceWithHeader;

/// Enable [`Deref`]-ing the header for [`SliceWithHeader`]
///
/// The deref target is the header.
pub trait SliceWithHeaderDerefEnable {}

/// A [`deref`](SliceWithHeaderDerefEnable) configuration
#[derive(Debug, Clone, Copy)]
pub struct DerefConfig;

impl SliceWithHeaderDerefEnable for DerefConfig {}
