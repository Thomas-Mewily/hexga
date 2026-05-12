use super::*;

use experimental::*;
use hexga_image::image::Image;

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
{
    pub(crate) param: WindowParam,
    pub(crate) size: Point2,
    pub(crate) pos: Point2,
    pub(crate) window: WinitWindowShared,
    pub(crate) surface: Option<Surface>,
}

pub type WindowError = ();
pub type WindowResult<T> = Result<T, WindowError>;

pub trait WindowManager<Surface>
{
    fn create_window(&mut self, param: WindowParam) -> WindowResult<Window<Surface>>;
}

pub trait Windowable
{
    /// Returns the list of all the monitors available on the system.
    fn current_monitor(&self) -> Option<Monitor>;

    /// Gets whether the window has keyboard focus.
    fn has_focus(&self) -> bool;
    /// Brings the window to the front and sets input focus. Has no effect if the window is
    /// already in focus, minimized, or not visible.
    ///
    /// This method steals input focus from other applications. Do not use this method unless
    /// you are certain that's what the user wants. Focus stealing can cause an extremely disruptive
    /// user experience.
    fn focus(&mut self) -> &mut Self;

    /// Returns the primary monitor of the system.
    ///
    /// Returns `None` if it can't identify any monitor as a primary one.
    fn primary_monitor(&self) -> Option<Monitor>;

    fn is_minimised(&self) -> Option<bool>;
    fn set_minimised(&mut self, minimized: bool) -> &mut Self;

    /// Returns the list of all the monitors available on the system.
    fn available_monitors(&self) -> impl Iterator<Item = Monitor>;

    fn request_draw(&mut self) -> &mut Self;
    fn request_user_attention(&mut self, request_type: impl Into<Option<UserAttentionType>>);

    /// Modifies the cursor icon of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Orbital:** Unsupported.
    /// - **Web:** Custom cursors have to be loaded and decoded first, until then the previous
    ///   cursor is shown.
    fn set_cursor(&mut self, cursor: impl Into<Cursor>) -> &mut Self;

    /// Changes the position of the cursor in window coordinates.
    ///
    /// ## Platform-specific
    ///
    /// - **Wayland**: Cursor must be in [`CursorGrabMode::Locked`].
    /// - **iOS / Android / Web / Orbital:** Always returns an [`ExternalError::NotSupported`].
    fn set_cursor_pos(&mut self, pos: Point2) -> &mut Self;

    /// Modifies the cursor's visibility.
    ///
    /// If `false`, this will hide the cursor. If `true`, this will show the cursor.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** The cursor is only hidden within the confines of the window.
    /// - **X11:** The cursor is only hidden within the confines of the window.
    /// - **Wayland:** The cursor is only hidden within the confines of the window.
    /// - **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor
    ///   is outside of the window.
    /// - **iOS / Android:** Unsupported.
    fn set_cursor_visible(&mut self, visible: bool) -> &mut Self;

    /// Set grabbing [mode][CursorGrabMode] on the cursor preventing it from leaving the window.
    fn set_cursor_grab(&mut self, mode: CursorGrab) -> CursorResult;

    /// Modifies whether the window catches cursor events.
    ///
    /// If `true`, the window will catch the cursor events. If `false`, events are passed through
    /// the window such that any other window behind it receives them. By default hittest is
    /// enabled.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Orbital:** Always returns an [`ExternalError::NotSupported`].
    fn set_cursor_hittest(&mut self, hittest: bool) -> CursorResult;

    /// Lib specific method
    #[doc(hidden)]
    fn winit_window(&self) -> WinitWindowShared;

    fn destroy_surface(&mut self) -> &mut Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Theme
{
    Light,
    #[default]
    Dark,
}
impl From<Theme> for winit::window::Theme
{
    fn from(value: Theme) -> Self
    {
        match value
        {
            Theme::Light => winit::window::Theme::Light,
            Theme::Dark => winit::window::Theme::Light,
        }
    }
}
impl From<winit::window::Theme> for Theme
{
    fn from(value: winit::window::Theme) -> Self
    {
        match value
        {
            winit::window::Theme::Light => Theme::Light,
            winit::window::Theme::Dark => Theme::Light,
        }
    }
}

pub trait WindowableSurface<Surface>
{
    fn surface(&self) -> Option<&Surface>;
    /// Don't forget to request_draw if needed
    fn replace_surface(&mut self, surface: Option<Surface>) -> Option<Surface>;
}

impl<Surface> Window<Surface>
{
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

