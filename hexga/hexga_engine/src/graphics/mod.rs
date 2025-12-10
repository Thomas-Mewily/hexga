use super::*;

pub use wgpu;
pub use hexga_graphics::*;

pub mod prelude
{
    pub use super::Pen;
    pub use hexga_graphics::prelude::*;
    pub(crate) use super::{AppGraphics,wgpu,GpuMessage};
}

singleton_single_thread_access!(
    pub Pen,
    AppGraphics,
    { App::try_read().map(|v|v.inner_reference.graphics.as_ref()).flatten().map(|v| v.into()) },
    { App::try_write().map(|v|v.inner_reference.graphics.as_mut()).flatten().map(|v| v.into()) }
);

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
    pub(crate) fn new(size: Point2, output: GpuInitOutput) -> Self
    {
        let surface = ConfiguredSurface::from_surface(output.surface.expect("failed to init the surface"), size);
        Self
        {
            surface: Some(surface)
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

    pub(crate) async fn init_gpu(window: Arc<WinitWindow>, mut param: GpuParam) -> GpuResult<Self>
    {
        let size: Point2 = window.inner_size().convert();
        let size = size.max(one());

        if param.compatible_surface.is_none()
        {
            param.compatible_surface = Some(window.into());
        }

        let output = Gpu::new(param).await?;
        Ok(Self::new(size, output))
    }

    pub(crate) async fn async_init_gpu(window: Arc<WinitWindow>, param: GpuParam, proxy: EventLoopProxy)
    {
        let _ = proxy.send_event(AppInternalEvent::Gpu(Self::init_gpu(window, param).await));
    }

    pub(crate) fn init(window: Arc<WinitWindow>, param: GpuParam, proxy: EventLoopProxy)
    {
        if App.graphics.is_some() { return; }

        Self::async_init_gpu(window, param, proxy).spawn();

        /*
        match Gpu::try_context()
        {
            Some(ctx) =>
            {
                todo!()
            },
            None =>
            {
                /*
                (async ||
                {
                    proxy.send_event(AppInternalEvent::Gpu(GpuMessage::InitGpu(Self::init_gpu(window).await)))
                }).spawn();
            */
                todo!();
                //let d = Gpu::default(GpuParam { compatible_surface: Some(&window) });
                //let instance = Instance::new();

                //Gpu::default(GpuParam { default_surface: () })
            },
        }*/
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


pub(crate) type GpuMessage = GpuResult<AppGraphics>;
/*
#[derive(Debug)]
pub(crate) enum GpuMessage
{
    InitGpu(GpuResult<AppGraphics>),
    //InitSurface(GpuResult<ConfiguredSurface<'static>>)
}*/