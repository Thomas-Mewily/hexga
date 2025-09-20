use super::*;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindowShared = Arc<WinitWindow>;

#[derive(Debug)]
pub struct AppWindows
{
    pub(crate) active : Option<WinitWindowShared>,
}

impl AppWindows
{
    pub(crate) fn new() -> Self { Self{ active: None } }
}
impl<E> ScopedMessage<E> for AppWindows where E: IEvent
{
    fn begin_flow_resumed(&mut self, ctx: MessageCtx<'_,E>) {
        if self.active.is_none()
        {
            #[allow(unused_mut)]
            let mut win_attr = WinitWindow::default_attributes().with_title("wgpu winit example");
            
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            let window = Arc::new(
                ctx.event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.active = Some(window.clone());
            AppGpu::request(window, ctx.proxy.clone()).unwrap();
        }
    }

    fn begin_window(&mut self, window: &WindowEvent, _ctx: MessageCtx<'_,E>) {
        if let Some(active) = &mut self.active
        {
            match window
            {
                WindowEvent::Resized(size) => 
                {
                    Gpu.resize(*size);
                    active.request_redraw();
                },
            }
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum WindowEvent
{
    Resized(Point2),
}