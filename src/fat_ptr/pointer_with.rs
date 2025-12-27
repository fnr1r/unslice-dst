#[cfg(feature = "plain")]
use plain::Plain;

/// Structure for fat pointers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct PointerWith<T> {
    /// The address of the pointer
    pub address: *const (),
    /// The metadata of the pointer
    ///
    /// For slices it's [`usize`].
    pub metadata: T,
}

#[cfg(feature = "plain")]
unsafe impl<T: Plain> Plain for PointerWith<T> {}
