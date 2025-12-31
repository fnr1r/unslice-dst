/// Automatically implements [`DstCast`] for specified type
///
/// This macro will cause a compilation failure if the fat pointer metadata is
/// not [`usize`].
///
/// # Safety
///
/// This macro automatically implements an `unsafe` trait. See [`DstCast`] for
/// more.
///
/// # Notes
///
/// This is a TT muncher macro. See crate-level docs for info.
///
/// [`DstCast`]: [super::DstCast]
#[macro_export]
macro_rules! dst_cast_impl {
    (
        @impl
        $(generics [$($generics:tt)*])?
        for $type:ty
        $(where [$($where_clauses:tt)+])?
    ) => {
        unsafe impl
        $(<$($generics)*>)?
        $crate::DstCast for $type
        $(where $($where_clauses)+)?
        {
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
    (
        @passthrough
        $($all:tt)*
    ) => {
        $($all)*
    };
    (
        @do_parse_generics
        generics [$($generics:tt)+] > for
        $($rest:tt)+
    ) => {
        $crate::dst_cast_impl! {
            @do_parse_for
            generics [$($generics)+]
            for
            $($rest)+
        }
    };
    (
        @do_parse_generics
        generics [$($generics:tt)*] $token:tt
        $($rest:tt)+
    ) => {
        $crate::dst_cast_impl! {
            @do_parse_generics
            generics [$($generics)* $token]
            $($rest)+
        }
    };
    (
        @do_parse_for
        $(generics [$($generics:tt)*])?
        for $type:ty where
        $($rest:tt)*
    ) => {
        $crate::dst_cast_impl! {
            @do_parse_where
            $(generics [$($generics)*])?
            for $type
            where $($rest)*
        }
    };
    (
        @do_parse_for
        $(generics [$($generics:tt)*])?
        for $type:ty ;
        $($rest:tt)*
    ) => {
        $crate::dst_cast_impl! {
            @impl
            $(generics [$($generics)*])?
            for $type
        }
        $crate::dst_cast_impl! {
            $($rest)*
        }
    };
    (
        @do_parse_for
        $(generics [$($generics:tt)*])?
        for $type:ty
    ) => {
        $crate::dst_cast_impl! {
            @impl
            $(generics [$($generics)*])?
            for $type
        }
    };
    (
        @do_parse_where
        $(generics [$($generics:tt)*])?
        for $type:ty
        where [$($where_clauses:tt)*] ;
        $($rest:tt)*
    ) => {
        $crate::dst_cast_impl! {
            @impl
            $(generics [$($generics)*])?
            for $type
            where [$($where_clauses)*]
        }
        $crate::dst_cast_impl! {
            $($rest)*
        }
    };
    (
        @do_parse_where
        $(generics [$($generics:tt)*])?
        for $type:ty
        where [$($where_clauses:tt)*] $token:tt
        $($rest:tt)*
    ) => {
        $crate::dst_cast_impl! {
            @do_parse_where
            $(generics [$($generics)*])?
            for $type
            where [$($where_clauses)* $token]
            $($rest)*
        }
    };
    (<$($rest:tt)*) => {
        $crate::dst_cast_impl! {
            @do_parse_generics
            generics []
            $($rest)*
        }
    };
    (for $($rest:tt)*) => {
        $crate::dst_cast_impl! {
            @do_parse_for
            for $($rest)*
        }
    };
    () => {};
}
