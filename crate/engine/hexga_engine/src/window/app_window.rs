use super::*;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitWindowAttributes = winit::window::WindowAttributes;
pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindowShared = Arc<WinitWindow>;

#[derive(Debug, Default)]
pub struct Window
{
    pub(crate) param : DirtyFlag<WindowParam>,
    //pub(crate) winit_param: WinitWindowAttributes,

    pub(crate) window: Option<WinitWindowShared>,
    pub(crate) surface: Option<GpuConfiguredSurface<'static>>,
}

impl Window
{
    pub(crate) fn init_window_if_needed(&mut self, active: &WinitEventLoopActive) -> bool
    {
        if self.window.is_some()
        {
            return false;
        }

        #[allow(unused_mut)]
        let mut win_attr = self.param.deref().clone().into();

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            win_attr = win_attr.with_append(true);
        }

        let window = WinitWindowShared::new(active.create_window(win_attr).expect("can't create window"));
        self.window = Some(window);
        true
    }

    pub(crate) fn init_surface_if_needed(&mut self) -> bool
    {
        if Gpu::is_not_init()
        {
            return false;
        }
        if self.surface.is_some()
        {
            return false;
        }

        let Some(window) = &self.window else { return false; };
        let shared_window = window.clone();
        

        let size = self.size();
        let surface = Gpu
            .wgpu
            .wgpu_instance()
            .create_surface(shared_window)
            .expect("failed to create the window");
        self.surface = Some(GpuConfiguredSurface::from_surface(surface.into(), size));

        true
    }
}

impl GetPosition<int,2> for Window
{
    fn pos(&self) -> Vector<int, 2> {
        self.param.pos()
    }
}
impl SetPosition<int,2> for Window
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self {
        self.param.set_pos(pos); 
        self
    }
}
impl GetSize<int,2> for Window
{
    fn size(&self) -> Vector<int, 2> {
        self.param.size()
    }
}
impl SetSize<int,2> for Window
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self {
        self.param.set_size(size); self
    }
}

impl WindowAttribute for Window
{
    fn title(&self) -> &str {
        self.param.title()
    }

    fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.param.set_title(title);
        self
    }

    fn level(&self) -> WindowLevel {
        self.param.level()
    }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self {
        self.param.set_level(level);
        self
    }

    fn is_resizable(&self) -> bool {
        self.param.is_resizable()
    }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.param.set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags {
        self.param.buttons()
    }

    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self {
        self.param.set_buttons(buttons);
        self
    }

    fn maximised(&self) -> bool {
        self.param.maximised()
    }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self {
        self.param.set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool {
        self.param.is_visible()
    }

    fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.param.set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool {
        self.param.is_transparent()
    }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        self.param.set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool {
        self.param.have_blur()
    }

    fn set_blur(&mut self, blur: bool) -> &mut Self {
        self.param.set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool {
        self.param.have_decoration()
    }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self {
        self.param.set_decoration(decorations);
        self
    }

    fn is_content_protected(&self) -> bool {
        self.param.is_content_protected()
    }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self {
        self.param.set_content_protected(protected);
        self
    }

    fn is_active(self) -> bool {
        self.param.is_active()
    }

    fn set_active(&mut self, active: bool) -> &mut Self {
        self.param.set_active(active);
        self
    }
}

pub trait WindowAttribute: 
    Sized + GetSize<int,2> + SetSize<int,2> + GetPosition<int,2> + SetPosition<int,2>
{
    fn title(&self) -> &str;
    fn set_title(&mut self, title: impl Into<String>) -> &mut Self;
    fn with_title(mut self, title: impl Into<String>) -> Self { self.set_title(title); self}

    fn level(&self) -> WindowLevel;
    fn set_level(&mut self, level: WindowLevel) -> &mut Self;
    fn with_level(mut self, level: WindowLevel) -> Self { self.set_level(level); self }

    fn is_resizable(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool) -> &mut Self;
    fn with_resizable(mut self, resizable: bool) -> Self { self.set_resizable(resizable); self }

    fn buttons(&mut self) -> WindowButtonFlags;
    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self;
    fn with_buttons(mut self, buttons: WindowButtonFlags) -> Self { self.set_buttons(buttons); self }

    fn maximised(&self) -> bool;
    fn set_maximized(&mut self, maximized: bool) -> &mut Self;
    fn with_maximized(mut self, maximized: bool) -> Self { self.set_maximized(maximized); self }

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool) -> &mut Self;
    fn with_visible(mut self, visible: bool) -> Self { self.set_visible(visible); self }

    fn is_transparent(&self) -> bool;
    fn set_transparent(&mut self, transparent: bool) -> &mut Self;
    fn with_transparent(mut self, transparent: bool) -> Self { self.set_transparent(transparent); self }

    fn have_blur(&self) -> bool;
    fn set_blur(&mut self, blur: bool) -> &mut Self;
    fn with_blur(mut self, blur: bool) -> Self { self.set_blur(blur); self }

    fn have_decoration(&self) -> bool;
    fn set_decoration(&mut self, decorations: bool) -> &mut Self;
    fn with_decoration(mut self, decorations: bool) -> Self { self.set_decoration(true); self }

    fn is_content_protected(&self) -> bool;
    fn set_content_protected(&mut self, protected: bool) -> &mut Self;
    fn with_content_protected(mut self, protected: bool) -> Self { self.set_content_protected(protected); self }

    fn is_active(self) -> bool;
    fn set_active(&mut self, active: bool) -> &mut Self;
    fn with_active(mut self, active: bool) -> Self { self.set_active(active); self }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct WindowParam
{
    pub title: String,
    pub size: Point2,
    pub position: Point2,
    pub level: WindowLevel,
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
    fn default() -> Self
    {
        Self 
        {
            title: String::new(),
            size: Point2::ZERO,
            position: Point2::ZERO,
            level: ___(),
            resizable: true,
            buttons: ___(),
            maximized: false,
            visible: true,
            transparent: false,
            blur: false,
            decorations: true,
            content_protected: false,
            active: true,
        }
    }
}

impl GetPosition<int,2> for WindowParam
{
    fn pos(&self) -> Vector<int, 2> {
        self.position
    }
}
impl SetPosition<int,2> for WindowParam
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self {
        self.position = pos; self
    }
}
impl GetSize<int,2> for WindowParam
{
    fn size(&self) -> Vector<int, 2> {
        self.size
    }
}
impl SetSize<int,2> for WindowParam
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self {
        self.size = size; self
    }
}


