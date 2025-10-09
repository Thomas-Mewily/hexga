use super::*;

#[derive(Debug)]
pub struct AppWindow
{
    pub(crate) active : Option<WinitWindowShared>,
}

impl AppWindow
{
    pub(crate) fn new() -> Self { Self{ active: None } }

    pub(crate) fn as_ref(&self) -> Option<&WinitWindow> { self.active.as_ref().map(|w| w.as_ref()) }

    pub(crate) fn resize(&mut self, size: Point2, active: &EventLoopActive)
    {
        if let Some(active) = &mut self.active
        {
            active.request_redraw();
        }
    }

    pub(crate) fn begin_resumed(&mut self, active: &EventLoopActive)
    {
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
                active
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.active = Some(window.clone());
            //AppGpu::request(window, ctx.proxy.clone()).unwrap();
        }
    }
}