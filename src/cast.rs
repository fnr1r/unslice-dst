//! Support for casting slice-like DSTs

/// This trait signifies that Self if a DST that is cast-able to a slice.
///
/// Rust only allows casting a fat pointer if both types have the same metadata.
///
/// This is why this trait exists.
///
/// Then Rust can assume that the fat pointer metadata type ([`usize`] for all
/// slices) is the same, allowing the as cast.
///
/// Avoid implementing this trait manually. Please, use the [`dst_cast_impl`]
/// macro.
///
/// For a proper source of info, see the official documentation on
/// [pointer-to-pointer casts].
///
/// # Notes
///
/// [`AnyDst`] is used as a generic/untyped representation of a slice DST.
///
/// # Safety
///
/// It's on the user of this trait to ensure proper use.
///
/// [`AnyDst`]: crate::AnyDst
/// [`dst_cast_impl`]: crate::dst_cast_impl
/// [pointer-to-pointer casts]: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.pointer>
pub unsafe trait DstCast {
    /// Cast a const pointer to a slice-like DST to a slice
    ///
    /// See [`DstCast`] for more info
    fn cast_into_slice<U>(this: *const Self) -> *const [U];
    /// Cast a const pointer to a slice to a slice-like DST
    ///
    /// See [`DstCast`] for more info
    fn cast_from_slice<U>(this: *const [U]) -> *const Self;
    /// Cast a mut pointer to a slice-like DST to a slice
    ///
    /// See [`DstCast`] for more info
    fn cast_into_slice_mut<U>(this: *mut Self) -> *mut [U];
    /// Cast a mut pointer to a slice to a slice-like DST
    ///
    /// See [`DstCast`] for more info
    fn cast_from_slice_mut<U>(this: *mut [U]) -> *mut Self;

    /// Cast a const pointer to a slice-like DST to a different slice-like DST
    ///
    /// See [`DstCast`] for more info
    #[inline]
    fn cast_dst_const<U: ?Sized + DstCast>(this: *const U) -> *const Self {
        Self::cast_from_slice(U::cast_into_slice::<()>(this))
    }
    /// Cast a mut pointer to a slice-like DST to a different slice-like DST
    ///
    /// See [`DstCast`] for more info
    #[inline]
    fn cast_dst_mut<U: ?Sized + DstCast>(this: *mut U) -> *mut Self {
        Self::cast_from_slice_mut(U::cast_into_slice_mut::<()>(this))
    }
}
