use winit::platform::windows::WindowAttributesExtWindows;

use super::*;

singleton_single_thread_project!(pub Window, AppWindow, App, window );

#[derive(Debug)]
pub struct AppWindow
{
    pub(crate) active : Option<WinitWindowShared>,
}

impl AppWindow
{
    pub(crate) fn new() -> Self { Self{ active: None } }

    pub(crate) fn as_ref(&self) -> Option<&WinitWindow> { self.active.as_ref().map(|w| w.as_ref()) }

    pub(crate) fn destroy(&mut self)
    {
        self.active = None;
    }

    pub(crate) fn begin_resumed_with_active_loop(&mut self, active: &EventLoopActive)
    {
        if self.active.is_none()
        {
            #[allow(unused_mut)]
            let mut win_attr = WinitWindow::default_attributes()
                .with_title(App.param.title.to_owned());

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

            AppGraphics::init(window, proxy);
            //AppGpu::request(window, ctx.proxy.clone()).unwrap();
        }
    }

    pub fn request_draw(&mut self)
    {
        self.active.as_ref().map(|w| w.request_redraw());
    }
}

impl GetPosition<int,2> for AppWindow
{
    fn pos(&self) -> Point2
    {
        self.active.as_ref().and_then(|w| w.outer_position().ok()).map(|p| p.convert()).unwrap_or(zero())
    }
}
impl SetPosition<int,2> for AppWindow
{
    fn set_pos(&mut self, pos: Point2) -> &mut Self
    {
        if let Some(active) = &mut self.active
        {
            let _ = active.set_outer_position(winit::dpi::PhysicalPosition::new(pos.x, pos.y));
        }
        self
    }
}
impl GetRectangle<int,2> for AppWindow
{
    fn size(&self) -> Vector<int,2>
    {
        self.active.as_ref().map(|w| w.inner_size().convert()).unwrap_or(one())
    }
}
impl SetRectangle<int,2> for AppWindow
{
    fn set_size(&mut self, size: Vector<int,2>) -> &mut Self
    {
        if let Some(active) = &mut self.active
        {
            let _ = active.request_inner_size(winit::dpi::PhysicalSize::new(size.x.max(1) as u32, size.y.max(1) as u32));
            self.request_draw();
        }
        self
    }
}
