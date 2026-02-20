use super::*;


pub const MAX_ALIGN : usize = std::mem::align_of::<word>();


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AllocLayout
{
    pub size: usize,
    pub align: usize,
}
impl AllocLayout
{
    pub const fn size(&self) -> usize { self.size }
    pub const fn align(&self) -> usize { self.align }
}

pub trait FromAllocLayout : From<AllocLayout>
{
    fn from_alloc_layout(layout: AllocLayout) -> Self { Self::from(layout) }

    fn of_type<T>() -> Self { Self::from_size_and_align(std::mem::size_of::<T>(), std::mem::align_of::<T>()) }
    fn from_size_and_align(size: usize, align: usize) -> Self { Self::from(AllocLayout::from_size_and_align(size, align)) }
    fn from_size_and_align_and_array(size: usize, align: usize, len: usize) -> Self { Self::from(AllocLayout::from_size_and_align(size, align).array(len)) }
    /// Use the maximum align
    fn from_size(size: usize) -> Self { Self::from_size_and_align(size, MAX_ALIGN) }
}

impl<T> FromAllocLayout for T where T: From<AllocLayout> {}

impl AllocLayout
{
    pub fn from_size_and_align(size: usize, align: usize) -> Self { Self { size, align } }
    pub fn try_array(self, len: usize) -> Option<Self>
    {
        let size = self.size.checked_mul(len)?;
        Some(Self { size, align: self.align })
    }
    /// Panics on multiplicatin overflow
    #[track_caller]
    pub fn array(self, len: usize) -> Self {
        self.try_array(len).expect("overflow")
    }
}
