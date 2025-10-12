use super::*;


singleton_access!(
    pub Pen,
    GpuPen,
    { Gpu::try_as_ref().map(|gpu| &gpu.pen) },
    { Gpu::try_as_mut().map(|gpu| &mut gpu.pen) }
);

#[derive(Debug)]
pub struct GpuPen
{

}

impl GpuPen
{
    pub fn new() -> Self
    {
        Self{}
    }
}


impl ScopedFlow for GpuPen
{
    fn begin_flow_draw(&mut self)
    {
        /*
        self.big_mesh.clear();
        self.draw_calls.clear();

        assert_eq!(self.params.len(), 1, "Forget to pop a camera");

        let dcall_param = &mut self.default_param;
        let scissor = param.window_size.to_rect();
        let viewport = scissor.cast_into();
        dcall_param.scissor = scissor;
        dcall_param.viewport = viewport;
        self.params.replace(*dcall_param);
        self.draw_calls.param = *dcall_param;
        */
    }

    fn end_flow_draw(&mut self)
    {
        /*
        self.update_last_draw_call();
        assert_eq!(self.params.len(), 1, "Forget to pop a camera");
        */
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DrawCallParam
{
    pub camera  : Camera,
    pub viewport: Rect2,
    pub viewport_min_depth: float,
    pub viewport_max_depth: float,
    pub scissor : Rect2P,
}
impl Default for DrawCallParam
{
    fn default() -> Self {
        Self { camera: ___(), viewport: ___(), viewport_min_depth: 0., viewport_max_depth: 1., scissor: ___() }
    }
}