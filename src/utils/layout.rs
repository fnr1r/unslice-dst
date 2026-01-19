use core::alloc::{Layout, LayoutError};

pub(crate) type LayoutPack<const N: usize> = (Layout, [usize; N]);
type Result<T, E = LayoutError> = core::result::Result<T, E>;

pub(crate) fn repr_c<const N: usize>(fields: [Layout; N]) -> Result<LayoutPack<N>> {
    let mut offsets = [0; N];
    let mut iter = fields.into_iter();
    let Some(mut layout) = iter.next() else {
        panic!("No layouts passed into repr_c!");
    };
    for (i, mlayout) in (1..N).zip(iter) {
        let (new_layout, this_offset) = layout.extend(mlayout)?;
        layout = new_layout;
        offsets[i] = this_offset;
    }
    let res = (layout.pad_to_align(), offsets);
    Ok(res)
}
