use super::*;

#[derive(Debug, Clone)]
pub struct GpuInstance
{
    pub wgpu: wgpu::Instance,
}
impl<'a> From<wgpu::Instance> for GpuInstance
{
    fn from(surface: wgpu::Instance) -> Self { Self { wgpu: surface } }
}
impl<'a> From<GpuInstance> for wgpu::Instance
{
    fn from(surface: GpuInstance) -> Self { surface.wgpu }
}
impl GpuInstance
{
    pub fn new(desc: &GpuInstanceDescriptor) -> Self
    {
        Self {
            wgpu: wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: desc.backends.into(),
                flags: desc.wgpu.flags,
                backend_options: desc.wgpu.backend_options.clone(),
            }),
        }
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GpuInstanceDescriptor
{
    pub backends: BackendFlags,
    pub wgpu: WgpuInstanceDescriptor,
}
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct WgpuInstanceDescriptor
{
    /// Flags to tune the behavior of the instance.
    pub flags: wgpu::InstanceFlags,
    /// Options the control the behavior of various backends.
    pub backend_options: wgpu::BackendOptions,
}

impl PartialEq for WgpuInstanceDescriptor
{
    fn eq(&self, other: &Self) -> bool
    {
        // Todo : also take into consideration the backend_options?
        self.flags == other.flags
    }
}

impl Default for WgpuInstanceDescriptor
{
    fn default() -> Self
    {
        Self {
            flags: Default::default(),
            backend_options: Default::default(),
        }
    }
}

impl Default for GpuInstance
{
    fn default() -> Self { Self::new(&___()) }
}
