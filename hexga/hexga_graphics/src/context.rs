use super::*;

static GPU_CTX: OnceLock<WgpuGpu> = OnceLock::new();

pub struct Gpu;

impl Gpu
{
    pub fn try_init(ctx: WgpuGpu) -> GpuResult<&'static WgpuGpu>
    {
        GPU_CTX.try_insert(ctx).map_err(|_|GpuError::GpuAlreadyInit)
    }
    /// Same as [try_init][`Self::try_init`] but panic on error.
    pub fn init(ctx: WgpuGpu) -> &'static WgpuGpu
    {
        Self::try_init(ctx).expect("The gpu context was already initialized")
    }
    pub fn context_or_init<F>(f: F) -> &'static WgpuGpu
        where F: FnOnce() -> WgpuGpu
    {
        GPU_CTX.get_or_init(f)
    }
    pub fn context() -> &'static WgpuGpu
    {
        Self::context_or_init(|| panic!("gpu context was not init"))
    }

    pub async fn default_init(param: GpuParam<'_,'_>) -> GpuResult
    {
        let gpu = Self::default(param).await?;
        Gpu::try_init(gpu)?;
        Ok(())
    }

    pub async fn default(param: GpuParam<'_,'_>) -> GpuResult<WgpuGpu>
    {
        let mut flags = wgpu::InstanceFlags::empty();
        if cfg!(debug_assertions)
        {
            flags |= wgpu::InstanceFlags::VALIDATION;
        }

        let mut backends = wgpu::Backends::empty();
        backends |= wgpu::Backends::GL;
        backends |= wgpu::Backends::METAL;
        //backends |= wgpu::Backends::DX12; // Seem to Allocate at least 250 MB of RAM
        backends |= wgpu::Backends::BROWSER_WEBGPU;
        // Todo: make a flag for it
        // backends |= wgpu::Backends::VULKAN; // Why it is slow as hell to start

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            flags,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: param.default_surface.as_ref().map(|s| s.surface),
            })
            .await?;

        let required_limits = wgpu::Limits::default();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits,
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await?;

        if let Some(surface) = param.default_surface
        {
            let size = max(surface.size, one());
            let surface_config = surface.surface.get_default_config(Gpu.adapter(), size.x as _, size.y as _).ok_or(())?;
            surface.surface.configure(Gpu.device(), &surface_config);
        }

        let gpu = WgpuGpu
        {
            instance,
            adapter,
            device,
            queue,
        };

        Ok(gpu)
    }
}


#[derive(Default)]
pub struct GpuParam<'a,'b>
{
    pub default_surface: Option<GpuParamDefaultSurface<'a,'b>>
}
pub struct GpuParamDefaultSurface<'a,'b>
{
    pub surface: &'a wgpu::Surface<'b>,
    pub size: Point2,
}

impl Gpu
{
    pub fn queue(self) -> &'static wgpu::Queue
    {
        &Self::context().queue
    }
    pub fn device(self) -> &'static wgpu::Device
    {
        &Self::context().device
    }
    pub fn adapter(self) -> &'static wgpu::Adapter
    {
        &Self::context().adapter
    }
    pub fn instance(self) -> &'static wgpu::Instance
    {
        &Self::context().instance
    }
}


#[derive(Debug, Clone)]
pub struct WgpuGpu
{
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}