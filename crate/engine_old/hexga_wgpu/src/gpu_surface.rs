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
pub struct GpuConfiguredSurface<'a>
{
    pub wgpu: WgpuConfiguredSurface<'a>,
}
#[derive(Debug)]
pub struct WgpuConfiguredSurface<'a>
{
    pub surface: GpuSurface<'a>,
    pub configuration: wgpu::SurfaceConfiguration,
}

impl<'a> GpuConfiguredSurface<'a>
{
    pub fn from_surface(surface: GpuSurface<'a>, size: Point2) -> Self
    {
        let size = size.max(one());
        let configuration = surface
            .wgpu
            .get_default_config(&Gpu.wgpu.adapter, size.x as _, size.y as _)
            .unwrap();
        surface.wgpu.configure(&Gpu.wgpu.device, &configuration);
        Self {
            wgpu: WgpuConfiguredSurface {
                surface,
                configuration,
            },
        }
    }

    fn size(&self) -> Point2
    {
        point2(
            self.wgpu.configuration.width as _,
            self.wgpu.configuration.height as _,
        )
    }
}
impl<'a> SetSize<int, 2> for GpuConfiguredSurface<'a>
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self
    {
        let size = size.max(one());
        self.wgpu.configuration.width = size.x as _;
        self.wgpu.configuration.height = size.y as _;
        self.wgpu
            .surface
            .wgpu
            .configure(&Gpu.wgpu.device, &self.wgpu.configuration);
        self
    }
}
impl<'a> GetSize<int, 2> for GpuConfiguredSurface<'a>
{
    fn size(&self) -> Vector<int, 2>
    {
        point2(
            self.wgpu.configuration.width as _,
            self.wgpu.configuration.height as _,
        )
    }
}
