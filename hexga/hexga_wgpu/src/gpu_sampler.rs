use super::*;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpuSampler
{
    pub wgpu: wgpu::Sampler,
}
impl From<wgpu::Sampler> for GpuSampler
{
    fn from(wgpu: wgpu::Sampler) -> Self { Self { wgpu } }
}
impl From<GpuSampler> for wgpu::Sampler
{
    fn from(sampler: GpuSampler) -> Self { sampler.wgpu }
}
impl Default for GpuSampler
{
    fn default() -> Self { Self::new_linear() }
}
impl GpuSampler
{
    pub fn new_pixel_art() -> Self { Self::new_nearest() }
    pub fn new_nearest() -> Self
    {
        Gpu.wgpu
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })
            .into()
    }

    pub fn new_linear() -> Self
    {
        Gpu.wgpu
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            })
            .into()
    }
}
