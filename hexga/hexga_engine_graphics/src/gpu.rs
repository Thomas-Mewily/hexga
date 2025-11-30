use super::*;

#[derive(Clone)]
pub struct Context
{
    pub adapter : wgpu::Adapter,
    pub instance: wgpu::Instance,
    pub executor: Executor,
}
impl Debug for Context
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context").field("adapter", &self.adapter).field("instance", &self.instance).field("executor", &self.executor).finish()
    }
}




/*
impl Context
{
    pub async fn with_instance_and_surface(instance: Instance, surface: Option<&wgpu::Surface<'_>>) -> GpuResult<Self>
    {
        let adapter = instance.instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: surface,
                force_fallback_adapter: false,
            }).await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor
                {
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    ..Default::default()
                })
            .await?;

        Ok(
            Self
            {
                adapter,
                instance,
                device,
                queue,
            }
        )
    }
    pub async fn with_surface(surface: Option<&wgpu::Surface<'_>>) -> GpuResult<Self>
    {
        Self::with_instance_and_surface(Instance::new(instance_desc), surface).await
    }
    pub async fn new() -> GpuResult<Self> {
        Self::with_surface(None).await
    }
}
*/
/*
pub struct Instance
{
    pub instance: wgpu::Instance,
}
impl Instance
{
    pub fn new() -> Instance
    {
        Self::default()
    }
}
impl From<wgpu::Instance> for Instance
{
    fn from(instance: wgpu::Instance) -> Self {
        Self { instance }
    }
}
impl From<Instance> for wgpu::Instance
{
    fn from(value: Instance) -> Self {
        value.instance
    }
}
impl Default for Instance
{
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let mut backends = wgpu::Backends::GL | wgpu::Backends::METAL | wgpu::Backends::DX12 | wgpu::Backends::BROWSER_WEBGPU;

        // Vulkan is super slow to start (~30s to create a window)
        #[cfg(all(not(target_arch = "wasm32"), not(debug_assertions)))]
        {
            backends |= wgpu::Backends::VULKAN;
        }

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            ..Default::default()
        });

        Self { instance }
    }
}





pub struct Pipeline
{

}

pub struct GpuTexture
{
    inner: wgpu::Texture,
}

pub struct Texture
{
    pub(crate) view: GpuTexture,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) bind_group : wgpu::BindGroup
}
*/