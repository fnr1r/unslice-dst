#![allow(dead_code)]

use core::mem::MaybeUninit;

use crate::{DstLayout, uninit::UninitMut};

pub(crate) trait InitDst<T: ?Sized>: FnOnce(UninitMut<'_, T>) {}

impl<T: ?Sized, F: FnOnce(UninitMut<'_, T>)> InitDst<T> for F {}

pub(crate) trait InitSized<T>: FnOnce(UninitMut<'_, T>) -> &mut T {}
pub(crate) trait InitSizedCan<T>: FnOnce(&mut MaybeUninit<T>) -> &mut T {}
pub(crate) trait InitSlice<T>: FnOnce(UninitMut<'_, [T]>) -> &mut [T] {}
pub(crate) trait InitSliceCan<T>: FnOnce(&mut [MaybeUninit<T>]) -> &mut [T] {}

impl<T, F: FnOnce(UninitMut<'_, T>) -> &mut T> InitSized<T> for F {}
impl<T, F: FnOnce(&mut MaybeUninit<T>) -> &mut T> InitSizedCan<T> for F {}
impl<T, F: FnOnce(UninitMut<'_, [T]>) -> &mut [T]> InitSlice<T> for F {}
impl<T, F: FnOnce(&mut [MaybeUninit<T>]) -> &mut [T]> InitSliceCan<T> for F {}

pub(crate) trait InitHead<T: ?Sized + DstLayout>: InitSized<T::Head> {}
pub(crate) trait InitHeadCan<T: ?Sized + DstLayout>: InitSizedCan<T::Head> {}
pub(crate) trait InitTail<T: ?Sized + DstLayout>: InitSlice<T::Tail> {}
pub(crate) trait InitTailCan<T: ?Sized + DstLayout>: InitSliceCan<T::Tail> {}

impl<T: ?Sized + DstLayout, F: InitSized<T::Head>> InitHead<T> for F {}
impl<T: ?Sized + DstLayout, F: InitSizedCan<T::Head>> InitHeadCan<T> for F {}
impl<T: ?Sized + DstLayout, F: InitSlice<T::Tail>> InitTail<T> for F {}
impl<T: ?Sized + DstLayout, F: InitSliceCan<T::Tail>> InitTailCan<T> for F {}
