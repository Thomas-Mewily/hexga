use std::default;

use hexga_bitflags::bitindex;
use hexga_generational::prelude::{GenVec, GenVecID};
use hexga_graphics::image::Image;
use hexga_math::prelude::*;
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowID(pub(crate) winit::window::WindowId);
impl From<winit::window::WindowId> for WindowID
{
    fn from(value: winit::window::WindowId) -> Self {
        Self(value)
    }
}

mod cursor;
pub use cursor::*;

#[allow(dead_code)]
pub struct Window
{
    pub(crate) window : winit::window::Window,
    pub(crate) dpi    : float,
    pub(crate) childs : Vec<WindowID>,
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
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WindowParam
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
}

impl Into<winit::window::WindowAttributes> for WindowParam
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

impl Default for WindowParam
{
    fn default() -> Self {
        Self {
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
        }
    }
}
impl WindowParam
{
    pub fn new() -> Self {
        Self::default()
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
