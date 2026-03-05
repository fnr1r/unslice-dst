use super::SliceWithHeader;

pub trait SliceWithHeaderDerefEnable<H, I>: Sized {
    type DerefTarget: ?Sized;
    fn deref_impl(this: &SliceWithHeader<H, I, Self>) -> &Self::DerefTarget;
}

pub trait SliceWithHeaderDerefMutEnable<H, I>: Sized + SliceWithHeaderDerefEnable<H, I> {
    fn deref_mut_impl(this: &mut SliceWithHeader<H, I, Self>) -> &mut Self::DerefTarget;
}

pub type DefaultConfig = ();

#[derive(Debug, Clone, Copy)]
pub struct DerefToHeaderConfig;

impl<H, I> SliceWithHeaderDerefEnable<H, I> for DerefToHeaderConfig {
    type DerefTarget = H;
    #[inline]
    fn deref_impl(this: &super::SliceWithHeader<H, I, Self>) -> &Self::DerefTarget {
        &this.header
    }
}

impl<H, I> SliceWithHeaderDerefMutEnable<H, I> for DerefToHeaderConfig {
    #[inline]
    fn deref_mut_impl(this: &mut SliceWithHeader<H, I, Self>) -> &mut Self::DerefTarget {
        &mut this.header
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DerefToSliceConfig;

impl<H, I> SliceWithHeaderDerefEnable<H, I> for DerefToSliceConfig {
    type DerefTarget = [I];
    #[inline]
    fn deref_impl(this: &super::SliceWithHeader<H, I, Self>) -> &Self::DerefTarget {
        &this.slice
    }
}

impl<H, I> SliceWithHeaderDerefMutEnable<H, I> for DerefToSliceConfig {
    #[inline]
    fn deref_mut_impl(this: &mut SliceWithHeader<H, I, Self>) -> &mut Self::DerefTarget {
        &mut this.slice
    }
}
