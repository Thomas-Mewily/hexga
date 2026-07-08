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
    pub backends: GpuBackendFlags,
    pub wgpu: WgpuInstanceDescriptor,
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

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct WgpuInstanceDescriptor
{
    /// Flags to tune the behavior of the instance.
    pub flags: wgpu::InstanceFlags,
    /// Options the control the behavior of various backends.
    pub backend_options: wgpu::BackendOptions,
}

#[bit_index]
#[repr(u8)]
pub enum GpuBackend
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
    /// Apis that are quick to debug / setup for a fast iteration cycle
    Debug = Self::Gl,
}

impl From<GpuBackend> for wgpu::Backends
{
    fn from(value: GpuBackend) -> Self
    {
        use wgpu::Backends as B;
        match value
        {
            GpuBackend::Noop => B::NOOP,
            GpuBackend::Vulkan => B::VULKAN,
            GpuBackend::Gl => B::GL,
            GpuBackend::Metal => B::METAL,
            GpuBackend::Dx12 => B::DX12,
            GpuBackend::BrowserWebgpu => B::BROWSER_WEBGPU,
        }
    }
}
impl From<GpuBackendFlags> for wgpu::Backends
{
    fn from(backends: GpuBackendFlags) -> Self
    {
        let mut flags = wgpu::Backends::empty();
        for backend in backends
        {
            flags |= backend.into();
        }
        flags
    }
}

impl Default for GpuBackendFlags
{
    fn default() -> Self
    {
        if cfg!(debug_assertions)
        {
            GpuBackendFlags::Debug
        }
        else
        {
            GpuBackendFlags::Primary | GpuBackendFlags::Secondary
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct GpuParam
{
    pub instance: GpuInstanceDescriptor,
    pub wgpu_instance: WgpuInstanceDescriptor,
    pub power_preference: GpuPowerPreference,
    //pub compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
}
impl Debug for GpuParam
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("GpuParam")
            .field("instance", &self.instance)
            .field("power_preference", &self.power_preference)
            .finish()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GpuPowerPreference
{
    #[default]
    /// Power usage is not considered when choosing an adapter.
    None = 0,
    /// Adapter that uses the least possible power. This is often an integrated GPU.
    LowPower = 1,
    /// Adapter that has the highest performance. This is often a discrete GPU.
    HighPerformance = 2,
}
impl Into<wgpu::PowerPreference> for GpuPowerPreference
{
    fn into(self) -> wgpu::PowerPreference
    {
        match self
        {
            GpuPowerPreference::None => wgpu::PowerPreference::None,
            GpuPowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            GpuPowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}
