use super::*;

static GPU_CTX: OnceLock<GpuContext> = OnceLock::new();

pub struct Gpu;

impl Deref for Gpu
{
    type Target=GpuContext;
    fn deref(&self) -> &Self::Target {
        GPU_CTX.get().expect("gpu not init")
    }
}
impl Gpu
{
    pub fn is_init() -> bool { GPU_CTX.get().is_some() }
    pub fn is_not_init() -> bool { !Self::is_init() }
}

impl<F> AsyncRunner<F,GpuParam> for Gpu
    where F: AsyncFnOnce(GpuInitOutput)
{
    type Output=GpuResult;
    async fn run_with_param(f: F, param: GpuParam) -> Self::Output {
        let output = Self::new(param).await?;
        f(output);
        Ok(())
    }
}

impl Gpu
{
    pub async fn new(param: GpuParam) -> GpuResult<GpuInitOutput>
    {
        if Gpu::is_init() { return Err(GpuError::GpuAlreadyInit) }
        Self::from_init(GpuInit::new(param).await?).await
    }
    pub async fn from_init(gpu: GpuInit) -> GpuResult<GpuInitOutput>
    {
        let GpuInit{ gpu, output } = gpu;
        match GPU_CTX.try_insert(gpu)
        {
            Ok(_) => Ok(output),
            Err(_) => Err(GpuError::GpuAlreadyInit),
        }
    }
}


#[derive(Default)]
pub struct GpuParam
{
    pub instance: InstanceDescriptor,
    pub wgpu_instance: WgpuInstanceDescriptor,

    pub power_preference: PowerPreference,
    pub compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
}
impl Debug for GpuParam
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuParam").field("instance", &self.instance).field("power_preference", &self.power_preference).field("compatible_surface", &self.compatible_surface.is_some()).finish()
    }
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerPreference {
    #[default]
    /// Power usage is not considered when choosing an adapter.
    None = 0,
    /// Adapter that uses the least possible power. This is often an integrated GPU.
    LowPower = 1,
    /// Adapter that has the highest performance. This is often a discrete GPU.
    HighPerformance = 2,
}
impl Into<wgpu::PowerPreference> for PowerPreference
{
    fn into(self) -> wgpu::PowerPreference {
        match self
        {
            PowerPreference::None => wgpu::PowerPreference::None,
            PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}


#[derive(Debug)]
pub struct GpuInit
{
    gpu: GpuContext,
    output: GpuInitOutput,
}
impl GpuInit
{
    pub async fn from_instance_and_surface(instance: GpuInstance, surface: Option<GpuSurface<'static>>, param: GpuParam) -> GpuResult<Self>
    {
        let adapter = instance.wgpu
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: param.power_preference.into(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: surface.as_ref().map(|s| &s.wgpu),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        Ok(GpuInit
            {
                gpu:  GpuContext { wgpu: WgpuContext { instance, adapter, device, queue } },
                output: GpuInitOutput { surface: surface.map(|wgpu| wgpu.into()) }
            }
        )
    }
    pub async fn new(mut param: GpuParam) -> GpuResult<Self>
    {
        let instance = GpuInstance::new(&param.instance);

        let surface = match param.compatible_surface.take()
        {
            Some(s) => Some(instance.wgpu.create_surface(s)?),
            None => None,
        };
        Self::from_instance_and_surface(instance, surface.map(|v| v.into()), param).await
    }
}
#[derive(Debug)]
pub struct GpuInitOutput
{
    pub surface: Option<GpuSurface<'static>>,
}

#[bit_index]
#[repr(u8)]
pub enum Backend
{
    // No operation
    Noop,
    /// Supported on Windows, Linux/Android, and macOS/iOS via Vulkan Portability (with the Vulkan feature enabled)
    Vulkan,

    /// Supported on Linux/Android, the web through webassembly via WebGL, and Windows and
    /// macOS/iOS via ANGLE
    Gl,
    /// Supported on macOS and iOS.
    Metal,
    /// Supported on Windows 10 and later
    Dx12,

    /// Supported when targeting the web through WebAssembly with the `webgpu` feature enabled.
    BrowserWebgpu,

    /// All the apis that wgpu offers first tier of support for.
    Primary = Self::Vulkan | Self::Metal | Self::Dx12 | Self::BrowserWebgpu,
    /// All the apis that wgpu offers second tier of support for. These may
    /// be unsupported/still experimental.
    Secondary = Self::Gl,
    Debug = Self::Gl,
}

impl From<Backend> for wgpu::Backends
{
    fn from(value: Backend) -> Self {
        use wgpu::Backends as B;
        match value
        {
            Backend::Noop => B::NOOP,
            Backend::Vulkan => B::VULKAN,
            Backend::Gl => B::GL,
            Backend::Metal => B::METAL,
            Backend::Dx12 => B::DX12,
            Backend::BrowserWebgpu => B::BROWSER_WEBGPU,
        }
    }
}
impl From<BackendFlags> for wgpu::Backends
{
    fn from(backends: BackendFlags) -> Self {
        let mut flags = wgpu::Backends::empty();
        for backend in backends
        {
            flags |= backend.into();
        }
        flags
    }
}

impl Default for BackendFlags
{
    fn default() -> Self
    {
        if cfg!(debug_assertions)
        {
            BackendFlags::Debug
        }else
        {
            BackendFlags::Primary | BackendFlags::Secondary
        }
    }
}



#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpuContext
{
    pub wgpu: WgpuContext,
}

#[derive(Debug, Clone)]
pub struct WgpuContext
{
    pub instance: GpuInstance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
impl WgpuContext
{
    pub fn wgpu_instance(&self) -> &wgpu::Instance { &self.instance.wgpu }
    pub fn wgpu_adapter(&self) -> &wgpu::Adapter { &self.adapter }
    pub fn wgpu_device(&self) -> &wgpu::Device { &self.device }
    pub fn wgpu_queue(&self) -> &wgpu::Queue { &self.queue }
}

pub fn instance() -> impl Deref<Target=GpuInstance> { &Gpu.wgpu.instance }

pub fn wgpu_instance() -> impl Deref<Target=wgpu::Instance> { &Gpu.wgpu.instance.wgpu }
pub fn wgpu_adapter() -> impl Deref<Target=wgpu::Adapter> { &Gpu.wgpu.adapter }
pub fn wgpu_device() -> impl Deref<Target=wgpu::Device> { &Gpu.wgpu.device }
pub fn wgpu_queue() -> impl Deref<Target=wgpu::Queue> { &Gpu.wgpu.queue }

