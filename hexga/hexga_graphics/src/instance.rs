use super::*;

#[derive(Debug, Clone)]
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
impl Instance
{
    pub fn new(desc: &InstanceDescriptor) -> Self
    {
        Self
        {
            wgpu:
            wgpu::Instance::new(&wgpu::InstanceDescriptor
                {
                    backends: desc.backends.into(),
                    flags: desc.wgpu.flags,
                    backend_options: desc.wgpu.backend_options.clone(),
                }
            )
        }
    }
}

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct InstanceDescriptor
{
    pub backends : BackendFlags,
    pub wgpu: WgpuInstanceDescriptor,
}
#[non_exhaustive]
#[derive(Debug)]
pub struct WgpuInstanceDescriptor
{
    /// Flags to tune the behavior of the instance.
    pub flags: wgpu::InstanceFlags,
    /// Options the control the behavior of various backends.
    pub backend_options:  wgpu::BackendOptions,
}
impl Default for WgpuInstanceDescriptor
{
    fn default() -> Self {
        Self { flags: Default::default(), backend_options: Default::default() }
    }
}

impl Default for Instance
{
    fn default() -> Self {
        Self::new(&___())
    }
}