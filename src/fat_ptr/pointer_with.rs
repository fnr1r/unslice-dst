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

impl<T: Copy> PointerWith<T> {
    #[inline]
    pub(super) const fn from_raw_parts(address: *const (), metadata: T) -> Self {
        Self { address, metadata }
    }
    #[inline]
    pub(super) const fn into_raw_parts(self) -> (*const (), T) {
        (self.address, self.metadata)
    }
}
