use core::alloc::LayoutError;

use super::FieldInfo;
use crate::layout::{
    DstLayout,
    manual::{ManualLayout, layout_try_for_len},
};

/// Field information for a DST with a known layout
#[derive(Debug)]
pub struct DstFieldInfo<T: ?Sized + DstLayout> {
    pub head: FieldInfo<T, T::Head>,
    pub tail: FieldInfo<T, [T::Tail]>,
}

impl<T: ?Sized + DstLayout> Copy for DstFieldInfo<T> {}

impl<T: ?Sized + DstLayout> Clone for DstFieldInfo<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized + DstLayout> DstFieldInfo<T> {
    const fn from_manual_layout(data: ManualLayout) -> Self {
        Self {
            head: unsafe { FieldInfo::new(data.head_offset) },
            tail: unsafe { FieldInfo::new(data.tail_offset) },
        }
    }
    const fn try_for_len(len: usize) -> Result<Self, LayoutError> {
        Ok(match layout_try_for_len::<T>(len) {
            Ok(res) => Self::from_manual_layout(res),
            Err(e) => return Err(e),
        })
    }
    pub const fn for_len(len: usize) -> Self {
        match Self::try_for_len(len) {
            Ok(res) => res,
            Err(_) => panic!(),
        }
    }
}
