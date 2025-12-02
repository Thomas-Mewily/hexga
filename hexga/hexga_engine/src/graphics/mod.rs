use super::*;

pub mod prelude
{
    pub(crate) use super::*;
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
    pub(crate) surface: Surf,

    pub(crate) binding: GpuBinding,
    pub(crate) render: GpuRender,

    pub(crate) immediate_mesh: Option<Mesh>,
    pub(crate) background_color : Option<Color>,
    pub(crate) white_pixel: Option<Texture>,
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
                AppPen::request(w.clone(), App.proxy.as_ref().unwrap().clone()).unwrap();
            }
        }
    }
}