    /*
    /// Lib specific method, expose impl details
    #[doc(hidden)]
    pub(crate) fn apply_change(&mut self)
    {
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
    }*/

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

impl<Surface> Windowable for Window<Surface>
{
    fn current_monitor(&self) -> Option<Monitor> { self.window.current_monitor().map(Into::into) }

    fn primary_monitor(&self) -> Option<Monitor> { self.window.primary_monitor().map(Into::into) }

    fn available_monitors(&self) -> impl Iterator<Item = Monitor>
    {
        self.window.available_monitors().map(Into::into)
    }

    fn request_draw(&mut self) -> &mut Self
    {
        self.window.request_redraw();
        self
    }

    fn request_user_attention(&mut self, request_type: impl Into<Option<UserAttentionType>>)
    {
        let request_type = request_type.into();
        self.window
            .request_user_attention(request_type.map(|v| v.into()));
    }

    /// Lib specific method, expose impl details
    #[doc(hidden)]
    fn winit_window(&self) -> WinitWindowShared { self.window.clone() }

    fn has_focus(&self) -> bool { self.window.has_focus() }

    fn focus(&mut self) -> &mut Self
    {
        self.window.focus_window();
        self
    }

    fn is_minimised(&self) -> Option<bool> { self.window.is_minimized() }

    fn set_minimised(&mut self, minimized: bool) -> &mut Self
    {
        self.window.set_minimized(minimized);
        self
    }

    fn set_cursor(&mut self, cursor: impl Into<Cursor>) -> &mut Self
    {
        let cursor: Cursor = cursor.into();
        self.window.set_cursor(cursor);
        self
    }

    fn set_cursor_pos(&mut self, pos: Point2) -> &mut Self
    {
        let p : WinitPhysicialPos = pos.convert();
        self.window.set_cursor_position(p);
        self
    }

    fn set_cursor_visible(&mut self, visible: bool) -> &mut Self
    {
        self.window.set_cursor_visible(visible);
        self
    }

    fn set_cursor_grab(&mut self, mode: CursorGrab) -> CursorResult
    {
        self.window
            .set_cursor_grab(mode.into())
            .map_err(|_| CursorError)
    }

    fn set_cursor_hittest(&mut self, hittest: bool) -> CursorResult
    {
        self.window
            .set_cursor_hittest(hittest)
            .map_err(|_| CursorError)
    }

    fn destroy_surface(&mut self) -> &mut Self
    {
        self.surface = None;
        self
    }
}
impl<Surface> WindowableSurface<Surface> for Window<Surface>
{
    fn surface(&self) -> Option<&Surface> { self.surface.as_ref() }

    fn replace_surface(&mut self, surface: Option<Surface>) -> Option<Surface>
    {
        let s = std::mem::replace(&mut self.surface, surface);
        s
    }
}

impl<Surface> GetPosition<int, 2> for Window<Surface>
{
    fn pos(&self) -> Vector<int, 2>
    {
        match &self.window.outer_position()
        {
            Ok(pos) =>
            {
                return pos.convert();
            }
            Err(e) => self.pos,
        }
    }
}
impl<Surface> SetPosition<int, 2> for Window<Surface>
{
    fn set_pos(&mut self, pos: Vector<int, 2>) -> &mut Self
    {
        self.pos = pos;
        let p : WinitPhysicialPos = pos.convert();
        self.window.set_outer_position(p);
        self
    }
}
impl<Surface> GetSize<int, 2> for Window<Surface>
{
    fn size(&self) -> Vector<int, 2> { self.window.inner_size().convert() }
}
impl<Surface> SetSize<int, 2> for Window<Surface>
{
    fn set_size(&mut self, size: Vector<int, 2>) -> &mut Self
    {
        self.size = size;
        let size : WinitPhysicialSize = size.convert();
        self.window.request_inner_size(size);
        self
    }
}

impl<Surface> WindowAttribute for Window<Surface>
{
    fn title(&self) -> String { self.param.title() }

    fn set_title(&mut self, title: String) -> &mut Self
    {
        self.window.set_title(&title);
        self.param.set_title(title);
        self
    }

    fn level(&self) -> WindowLevel { self.param.level() }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self
    {
        self.param.set_level(level);
        self.window.set_window_level(level.into());
        self
    }

