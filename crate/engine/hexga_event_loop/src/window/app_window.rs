use super::*;

use experimental::*;
pub mod experimental
{
    use super::*;
    pub type WinitWindow = winit::window::Window;
    pub type WinitWindowAttributes = winit::window::WindowAttributes;
    pub type WinitWindowID = winit::window::WindowId;
    pub type WinitWindowShared = Arc<WinitWindow>;
}

#[derive(Debug)]
pub struct Window<Surface>
    where Surface: Clone
{
    pub(crate) param : DirtyFlag<WindowParam>,

    pub(crate) is_pos_dirty : bool,
    pub(crate) is_size_dirty : bool,
    pub(crate) is_content_protected_dirty : bool,
    //pub(crate) winit_param: WinitWindowAttributes,

    pub(crate) window: WinitWindowShared,
    pub(crate) surface: Option<Surface>,
}

pub type WindowError = ();
pub type WindowResult<T> = Result<T,WindowError>;

pub trait WindowManager<Surface>
    where Surface: Clone
{
    fn create_window(&mut self, param: WindowParam) -> WindowResult<Window<Surface>>;
}

pub trait Windowable<Surface>
{
    fn request_draw(&mut self);
    fn request_user_attention(&mut self, request_type : impl Into<Option<UserAttentionType>>);

    fn surface(&self) -> Option<Surface>;
    fn set_surface(&mut self, surface: Option<Surface>) -> &mut Self;

    /// Lib specific method
    fn winit_window(&self) -> WinitWindowShared;
}


impl<Surface> Window<Surface>
    where Surface: Clone
{
    pub(crate) fn set_dirty(&mut self, dirty: bool)
    {
        self.param.set_dirty(dirty);
        self.is_pos_dirty = dirty;
        self.is_size_dirty = dirty;
        self.is_content_protected_dirty = dirty;
    }

    /*
    pub(crate) fn init_window_if_needed<User>(&mut self, active: &EventLoop<User>) -> bool
        where User: PlatformCustomEvent
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

        let window = WinitWindowShared::new(active.winit_event_loop().create_window(win_attr).expect("can't create window"));
        self.window = Some(window);
        self.set_dirty(true);
        true
    }*/

    pub(crate) fn undirty_if_needed(&mut self)
    {
        if !self.param.is_dirty() { return; }
        //let Some(window) = &self.window else { return; };
        let window = &self.window;

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
            decoration, 
            content_protected, 
            active, 
            //cursor_visible 
        } = self.param.deref();

        window.set_title(title);

        if size.area().is_non_zero() && self.is_size_dirty
        {
            window.request_inner_size(to_winit_size(*size));
        }

        if self.is_pos_dirty
        {
            window.set_outer_position(to_winit_pos(*position));
        }
        window.set_window_level((*level).into());

        if window.is_visible() != Some(!visible)
        {
            window.set_visible(*visible);
        }

        window.set_resizable(*resizable);

        let buttons = (*buttons).into();
        if window.enabled_buttons() != buttons
        {
            window.set_enabled_buttons(buttons);
        }

        if window.is_maximized() != *maximized 
        {
            window.set_maximized(*maximized);
        }
        window.set_transparent(*transparent);
        window.set_blur(*blur);

        if window.is_decorated() != *decoration
        {
            window.set_decorations(*decoration);
        }

        if self.is_content_protected_dirty != *content_protected
        {
            window.set_content_protected(*content_protected);
        }
        
        // window.set_cursor_grab(attr.cursor_grab).ok();
        // window.set_cursor_visible(cursor_visible);

        self.set_dirty(false);
    }

    /*
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
        self.set_dirty(true);

        true
    }
    */
}

