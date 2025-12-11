use winit::{platform::windows::WindowAttributesExtWindows};
use super::*;

//singleton_single_thread_project!(pub Window, AppWindow, App, window );
pub type WinitWindow = winit::window::Window;
pub type WinitWindowID = winit::window::WindowId;
pub type WinitWindowShared = Arc<WinitWindow>;

pub trait WindowExtension :
    GetPosition<int,2> +
    SetPosition<int,2> +
    GetSize<int,2> +
    SetSize<int,2> +
{
    fn request_draw(&mut self);
    fn configure_surface(&mut self);
}

#[derive(Debug)]
pub struct AppWindow
{
    pub(crate) window : Option<Window>,
}
impl AppWindow
{
    pub(crate) fn new() -> Self { Self { window: None }}

    pub(crate) fn set_window(&mut self, window : Option<Window>)
    {
        self.window = window;
        match &mut self.window
        {
            Some(w) =>
            {
                w.request_draw();
            },
            None => {},
        }
    }

    pub(crate) fn destroy(&mut self)
    {
        self.window = None;
    }

    pub(crate) fn init_surface_if_needed(&mut self)
    {
        match self.try_main_window_mut()
        {
            Some(w) => w.init_surface_if_needed(),
            None => {},
        }
    }

    pub(crate) fn init_window_if_needed(&mut self, active: &EventLoopActive) -> bool
    {
        if self.window.is_some() { return false; }

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
        self.window = Some(Window::from_winit(window));
        true
    }

    pub fn main_window(&self) -> &Window { self.try_main_window().unwrap() }
    pub fn main_window_mut(&mut self) -> &mut Window { self.try_main_window_mut().unwrap() }

    pub fn try_main_window(&self) -> Option<&Window> { self.window.as_ref() }
    pub fn try_main_window_mut(&mut self) -> Option<&mut Window> { self.window.as_mut() }

}

impl WindowExtension for AppWindow
{
    fn request_draw(&mut self) {
        match self.try_main_window_mut()
        {
            Some(w) => w.request_draw(),
            None => {},
        }
    }

    fn configure_surface(&mut self)
    {
        match self.try_main_window_mut()
        {
            Some(w) => w.configure_surface(),
            None => {},
        }
    }
}
impl GetPosition<int,2> for AppWindow
{
    fn pos(&self) -> Point2
    {
        match self.try_main_window()
        {
            Some(w) => w.pos(),
            None => zero(),
        }
    }
}
impl SetPosition<int,2> for AppWindow
{
    fn set_pos(&mut self, pos: Point2) -> &mut Self
    {
        match self.try_main_window_mut()
        {
            Some(w) => { w.set_pos(pos); }
            None => {},
        }
        self
    }
}
impl GetSize<int,2> for AppWindow
{
    fn size(&self) -> Vector<int,2>
    {
        match self.try_main_window()
        {
            Some(w) => w.size(),
            None => one(),
        }
    }
}
impl SetSize<int,2> for AppWindow
{
    fn set_size(&mut self, size: Vector<int,2>) -> &mut Self
    {
        match self.try_main_window_mut()
        {
            Some(w) => { w.set_size(size); },
            None => {},
        }
        self
    }
}


#[derive(Debug)]
pub struct Window
{
    pub(crate) window : WinitWindowShared,
    /// Destroyed on suspend and recreated on resume
    pub(crate) surface: Option<ConfiguredSurface<'static>>,
}
impl Window
{
    pub(crate) fn from_winit(window: WinitWindowShared) -> Self
    {
        Self { window, surface: None }
    }
    pub(crate) fn set_surface(&mut self, surface: hexga_graphics::Surface<'static>)
    {
        let surface = ConfiguredSurface::from_surface(surface, self.size());
        self.surface = Some(surface);
    }
    pub(crate) fn drop_surface(&mut self)
    {
        self.surface = None;
    }

    pub(crate) fn init_surface_if_needed(&mut self)
    {
        if App.graphics.is_none() { return; }
        if self.surface.is_some() { return; }

        let size = self.size();
        let surface = Gpu.wgpu.wgpu_instance().create_surface(self.window.clone()).expect("failed to create the window");
        self.surface = Some(ConfiguredSurface::from_surface(surface.into(), size));
    }
}
impl WindowExtension for Window
{
    fn request_draw(&mut self) {
        self.window.request_redraw();
    }
    fn configure_surface(&mut self)
    {
        let size= self.size();
        self.init_surface_if_needed();
        self.surface.as_mut().map(|s|
            {
                s.resize(size);
                self.window.request_redraw();
            }
        );
    }
}
impl GetPosition<int,2> for Window
{
    fn pos(&self) -> Point2
    {
        self.window.outer_position().ok().map(|p| p.convert()).unwrap_or(zero())
    }
}
impl SetPosition<int,2> for Window
{
    fn set_pos(&mut self, pos: Point2) -> &mut Self
    {
        self.window.set_outer_position(winit::dpi::PhysicalPosition::new(pos.x, pos.y));
        self
    }
}
impl GetSize<int,2> for Window
{
    fn size(&self) -> Vector<int,2>
    {
        let size : Point2 = self.window.inner_size().convert();
        size.max(one())
    }
}
impl SetSize<int,2> for Window
{
    fn set_size(&mut self, size: Vector<int,2>) -> &mut Self
    {
        let size = size.max(one());
        let _ = self.window.request_inner_size(winit::dpi::PhysicalSize::new(size.x as i32, size.y as i32));

        self.configure_surface();
        self
    }
}
