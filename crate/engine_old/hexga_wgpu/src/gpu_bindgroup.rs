use super::*;
use hexga::image::ImageBaseOf;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpuBindGroup
{
    pub wgpu: wgpu::BindGroup,
}
impl From<wgpu::BindGroup> for GpuBindGroup
{
    fn from(wgpu: wgpu::BindGroup) -> Self { Self { wgpu } }
}
impl From<GpuBindGroup> for wgpu::BindGroup
{
    fn from(value: GpuBindGroup) -> Self { value.wgpu }
}