impl<Surface> Windowable<Surface> for Window<Surface>
    where Surface: Clone
{
    fn request_draw(&mut self)
    {
        self.window.request_redraw()
    }

    fn request_user_attention(&mut self, request_type : impl Into<Option<UserAttentionType>>)
    {
        let request_type = request_type.into();
        self.window.request_user_attention(request_type.map(|v| v.into()));
    }
    
    /// Lib specific method, expose impl details
    #[self::unstable]
    fn winit_window(&self) -> WinitWindowShared {
        self.window.clone()
    }
    
    fn surface(&self) -> Option<Surface> {
        self.surface.clone()
    }
    
    fn set_surface(&mut self, surface: Option<Surface>) -> &mut Self {
        self.surface = surface; self
    }
}


impl<Surface> GetPosition<int,2> for Window<Surface>
    where Surface: Clone
{
    fn pos(&self) -> Vector<int, 2> 
    {
        match &self.window.outer_position()
        {
            Ok(pos) => { return pos.convert(); },
            Err(e) => self.param.pos(),
        }
    }
}
impl<Surface> SetPosition<int,2> for Window<Surface>
    where Surface: Clone
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self {
        self.param.set_pos(pos); 
        self.is_pos_dirty = true;
        self
    }
}
impl<Surface> GetSize<int,2> for Window<Surface>
    where Surface: Clone
{
    fn size(&self) -> Vector<int, 2> {
        self.window.inner_size().convert()
    }
}
impl<Surface> SetSize<int,2> for Window<Surface>
    where Surface: Clone
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self {
        self.param.set_size(size); 
        self.is_size_dirty = true;
        self
    }
}

impl<Surface> WindowAttribute for Window<Surface>
    where Surface: Clone
{
    fn title(&self) -> String {
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
        self.is_content_protected_dirty = true;
        self
    }

    fn is_active(&self) -> bool {
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
    fn title(&self) -> String;
    fn set_title(&mut self, title: impl Into<String>) -> &mut Self;
    fn with_title(mut self, title: impl Into<String>) -> Self { self.set_title(title); self }

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

    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool) -> &mut Self;
    fn with_active(mut self, active: bool) -> Self { self.set_active(active); self }

    /* Not related to window
    fn is_cursor_visible(&self) -> bool;
    fn set_cursor_visible(&mut self, visible: bool) -> &mut Self;
    fn with_cursor_visible(mut self, visible: bool) -> Self { self.set_cursor_visible(visible); self}
    */
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
    pub decoration: bool,
    pub content_protected: bool,
    pub active: bool,
    //pub cursor_visible: bool,
}
impl Default for WindowParam
{
    fn default() -> Self
    {
        Self 
        {
            title: "hexga app".to_owned(),
            size: Point2::ZERO,
            position: Point2::ZERO,
            level: ___(),
            resizable: true,
            buttons: ___(),
            maximized: false,
            visible: true,
            transparent: false,
            blur: false,
            decoration: true,
            content_protected: false,
            active: true,
            //cursor_visible: true,
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
    fn title(&self) -> String { self.title.clone() }
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
        self.decoration
    }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self {
        self.decoration = decorations;
        self
    }

    fn is_content_protected(&self) -> bool {
        self.content_protected
    }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self {
        self.content_protected = protected;
        self
    }
    
    fn is_active(&self) -> bool {
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
            decoration,
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

/// ## Platform-specific
///
/// - **X11:** Sets the WM's `XUrgencyHint`. No distinction between [`Critical`] and
///   [`Informational`].
///
/// [`Critical`]: Self::Critical
/// [`Informational`]: Self::Informational
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UserAttentionType {
    /// ## Platform-specific
    ///
    /// - **macOS:** Bounces the dock icon until the application is in focus.
    /// - **Windows:** Flashes both the window and the taskbar button until the application is in
    ///   focus.
    Critical,

    /// ## Platform-specific
    ///
    /// - **macOS:** Bounces the dock icon once.
    /// - **Windows:** Flashes the taskbar button until the application is in focus.
    #[default]
    Informational,
}
impl From<UserAttentionType> for winit::window::UserAttentionType
{
    fn from(value: UserAttentionType) -> Self {
        match value
        {
            UserAttentionType::Critical => winit::window::UserAttentionType::Critical,
            UserAttentionType::Informational => winit::window::UserAttentionType::Informational,
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