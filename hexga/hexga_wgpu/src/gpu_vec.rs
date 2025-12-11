use super::*;

pub type GpuVecUsages = wgpu::BufferUsages;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GpuVecDesc
{
    pub usages: GpuVecUsages,
    pub name: Option<&'static str>
}
impl Default for GpuVecDesc
{
    fn default() -> Self {
        Self::new()
    }
}
impl GpuVecDesc
{
    pub const fn new() -> Self { Self { usages: GpuVecUsages::COPY_DST.union(GpuVecUsages::COPY_SRC), name: None }}

    pub const fn add_usage(mut self, usage : GpuVecUsages) -> Self { self.usages = self.usages.union(usage); self }
    pub const fn with_usages(mut self, usages : GpuVecUsages) -> Self { self.usages = usages; self }
    pub const fn with_label(mut self, label : Option<&'static str>) -> Self { self.name = label; self }

    pub const VERTEX : Self = Self::new().add_usage(GpuVecUsages::VERTEX);
    pub const INDEX : Self = Self::new().add_usage(GpuVecUsages::INDEX);
}


#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GpuVec<T> where T:Copy
{
    pub(crate) buffer   : wgpu::Buffer,
    pub(crate) capacity : usize,
    pub(crate) len      : usize,
    pub(crate) desc     : GpuVecDesc,
    phantom : PhantomData<T>,
}

impl<T> GpuVec<T> where T:Copy
{
    pub fn name(&self) -> Option<&'static str> { self.desc.name }
    pub fn clear(&mut self)
    {
        self.len = 0;
    }
}
