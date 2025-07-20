use super::*;

pub type WindowID = GenVecID<Window>;
pub(crate) type WinitWindowID = winit::window::WindowId;


#[cfg(target_arch = "wasm32")]
pub type WinitWindowPtrKind<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type WinitWindowPtrKind<T> = std::sync::Arc<T>;

pub(crate) type WinitWindow = winit::window::Window;

pub struct WinitWindowPtr
{
    window : WinitWindowPtrKind<WinitWindow>
}
impl WinitWindowPtr
{
    pub fn new(window : WinitWindow) -> Self { Self { window : WinitWindowPtrKind::new(window) }}
    pub fn winit_window(&self) -> &WinitWindow { &self.window }
}
impl Deref for WinitWindowPtr
{
    type Target=WinitWindow;
    fn deref(&self) -> &Self::Target { &self.window }
}

#[allow(dead_code)]
pub struct Window
{
    // window (and surface) are destroyed when pausing/resumed
    pub(crate) winit_window : Option<WinitWindowPtr>,
    pub(crate) param    : WindowParam,
    pub(crate) id       : WindowID,
    pub(crate) winit_id : WinitWindowID,
}



impl Debug for Window
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id)
            .field("param", &self.param)
            .finish()
    }
}

impl Window
{
    pub fn id(&self) -> WindowID { self.id }
    pub(crate) fn winit_id(&self) -> WinitWindowID { self.winit_id }

    pub fn param(&self) -> &WindowParam { &self.param }

    pub fn physical_size(&self) -> Point2 { self.winit_window.as_ref().map(|w| w.inner_size().convert()).unwrap_or(one()) }
    pub fn logical_size(&self) -> Vec2 { self.physical_size().to_vec2() / self.param.dpi }

    pub(crate) fn winit_window(&self) -> Option<&WinitWindowPtr> { self.winit_window.as_ref() }

    pub fn childs(&self) -> &[WindowID] { &self.param.childs }
    pub fn add_child(&mut self, child: WindowID)
    {
        if !self.param.childs.contains(&child)
        {
            self.param.childs.push(child);
        }
    }
    pub fn remove_child(&mut self, child: WindowID)
    {
        if let Some(pos) = self.param.childs.iter().position(|c| *c == child)
        {
            self.param.childs.remove(pos);
        }
    }
}

#[bitindex]
#[repr(u8)]
pub enum WindowButton
{
    Close,
    Minimize,
    Maximize,
}

impl Into<winit::window::WindowButtons> for WindowButtonFlags
{
    fn into(self) -> winit::window::WindowButtons
    {
        let mut buttons = winit::window::WindowButtons::empty();
        if self.contains(WindowButton::Close) {
            buttons |= winit::window::WindowButtons::CLOSE;
        }
        if self.contains(WindowButton::Minimize) {
            buttons |= winit::window::WindowButtons::MINIMIZE;
        }
        if self.contains(WindowButton::Maximize) {
            buttons |= winit::window::WindowButtons::MAXIMIZE;
        }
        buttons
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Into<winit::window::WindowLevel> for WindowLevel
{
    fn into(self) -> winit::window::WindowLevel {
        match self {
            WindowLevel::AlwaysOnBottom => winit::window::WindowLevel::AlwaysOnBottom,
            WindowLevel::Normal => winit::window::WindowLevel::Normal,
            WindowLevel::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
        }
    }
}


//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(PartialEq, PartialOrd, Clone)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    pub title: String,

    pub size : Point2,
    pub position : Point2,

    pub(crate) default_size_and_position : bool,

    pub resizable : bool,

    pub visible: bool,
    pub transparent: bool,

    pub buttons : WindowButtonFlags,
    pub level : WindowLevel,

    pub icon: Option<Icon>,
    pub active: bool,

    pub cursor_icon    : CursorIcon,
    pub cursor_grab    : CursorGrab,
    pub cursor_visible : bool,

    pub dpi : float,

    pub close_when_parent_exit : bool,

    pub childs : Vec<WindowID>,
}


impl Debug for WindowParam
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowParam")
            .field("title", &self.title)
            .field("size", &self.size)
            .field("position", &self.position)
            .field("resizable", &self.resizable)
            .field("visible", &self.visible)
            .field("transparent", &self.transparent)
            .field("buttons", &self.buttons)
            .field("level", &self.level)
            .field("icon", &self.icon)
            .field("active", &self.active)
            .field("cursor_icon", &self.cursor_icon)
            .field("cursor_grab", &self.cursor_grab)
            .field("cursor_visible", &self.cursor_visible)
            .field("dpi", &self.dpi)
            .field("close_when_parent_exit", &self.close_when_parent_exit)
            .field("childs", &self.childs)
            .finish()
    }
}

