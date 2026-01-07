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
    pub(crate) param: winit::window::WindowAttributes,
    pub(crate) window : Option<Window>,
}
impl AppWindow
{
    pub(crate) fn new() -> Self { Self { window: None, param: winit::window::WindowAttributes::default() }}

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

    pub(crate) fn init_if_needed(&mut self, active: &EventLoopActive) -> bool
    {
        if self.window.is_some() { return false; }

        #[allow(unused_mut)]
        let mut win_attr = self.param.clone();

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            win_attr = win_attr.with_append(true);
        }

        let window = Arc::new(active
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
    pub(crate) surface: Option<GpuConfiguredSurface<'static>>,
}
impl Window
{
    pub(crate) fn from_winit(window: WinitWindowShared) -> Self
    {
        Self { window, surface: None }
    }
    pub(crate) fn set_surface(&mut self, surface: GpuSurface<'static>)
    {
        let surface = GpuConfiguredSurface::from_surface(surface, self.size());
        self.surface = Some(surface);
    }
    pub(crate) fn drop_surface(&mut self)
    {
        self.surface = None;
    }

    pub(crate) fn init_surface_if_needed(&mut self)
    {
        if Gpu::is_not_init() { return; }
        if self.surface.is_some() { return; }

        let size = self.size();
        let surface = Gpu.wgpu.wgpu_instance().create_surface(self.window.clone()).expect("failed to create the window");
        self.surface = Some(GpuConfiguredSurface::from_surface(surface.into(), size));
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

pub(crate) fn to_winit_size(size: Point2) -> winit::dpi::PhysicalSize<i32>
{
    winit::dpi::PhysicalSize::new(size.x as i32, size.y as i32)
}
pub(crate) fn to_winit_pos(size: Point2) -> winit::dpi::PhysicalPosition<i32>
{
    winit::dpi::PhysicalPosition::new(size.x as i32, size.y as i32)
}

impl SetSize<int,2> for Window
{
    fn set_size(&mut self, size: Vector<int,2>) -> &mut Self
    {
        let size = size.max(one());
        let _ = self.window.request_inner_size(to_winit_size(size));

        self.configure_surface();
        self
    }
}

pub trait WindowParamBuilder: Sized
{
    fn with_title(self, title: impl Into<String>) -> Self;
    fn with_size(self, size: impl Into<Option<Point2>>) -> Self;
    fn with_position(self, position: impl Into<Option<Point2>>) -> Self;
    fn with_level(self, level: WindowLevel) -> Self;

    fn with_resizable(self, resizable: bool) -> Self;
    fn with_buttons(self, buttons: WindowButtonFlags) -> Self;
    fn with_maximized(self, maximized: bool) -> Self;

    fn with_visible(self, visible: bool) -> Self;
    fn with_transparent(self, transparent: bool) -> Self;
    fn with_blur(self, blur: bool) -> Self;

    fn with_decorations(self, decorations: bool) -> Self;
    fn with_content_protected(self, protected: bool) -> Self;

    fn with_active(self, active: bool) -> Self;
}
impl<T> WindowParamBuilder for T where T: HasMut<WindowParam> {
    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.retrive_mut().title = title.into();
        self
    }

    fn with_size(mut self, size: impl Into<Option<Point2>>) -> Self {
        self.retrive_mut().size = size.into();
        self
    }

    fn with_position(mut self, position: impl Into<Option<Point2>>) -> Self {
        self.retrive_mut().position = position.into();
        self
    }

    fn with_level(mut self, level: WindowLevel) -> Self {
        self.retrive_mut().level = level;
        self
    }

    fn with_resizable(mut self, resizable: bool) -> Self {
        self.retrive_mut().resizable = resizable;
        self
    }

    fn with_buttons(mut self, buttons: WindowButtonFlags) -> Self {
        self.retrive_mut().buttons = buttons;
        self
    }

    fn with_maximized(mut self, maximized: bool) -> Self {
        self.retrive_mut().maximized = maximized;
        self
    }

    fn with_visible(mut self, visible: bool) -> Self {
        self.retrive_mut().visible = visible;
        self
    }

    fn with_transparent(mut self, transparent: bool) -> Self {
        self.retrive_mut().transparent = transparent;
        self
    }

    fn with_blur(mut self, blur: bool) -> Self {
        self.retrive_mut().blur = blur;
        self
    }

    fn with_decorations(mut self, decorations: bool) -> Self {
        self.retrive_mut().decorations = decorations;
        self
    }

    fn with_content_protected(mut self, protected: bool) -> Self {
        self.retrive_mut().content_protected = protected;
        self
    }

    fn with_active(mut self, active: bool) -> Self {
        self.retrive_mut().active = active;
        self
    }
}


#[non_exhaustive]
#[derive(Debug)]
pub struct WindowParam
{
    pub title: String,
    pub size: Option<Point2>,
    pub position: Option<Point2>,
    pub level : WindowLevel,
    pub resizable: bool,
    pub buttons: WindowButtonFlags,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub content_protected: bool,
    pub active: bool,
}
impl Default for WindowParam
{
    fn default() -> Self {
        Self
        {
            title: "hexga application".to_owned(),
            size: None,
            position: None,
            level: ___(),
            resizable: true,
            buttons: ___(),
            maximized: false,
            visible: true,
            transparent: false,
            blur: false,
            decorations: true,
            content_protected: false,
            active: true
        }
    }
}

impl From<WindowParam> for winit::window::WindowAttributes
{
    fn from(value: WindowParam) -> Self
    {
        let WindowParam
        {
            title,
            size,
            position,
            level,
            resizable,
            buttons,
            maximized,
            visible,
            transparent,
            blur,
            decorations,
            content_protected,
            active,
        } = value;

        let mut attr = winit::window::Window::default_attributes();

        attr.inner_size = size.map(|v| to_winit_size(v).into());
        attr.min_inner_size = None;
        attr.max_inner_size = None;
        attr.position = position.map(|v| to_winit_pos(v).into());
        attr.resizable = resizable;
        attr.enabled_buttons = buttons.into();
        attr.title = title;
        attr.maximized = maximized;
        attr.visible = visible;
        attr.transparent = transparent;
        attr.blur = blur;
        attr.decorations = decorations;
        attr.content_protected = content_protected;
        attr.window_level = level.into();
        attr.active = active;
        attr.cursor = ___();
        attr.fullscreen = None;

        attr
    }
}




/// A window level groups windows with respect to their z-position.
///
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of a window within the same window level may change dynamically on user interaction.
///
/// ## Platform-specific
///
/// - **iOS / Android / Web / Wayland:** Unsupported.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum WindowLevel {
    /// The window will always be below normal windows.
    ///
    /// This is useful for a widget-based app.
    AlwaysOnBottom,

    /// The default.
    #[default]
    Normal,

    /// The window will always be on top of normal windows.
    AlwaysOnTop,
}
impl From<WindowLevel> for winit::window::WindowLevel
{
    fn from(value: WindowLevel) -> Self {
        use winit::window::WindowLevel as W;
        match value
        {
            WindowLevel::AlwaysOnBottom => W::AlwaysOnBottom,
            WindowLevel::Normal => W::Normal,
            WindowLevel::AlwaysOnTop => W::AlwaysOnTop,
        }
    }
}

#[bit_index]
#[repr(u8)]
pub enum WindowButton
{
    Close = 0,
    Minimize = 1,
    Maximize = 2,
    All = Self::Close | Self::Minimize | Self::Maximize,
}
impl Default for WindowButtonFlags
{
    fn default() -> Self {
        Self::ALL
    }
}
impl From<WindowButton> for winit::window::WindowButtons
{
    fn from(value: WindowButton) -> Self {
        use winit::window::WindowButtons as B;
        match value
        {
            WindowButton::Close => B::CLOSE,
            WindowButton::Minimize => B::MINIMIZE,
            WindowButton::Maximize => B::MAXIMIZE,
        }
    }
}
impl From<WindowButtonFlags> for winit::window::WindowButtons
{
    fn from(value: WindowButtonFlags) -> Self {
        let mut flags = winit::window::WindowButtons::empty();
        for button in value
        {
            flags |= button.into();
        }
        flags
    }
}
impl From<winit::window::WindowButtons> for WindowButtonFlags
{
    fn from(value: winit::window::WindowButtons) -> Self {
        let mut flags = WindowButtonFlags::EMPTY;
        for button in value
        {
            if button == winit::window::WindowButtons::CLOSE { flags |= WindowButton::Close; }
            if button == winit::window::WindowButtons::MINIMIZE { flags |= WindowButton::Minimize; }
            if button == winit::window::WindowButtons::MAXIMIZE { flags |= WindowButton::Maximize; }
        }
        flags
    }
}