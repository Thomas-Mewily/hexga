use super::*;

#[derive(Debug)]
pub struct Instance
{
    pub wgpu: wgpu::Instance,
}
impl<'a> From<wgpu::Instance> for Instance
{
    fn from(surface: wgpu::Instance) -> Self { Self { wgpu: surface } }
}
impl<'a> From<Instance> for wgpu::Instance
{
    fn from(surface: Instance) -> Self { surface.wgpu }
}

impl Default for Instance
{
    fn default() -> Self {
        let mut flags = wgpu::InstanceFlags::empty();
        if cfg!(debug_assertions)
        {
            flags |= wgpu::InstanceFlags::VALIDATION;
        }

        // Todo: make a flag for it ?
        let mut backends = wgpu::Backends::empty();
        backends |= wgpu::Backends::GL;
        backends |= wgpu::Backends::METAL;
        //backends |= wgpu::Backends::DX12; // Seem to Allocate at least 250 MB of RAM
        backends |= wgpu::Backends::BROWSER_WEBGPU;
        if cfg!(debug_assertions)
        {
            // Why it is slow as hell to start
            backends |= wgpu::Backends::VULKAN;
        }

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            flags,
            ..Default::default()
        });
        Self { wgpu: instance }
    }
}