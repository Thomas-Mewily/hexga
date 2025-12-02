use super::*;

#[derive(Debug)]
pub struct Surface<'a>
{
    pub wgpu: wgpu::Surface<'a>,
}
impl<'a> From<wgpu::Surface<'a>> for Surface<'a>
{
    fn from(surface: wgpu::Surface<'a>) -> Self { Self { wgpu: surface } }
}
impl<'a> From<Surface<'a>> for wgpu::Surface<'a>
{
    fn from(surface: Surface<'a>) -> Self { surface.wgpu }
}


#[derive(Debug)]
pub struct ConfiguredSurface<'a>
{
    pub wgpu: WgpuConfiguredSurface<'a>,
}
#[derive(Debug)]
pub struct WgpuConfiguredSurface<'a>
{
    pub surface: Surface<'a>,
    pub configuration: wgpu::SurfaceConfiguration,
}

impl<'a> ConfiguredSurface<'a>
{
    pub fn resize(&mut self, size: Point2)
    {
        let size = size.max(one());
        self.wgpu.configuration.width = size.x as _;
        self.wgpu.configuration.height = size.y as _;
        self.wgpu.surface.wgpu.configure(Gpu.device(), &self.wgpu.configuration);
    }

    fn size(&self) -> Point2
    {
        point2(self.wgpu.configuration.width as _, self.wgpu.configuration.height as _)
    }
}