use super::*;

pub use wgpu;
pub use hexga_graphics::*;

pub mod prelude
{
    pub use super::Pen;
    pub use hexga_graphics::prelude::*;
    pub(crate) use super::{AppGraphics,wgpu,GpuMessage};
}

singleton_single_thread_project!(pub Pen, AppGraphics, App, graphics);

#[derive(Debug)]
pub struct AppGraphics
{
    /// Destroyed on suspend and recreated on resume
    pub(crate) surface: Option<ConfiguredSurface<'static>>,

    /*
    pub(crate) binding: GpuBinding,
    pub(crate) render: GpuRender,

    pub(crate) immediate_mesh: Option<Mesh>,
    pub(crate) background_color : Option<Color>,
    pub(crate) white_pixel: Option<Texture>,
    */
}

impl AppGraphics
{
    pub(crate) fn new() -> Self
    {
        Self
        {
            surface: None
        }
    }
    pub(crate) fn resize(&mut self, size: Point2)
    {
        self.surface_mut().resize(size);
    }

    pub(crate) fn surface_mut(&mut self) -> &mut ConfiguredSurface<'static>
    {
        self.surface.as_mut().expect("surface was not init")
    }

    pub(crate) fn gpu_event(&mut self, msg: GpuMessage)
    {
        match msg
        {
            GpuMessage::InitSurface(surface) =>
            {
                self.surface = Some(surface.expect("failed to create the surface"));
            },
        }
    }
}


impl ScopedFlow for Option<AppGraphics>
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.as_mut().map(|gpu| gpu.begin_flow(flow));
        self.dispatch_begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.as_mut().map(|gpu| gpu.end_flow(flow));
        self.dispatch_end_flow(flow);
    }

    fn begin_flow_resumed(&mut self) {
        if self.is_none()
        {
            if let Some(w) = App.window.active.as_ref()
            {
                AppGraphics::init_surface(w.clone(), App.proxy().clone());
            }
        }
    }
}
impl ScopedFlow for AppGraphics
{
    /*
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.render.begin_flow(flow);
        self.dispatch_begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.render.end_flow(flow);
        self.dispatch_end_flow(flow);
    }
    */

    fn end_flow_draw(&mut self)
    {
        //todo!();
        //self.send_data_to_gpu();
    }
}


impl AppGraphics
{
    pub(crate) fn init_surface(window: Arc<WinitWindow>, proxy : EventLoopProxy)
    {

    }
}

#[derive(Debug)]
pub(crate) enum GpuMessage
{
    InitSurface(GpuResult<ConfiguredSurface<'static>>)
}