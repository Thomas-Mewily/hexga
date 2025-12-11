use hexga_graphics::GpuParam;
use super::*;

pub type AppParam = AppParamOf;
pub type AppParamInternal = AppParamOf<winit::window::WindowAttributes>;

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct AppParamOf<W=WindowParam>
{
    pub window: W,
    pub gpu: Option<GpuParam>,
}

impl From<AppParam> for AppParamInternal
{
    fn from(value: AppParam) -> Self {
        Self { window: value.window.into(), gpu: value.gpu }
    }
}

impl WindowBuilder for AppParam
{
    fn with_title(self, title: impl Into<String>) -> Self { Self { window: self.window.with_title(title), ..self } }
    fn with_size(self, size: impl Into<Option<Point2>>) -> Self { Self { window: self.window.with_size(size), ..self } }
}

impl AppParam
{
    pub fn new() -> Self { ___() }
    pub fn with_gpu(self, gpu: impl Into<Option<GpuParam>>) -> Self { Self { gpu: gpu.into(), ..self } }
}