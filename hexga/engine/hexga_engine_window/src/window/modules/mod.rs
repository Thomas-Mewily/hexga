use hexga_bitflags::bitindex;
use hexga_generational::prelude::{GenVec, GenVecID};
use hexga_graphics::image::Image;
use hexga_math::prelude::*;
use crate::*;

pub type WindowID<W> = GenVecID<Window<W>>;
pub type WinitWindowID = winit::window::WindowId;


mod cursor;
pub use cursor::*;

mod context;
pub use context::*;


#[cfg(target_arch = "wasm32")]
type SharedWinitWindow = std::rc::Rc<winit::window::Window>;

#[cfg(not(target_arch = "wasm32"))]
type WinitWindowPtr = std::sync::Arc<winit::window::Window>;


#[allow(dead_code)]
pub struct Window<W>
{
    // window (and surface) are destroyed when pausing/resumed
    pub(crate) winit_window : Option<WinitWindowPtr>,
    pub(crate) param    : WindowParam<W>,
    pub(crate) id       : WindowID<W>,
    pub(crate) winit_id : WinitWindowID,
}

impl<W> Debug for Window<W>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id)
            .field("param", &self.param)
            .finish()
    }
}

impl<W> Window<W>
{
    pub fn id(&self) -> WindowID<W> { self.id }
    pub fn winit_id(&self) -> WinitWindowID { self.winit_id }

    pub fn param(&self) -> &WindowParam<W> { &self.param }

    pub fn data(&self) -> &W { &self.param.data }
    pub fn data_mut(&mut self) -> &mut W { &mut self.param.data }

    //pub fn size(&self) -> Point2 { self.physical_size() }

    pub fn physical_size(&self) -> Point2 { self.winit_window.as_ref().map(|w| w.inner_size().convert()).unwrap_or(one()) }
    pub fn logical_size(&self) -> Vec2 { self.physical_size().to_vec2() / self.param.dpi }

    pub fn winit_window(&self) -> Option<&WinitWindowPtr> { self.winit_window.as_ref() }

    pub fn childs(&self) -> &[WindowID<W>] { &self.param.childs }
    pub fn add_child(&mut self, child: WindowID<W>)
    {
        if !self.param.childs.contains(&child)
        {
            self.param.childs.push(child);
        }
    }
    pub fn remove_child(&mut self, child: WindowID<W>)
    {
        if let Some(pos) = self.param.childs.iter().position(|c| *c == child)
        {
            self.param.childs.remove(pos);
        }
    }

    pub(crate) fn resume(&mut self, active_event_loop : &ActiveEventLoop) -> Result<(), AppError>
    {
        let window = active_event_loop
            .create_window(self.param.clone_with_data(()).into())
            .map_err(|_| AppError::Unknow)?;

        let _ = window.set_cursor_grab(self.param.cursor_grab.into());
        window.set_cursor_visible(self.param.cursor_visible);

        // Where to handle dpi?

        self.winit_id = window.id();
        self.winit_window = Some(WinitWindowPtr::new(window));
        Ok(())
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
pub struct WindowParam<W>
{
    /// Title of the window, defaults to an empty string.
    pub title: String,

    pub size : Option<Vec2>,
    pub position : Option<Vec2>,
    pub resizable : bool,

    pub visible: bool,
    pub transparent: bool,

    pub buttons : WindowButtonFlags,
    pub level : WindowLevel,

    pub icon: Option<Icon>,
    pub active: bool,

    pub cursor_icon         : CursorIcon,
    pub cursor_grab    : CursorGrab,
    pub cursor_visible : bool,

    pub dpi : float,

    pub close_when_parent_exit : bool,

    pub data : W,

    pub childs : Vec<WindowID<W>>,
}


impl<W> Debug for WindowParam<W>
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

impl<W> Into<winit::window::WindowAttributes> for WindowParam<W>
{
    fn into(self) -> winit::window::WindowAttributes {
        let mut att = winit::window::Window::default_attributes();

        att.title = self.title;
        att.inner_size = self.size.map(|s| winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(s.x as _, s.y as _)));
        att.position = self.position.map(|s| winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(s.x as _, s.y as _)));
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
        att
    }
}

impl<W> Default for WindowParam<W> where W: Default
{
    fn default() -> Self { Self::new_with_data(___())}
}
impl<W> WindowParam<W>
{
    pub fn new() -> Self where W: Default { ___() }

    pub fn new_with_data(data : W) -> Self
    {
        Self
        {
            title: ___(),
            size: ___(),
            position: ___(),
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
            data,
            childs: ___(),
        }
    }

    pub fn clone_with_data<T2>(&self, data: T2) -> WindowParam<T2>
    {
        WindowParam
        {
            title: self.title.clone(),
            size: self.size,
            position: self.position,
            resizable: self.resizable,
            visible: self.visible,
            transparent: self.transparent,
            buttons: self.buttons,
            level: self.level,
            icon: self.icon.clone(),
            active: self.active,
            cursor_icon: self.cursor_icon,
            cursor_grab: self.cursor_grab,
            dpi: self.dpi,
            cursor_visible: self.cursor_visible,
            close_when_parent_exit: self.close_when_parent_exit,
            data,
            childs: self.childs.iter().map(|id| WindowID::from_other_id(*id)).collect(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    pub fn with_size(mut self, size: Option<Vec2>) -> Self {
        self.size = size;
        self
    }

    pub fn with_position(mut self, position: Option<Vec2>) -> Self {
        self.position = position;
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
    pub fn with_buttons(mut self, buttons: WindowButtonFlags) -> Self {
        self.buttons = buttons;
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
    pub fn with_data(mut self, data: W) -> Self {
        self.data = data;
        self
    }
    pub fn with_childs(mut self, childs: Vec<WindowID<W>>) -> Self {
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

/*

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum WindowMode
{
    Free,
    /// Inside another window
    #[default]
    Nested,
}

impl Default for WindowParam
{
    fn default() -> Self
    {
        Self
        {
            title: String::new(),
            size : point2(960, 540),
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            resizable: true,
            icon: None,
            can_be_free : true,
        }
    }
}




impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon").finish()
    }
}
*/
