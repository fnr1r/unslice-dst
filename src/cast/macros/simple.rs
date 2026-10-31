/// Automatically implements [`DstCast`] for specified type
///
/// This macro will cause a compilation failure if the fat pointer metadata is
/// not [`usize`].
///
/// # Broken
///
/// This macro doesn't support const generics required for the tests of the
/// `layout` module and is planned to be removed.
///
/// # Safety
///
/// This macro automatically implements an `unsafe` trait. See [`DstCast`] for
/// more.
///
/// # Notes
///
/// This is a regular macro. See crate-level docs for info.
///
/// [`DstCast`]: [super::DstCast]
#[macro_export]
macro_rules! dst_cast_impl {
    (
        $(< $($generic_name:ident),* >)?
        for $type:ty $(where $($rest:tt)+)?
    ) => {
        unsafe impl $(<$($generic_name),*>)? $crate::DstCast for $type
        $(where $($rest)+)?
        {
            const POINTER_SIZE_MATCHES_SLICE: $crate::cast::sealed::Sealed<Self> =
                $crate::cast::sealed::assert_fat_ptr_matches_size_dst();
            #[inline(always)]
            fn cast_into_slice<U>(this: *const Self) -> *const [U] {
                this as _
            }
            #[inline(always)]
            fn cast_from_slice<U>(this: *const [U]) -> *const Self {
                this as _
            }
            #[inline(always)]
            fn cast_into_slice_mut<U>(this: *mut Self) -> *mut [U] {
                this as _
            }
            #[inline(always)]
            fn cast_from_slice_mut<U>(this: *mut [U]) -> *mut Self {
                this as _
            }
        }
    };
}
