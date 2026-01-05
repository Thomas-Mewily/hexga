use super::*;

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
    //pub(crate) immediate: ImmediateRender,
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
    /*
    pub(crate) fn new(size: Point2, output: GpuInitOutput) -> Self
    {
        let surface = ConfiguredSurface::from_surface(output.surface.expect("failed to init the surface"), size);
        Self
        {
            surface: Some(surface)
        }
    }*/

    pub(crate) fn new() -> Self
    {
        Self
        {

        }
    }

    pub(crate) async fn init_gpu(instance: gpu::GpuInstance, surface: Option<graphics::GpuSurface<'static>>, window: Arc<WinitWindow>, mut param: GpuParam) -> GpuResult<Self>
    {
        let gpu_init = GpuInit::from_instance_and_surface(instance, surface, param).await?;
        let output = Gpu::from_init(gpu_init).await?;

        Ok(Self::new())
    }

    pub(crate) async fn async_init_gpu(instance: gpu::GpuInstance, surface: Option<graphics::GpuSurface<'static>>, window: Arc<WinitWindow>, param: GpuParam, proxy: EventLoopProxy)
    {
        let _ = proxy.send_event(AppInternalEvent::Gpu(Self::init_gpu(instance, surface, window, param).await));
    }

    pub(crate) fn init(window: Arc<WinitWindow>, mut param: GpuParam, proxy: EventLoopProxy) -> GpuResult
    {
        if APP.graphics.is_some() { return Err(GpuError::GpuAlreadyInit); }

        let surface_size: Point2 = window.inner_size().convert();
        let surface_size = surface_size.max(one());

        if param.compatible_surface.is_none()
        {
            param.compatible_surface = Some(window.clone().into());
        }

        let instance = gpu::GpuInstance::new(&param.instance);
        let surface = Some(instance.wgpu.create_surface(param.compatible_surface.take().expect("missing surface"))?.into());

        Self::async_init_gpu(instance, surface, window, param, proxy).spawn();
        Ok(())
    }
}


/*
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

    /*
    fn begin_flow_resumed(&mut self) {
        if self.is_none()
        {
            if let Some(w) = App.window.active.as_ref()
            {
                AppGraphics::init_surface(w.clone(), App.proxy().clone());
            }
        }
    }*/
}
*/
/*
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


    fn begin_flow_resumed(&mut self) {
        self.init_surface(WindowState.active.as_ref().unwrap().clone());
    }
    fn end_flow_paused(&mut self) {
        self.surface = None;
    }

    fn end_flow_draw(&mut self)
    {
        //todo!();
        //self.send_data_to_gpu();
    }
}
*/


pub(crate) type GpuMessage = GpuResult<AppGraphics>;
/*
#[derive(Debug)]
pub(crate) enum GpuMessage
{
    InitGpu(GpuResult<AppGraphics>),
    //InitSurface(GpuResult<ConfiguredSurface<'static>>)
}*/