    fn is_resizable(&self) -> bool { self.param.is_resizable() }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self
    {
        self.param.set_resizable(resizable);
        self.window.set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags { self.param.buttons() }

    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self
    {
        self.param.set_buttons(buttons);
        self.window.set_enabled_buttons(buttons.into());
        self
    }

    fn is_maximised(&self) -> bool { self.param.is_maximised() }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self
    {
        self.param.set_maximized(maximized);
        self.window.set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool { self.param.is_visible() }

    fn set_visible(&mut self, visible: bool) -> &mut Self
    {
        self.param.set_visible(visible);
        self.window.set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool { self.param.is_transparent() }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self
    {
        self.param.set_transparent(transparent);
        self.window.set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool { self.param.have_blur() }

    fn set_blur(&mut self, blur: bool) -> &mut Self
    {
        self.param.set_blur(blur);
        self.window.set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool { self.param.have_decoration() }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self
    {
        self.param.set_decoration(decorations);
        self.window.set_decorations(decorations);
        self
    }

    fn is_content_protected(&self) -> bool { self.param.is_content_protected() }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self
    {
        self.param.set_content_protected(protected);
        self.window.set_content_protected(protected);
        self
    }

    fn is_active(&self) -> bool { self.param.is_active() }

    fn set_active(&mut self, active: bool) -> &mut Self
    {
        self.param.set_active(active);
        self
    }

    fn icon(&self) -> Option<Image> { self.param.icon.clone() }

    fn set_icon(&mut self, icon: impl Into<Option<Image>>) -> &mut Self
    {
        let icon = icon.into();
        let winit_icon = match &icon
        {
            Some(icon) => image_to_winit_icon(icon),
            None => None,
        };

        self.window.set_window_icon(winit_icon);
        self.param.icon = icon;
        self
    }

    fn theme(&self) -> Option<Theme>
    {
        match self.window.theme()
        {
            Some(t) => Some(t.into()),
            None => self.param.theme,
        }
    }

    fn set_theme(&mut self, theme: Option<Theme>) -> &mut Self
    {
        self.window.set_theme(theme.map(Into::into));
        self.param.set_theme(theme);
        self
    }
}

pub(crate) fn image_to_winit_icon(image: &Image) -> Option<winit::window::Icon>
{
    let pixels_bytes: &[u8] = hexga_core::bit::transmute_slice(image.pixels());

    match winit::window::Icon::from_rgba(
        pixels_bytes.to_owned(),
        image.width() as _,
        image.height() as _,
    )
    {
        Ok(icon) => Some(icon),
        Err(_) => None,
    }
}

pub trait WindowAttribute: Sized
{
    fn title(&self) -> String;
    fn set_title(&mut self, title: String) -> &mut Self;
    fn with_title(mut self, title: String) -> Self
    {
        self.set_title(title);
        self
    }

    /// Returns the current window theme.
    ///
    /// Returns `None` if it cannot be determined on the current platform.
    fn theme(&self) -> Option<Theme>;

    /// Set or override the window theme.
    ///
    /// Specify `None` to reset the theme to the system default.
    fn set_theme(&mut self, theme: Option<Theme>) -> &mut Self;

    fn icon(&self) -> Option<Image>;
    /// Sets the window icon.
    ///
    /// On Windows and X11, this is typically the small icon in the top-left
    /// corner of the titlebar.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Web / Wayland / macOS / Orbital:** Unsupported.
    ///
    /// - **Windows:** Sets `ICON_SMALL`. The base size for a window icon is 16x16, but it's
    ///   recommended to account for screen scaling and pick a multiple of that, i.e. 32x32.
    ///
    /// - **X11:** Has no universal guidelines for icon sizes, so you're at the whims of the WM.
    ///   That said, it's usually in the same ballpark as on Windows.
    fn set_icon(&mut self, icon: impl Into<Option<Image>>) -> &mut Self;
    fn with_icon(mut self, icon: impl Into<Option<Image>>) -> Self
    {
        self.set_icon(icon);
        self
    }

    fn level(&self) -> WindowLevel;
    fn set_level(&mut self, level: WindowLevel) -> &mut Self;
    fn with_level(mut self, level: WindowLevel) -> Self
    {
        self.set_level(level);
        self
    }

    fn is_resizable(&self) -> bool;
    fn set_resizable(&mut self, resizable: bool) -> &mut Self;
    fn with_resizable(mut self, resizable: bool) -> Self
    {
        self.set_resizable(resizable);
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags;
    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self;
    fn with_buttons(mut self, buttons: WindowButtonFlags) -> Self
    {
        self.set_buttons(buttons);
        self
    }

    fn is_maximised(&self) -> bool;
    fn set_maximized(&mut self, maximized: bool) -> &mut Self;
    fn with_maximized(mut self, maximized: bool) -> Self
    {
        self.set_maximized(maximized);
        self
    }

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool) -> &mut Self;
    fn with_visible(mut self, visible: bool) -> Self
    {
        self.set_visible(visible);
        self
    }

    fn is_transparent(&self) -> bool;
    fn set_transparent(&mut self, transparent: bool) -> &mut Self;
    fn with_transparent(mut self, transparent: bool) -> Self
    {
        self.set_transparent(transparent);
        self
    }

    fn have_blur(&self) -> bool;
    fn set_blur(&mut self, blur: bool) -> &mut Self;
    fn with_blur(mut self, blur: bool) -> Self
    {
        self.set_blur(blur);
        self
    }

    fn have_decoration(&self) -> bool;
    fn set_decoration(&mut self, decorations: bool) -> &mut Self;
    fn with_decoration(mut self, decorations: bool) -> Self
    {
        self.set_decoration(true);
        self
    }

    fn is_content_protected(&self) -> bool;
    fn set_content_protected(&mut self, protected: bool) -> &mut Self;
    fn with_content_protected(mut self, protected: bool) -> Self
    {
        self.set_content_protected(protected);
        self
    }

    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool) -> &mut Self;
    fn with_active(mut self, active: bool) -> Self
    {
        self.set_active(active);
        self
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct WindowParam
{
    pub title: String,
    //pub size: Point2,
    //pub position: Point2,
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
    pub icon: Option<Image>,
    pub theme: Option<Theme>,
    //pub cursor_visible: bool,
}
impl Default for WindowParam
{
    fn default() -> Self
    {
        Self {
            title: "hexga app".to_owned(),
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
            icon: None,
            theme: ___(),
            //cursor_visible: true,
        }
    }
}


impl WindowAttribute for WindowParam
{
    fn title(&self) -> String { self.title.clone() }
    fn set_title(&mut self, title: String) -> &mut Self
    {
        self.title = title.into();
        self
    }

    fn level(&self) -> WindowLevel { self.level }

    fn set_level(&mut self, level: WindowLevel) -> &mut Self
    {
        self.level = level;
        self
    }

    fn is_resizable(&self) -> bool { self.resizable }

    fn set_resizable(&mut self, resizable: bool) -> &mut Self
    {
        self.resizable = resizable;
        self
    }

    fn buttons(&mut self) -> WindowButtonFlags { self.buttons }

    fn set_buttons(&mut self, buttons: WindowButtonFlags) -> &mut Self
    {
        self.buttons = buttons;
        self
    }

    fn is_maximised(&self) -> bool { self.maximized }

    fn set_maximized(&mut self, maximized: bool) -> &mut Self
    {
        self.maximized = maximized;
        self
    }

    fn is_visible(&self) -> bool { self.visible }

    fn set_visible(&mut self, visible: bool) -> &mut Self
    {
        self.visible = visible;
        self
    }

    fn is_transparent(&self) -> bool { self.transparent }

    fn set_transparent(&mut self, transparent: bool) -> &mut Self
    {
        self.transparent = transparent;
        self
    }

    fn have_blur(&self) -> bool { self.blur }

    fn set_blur(&mut self, blur: bool) -> &mut Self
    {
        self.blur = blur;
        self
    }

    fn have_decoration(&self) -> bool { self.decoration }

    fn set_decoration(&mut self, decorations: bool) -> &mut Self
    {
        self.decoration = decorations;
        self
    }

    fn is_content_protected(&self) -> bool { self.content_protected }

    fn set_content_protected(&mut self, protected: bool) -> &mut Self
    {
        self.content_protected = protected;
        self
    }

    fn is_active(&self) -> bool { self.active }

    fn set_active(&mut self, active: bool) -> &mut Self
    {
        self.active = active;
        self
    }

    fn theme(&self) -> Option<Theme> { self.theme }

    fn set_theme(&mut self, theme: Option<Theme>) -> &mut Self
    {
        self.theme = theme;
        self
    }

    fn icon(&self) -> Option<Image> { self.icon.clone() }

    fn set_icon(&mut self, icon: impl Into<Option<Image>>) -> &mut Self
    {
        self.icon = icon.into();
        self
    }
}

impl From<WindowParam> for WinitWindowAttributes
{
    fn from(value: WindowParam) -> Self
    {
        let WindowParam {
            title,
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
            icon,
            theme,
        } = value;

        let mut attr = winit::window::Window::default_attributes();

        attr.inner_size = None;
        attr.min_inner_size = None;
        attr.max_inner_size = None;
        attr.position = None;
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
        attr.preferred_theme = theme.map(Into::into);
        attr.window_icon = icon.map(|i| image_to_winit_icon(&i)).flatten();

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            win_attr = win_attr.with_append(true);
        }

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub enum UserAttentionType
{
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
    fn from(value: UserAttentionType) -> Self
    {
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