impl WindowAttribute for WindowParam
{
    fn title(&self) -> &str { &self.title }
    fn set_title(&mut self, title: impl Into<String>) -> &mut Self {  
        self.title = title.into(); 
        self 
    }

    fn level(&self) -> WindowLevel {
        self.level
    }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self {
        self.level = level;
        self
    }

    fn is_resizable(&self) -> bool {
        self.resizable
    }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags {
        self.buttons
    }

    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self {
        self.buttons = buttons;
        self
    }

    fn maximised(&self) -> bool {
        self.maximized
    }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self {
        self.maximized = maximized;
        self
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }

    fn is_transparent(&self) -> bool {
        self.transparent
    }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        self.transparent = transparent;
        self
    }

    fn have_blur(&self) -> bool {
        self.blur
    }

    fn set_blur(&mut self, blur: bool) -> &mut Self {
        self.blur = blur;
        self
    }

    fn have_decoration(&self) -> bool {
        self.decorations
    }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self {
        self.decorations = decorations;
        self
    }

    fn is_content_protected(&self) -> bool {
        self.content_protected
    }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self {
        self.content_protected = protected;
        self
    }
    
    fn is_active(self) -> bool {
        self.active
    }
    
    fn set_active(&mut self, active: bool) -> &mut Self {
        self.active = active; self
    }
}

impl From<WindowParam> for WinitWindowAttributes
{
    fn from(value: WindowParam) -> Self
    {
        let WindowParam {
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
            decorations: decoration,
            content_protected,
            active,
        } = value;

        let mut attr = winit::window::Window::default_attributes();

        // Todo do better than a special ZERO value
        let size = if size.area().is_zero()
        {
            None
        }else
        {
            Some(to_winit_size(size).into())
        };

        attr.inner_size = size;
        attr.min_inner_size = None;
        attr.max_inner_size = None;
        attr.position = Some(to_winit_pos(position).into());
        attr.resizable = resizable;
        attr.enabled_buttons = buttons.into();
        attr.title = title;
        attr.maximized = maximized;
        attr.visible = visible;
        attr.transparent = transparent;
        attr.blur = blur;
        attr.decorations = decoration;
        attr.content_protected = content_protected;
        attr.window_level = level.into();
        attr.active = active;
        attr.cursor = ___(); // Todo expose the cursor
        attr.fullscreen = None;

        attr
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


/// A window level groups windows with respect to their z-position.
///
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of a window within the same window level may change dynamically on user interaction.
///
/// ## Platform-specific
///
/// - **iOS / Android / Web / Wayland:** Unsupported.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum WindowLevel
{
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
    fn from(value: WindowLevel) -> Self
    {
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
    fn default() -> Self { Self::ALL }
}
impl From<WindowButton> for winit::window::WindowButtons
{
    fn from(value: WindowButton) -> Self
    {
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
    fn from(value: WindowButtonFlags) -> Self
    {
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
    fn from(value: winit::window::WindowButtons) -> Self
    {
        let mut flags = WindowButtonFlags::EMPTY;
        for button in value
        {
            if button == winit::window::WindowButtons::CLOSE
            {
                flags |= WindowButton::Close;
            }
            if button == winit::window::WindowButtons::MINIMIZE
            {
                flags |= WindowButton::Minimize;
            }
            if button == winit::window::WindowButtons::MAXIMIZE
            {
                flags |= WindowButton::Maximize;
            }
        }
        flags
    }
}
