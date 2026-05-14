use super::*;

#[derive(Debug)]
pub struct GpuSurface<'a>
{
    pub wgpu: wgpu::Surface<'a>,
}
impl<'a> From<wgpu::Surface<'a>> for GpuSurface<'a>
{
    fn from(surface: wgpu::Surface<'a>) -> Self { Self { wgpu: surface } }
}
impl<'a> From<GpuSurface<'a>> for wgpu::Surface<'a>
{
    fn from(surface: GpuSurface<'a>) -> Self { surface.wgpu }
}

#[derive(Debug)]
pub struct GpuSurfaceConfigured<'a>
{
    pub(crate) surface: GpuSurface<'a>,
    pub(crate) configuration: wgpu::SurfaceConfiguration,
}

impl<'a> GpuSurfaceConfigured<'a>
{
    pub fn from_surface(surface: GpuSurface<'a>, size: Point2) -> Self
    {
        let size = size.max(one());
        let configuration = surface
            .wgpu
            .get_default_config(&Gpu.adapter(), size.x as _, size.y as _)
            .unwrap();
        surface.wgpu.configure(&Gpu.device(), &configuration);
        Self {
            surface,
            configuration,
        }
    }

    fn size(&self) -> Point2
    {
        GetSize::size(self)
    }
}
impl<'a> SetSize<int, 2> for GpuSurfaceConfigured<'a>
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self
    {
        let size = size.max(one());
        self.configuration.width = size.x as _;
        self.configuration.height = size.y as _;
        self.surface.wgpu.configure(&Gpu.device(), &self.configuration);
        self
    }
}
impl<'a> GetSize<int, 2> for GpuSurfaceConfigured<'a>
{
    fn size(&self) -> Vector<int, 2>
    {
        point2(
            self.configuration.width as _,
            self.configuration.height as _,
        )
    }
}