impl Into<winit::window::WindowAttributes> for WindowParam
{
    fn into(self) -> winit::window::WindowAttributes {
        let mut att = winit::window::Window::default_attributes();

        att.title = self.title;
        att.inner_size = self.default_size_and_position.then(|| winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(self.size.x as _, self.size.y as _)));
        att.position = self.default_size_and_position.then(|| winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(self.position.x as _, self.position.y as _)));
        att.resizable = self.resizable;
        att.visible = self.visible;
        att.transparent = self.transparent;
        att.decorations = self.buttons != WindowButtonFlags::ZERO;
        att.window_level = self.level.into();
        att.window_icon = self.icon.map(|icon|
        {
            let (size, rgba) = icon.image.into_size_and_values();
            let rgba2 = rgba.into_iter().map(|v| v.to_array4()).flatten().collect();
            winit::window::Icon::from_rgba(rgba2, size.x as _, size.y as _).unwrap()
        });
        att.active = self.active;
        att.cursor = winit::window::Cursor::Icon(self.cursor_icon.into());
        att.enabled_buttons = self.buttons.into();
        att
    }
}

impl Default for WindowParam
{
    fn default() -> Self { Self::new() }
}
impl WindowParam
{
    pub fn new() -> Self
    {
        Self
        {
            title: ___(),
            size: ___(),
            position: ___(),
            default_size_and_position: true,
            resizable: true,
            visible: true,
            transparent: false,
            buttons: WindowButton::Close | WindowButton::Minimize | WindowButton::Maximize,
            level: ___(),
            icon: ___(),
            active: true,
            cursor_icon: ___(),
            cursor_grab: ___(),
            dpi: 1.,
            cursor_visible: true,
            close_when_parent_exit: true,
            childs: ___(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn with_size(mut self, size: Point2) -> Self {
        self.size = size;
        self
    }
    pub fn with_position(mut self, position: Point2) -> Self {
        self.position = position;
        self
    }
    pub fn with_default_size_and_position(mut self, default_size_and_position : bool) -> Self
    {
        self.default_size_and_position = default_size_and_position;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    pub fn with_transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }
    pub fn with_buttons(mut self, buttons: impl Into<WindowButtonFlags>) -> Self {
        self.buttons = buttons.into();
        self
    }
    pub fn with_level(mut self, level: WindowLevel) -> Self {
        self.level = level;
        self
    }
    pub fn with_icon(mut self, icon: Option<Icon>) -> Self {
        self.icon = icon;
        self
    }
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
    pub fn with_cursor_icon(mut self, cursor: CursorIcon) -> Self {
        self.cursor_icon = cursor;
        self
    }
    pub fn with_cursor_grab(mut self, cursor_grab: CursorGrab) -> Self {
        self.cursor_grab = cursor_grab;
        self
    }
    pub fn with_cursor_visible(mut self, cursor_visible: bool) -> Self {
        self.cursor_visible = cursor_visible;
        self
    }
    pub fn with_dpi(mut self, dpi: float) -> Self {
        self.dpi = dpi;
        self
    }
    pub fn with_close_when_parent_exit(mut self, close_when_parent_exit: bool) -> Self {
        self.close_when_parent_exit = close_when_parent_exit;
        self
    }
    pub fn with_childs(mut self, childs: Vec<WindowID>) -> Self {
        self.childs = childs;
        self
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
/// Icon image in three levels of detail.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Icon {
    /*
    /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_16x16: [u8; 16 * 16 * 4],
    /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_32x32: [u8; 32 * 32 * 4],
    /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_64x64: [u8; 64 * 64 * 4],
    */

    image : Image,
}

impl Icon
{
    pub fn from_image(image : Image) -> Self { Self { image }}
}
