use super::*;

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GpuInstanceDescriptor
{
    pub backends: BackendFlags,
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
    fn from(value: Backend) -> Self
    {
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
    fn from(backends: BackendFlags) -> Self
    {
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
        }
        else
        {
            BackendFlags::Primary | BackendFlags::Secondary
        }
    }
}


#[derive(Default, Clone, PartialEq)]
pub struct GpuParam
{
    pub instance: GpuInstanceDescriptor,
    pub wgpu_instance: WgpuInstanceDescriptor,
    pub power_preference: PowerPreference,
    //pub compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PowerPreference
{
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
    fn into(self) -> wgpu::PowerPreference
    {
        match self
        {
            PowerPreference::None => wgpu::PowerPreference::None,
            PowerPreference::LowPower => wgpu::PowerPreference::LowPower,
            PowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
        }
    }
}
