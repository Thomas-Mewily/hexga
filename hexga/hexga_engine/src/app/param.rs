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

impl HasMut<WindowParam> for AppParam
{
    fn retrive_mut(&mut self) -> &mut WindowParam {
        &mut self.window
    }
}

impl AppParam
{
    pub fn new() -> Self { ___() }
    pub fn with_gpu(self, gpu: impl Into<Option<GpuParam>>) -> Self { Self { gpu: gpu.into(), ..self } }
}