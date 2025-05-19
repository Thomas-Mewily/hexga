//! mainly based on [MiniQuad](https://github.com/not-fl3/miniquad)
use crate::*;

pub use miniquad::conf::{Platform,LinuxX11Gl,LinuxBackend,WebGLVersion,AppleGfxApi};

#[derive(Default)]
pub struct MultiMediaConfig 
{
    pub window_config : WindowConfig,
    pub pen_config    : PenConfig,
}

impl MultiMediaConfig
{
    pub fn new() -> Self { ___() }
    pub fn with_window_config(mut self, window : WindowConfig) -> Self { self.window_config = window; self }
    pub fn with_pen_config(mut self, pen : PenConfig) -> Self { self.pen_config = pen; self }
}

impl MultiMediaConfig
{
    pub fn run<T>(self, state : impl 'static + FnOnce() -> T) where T : EventLoop + 'static
    {
        let Self { window_config, pen_config } = self;
        miniquad::start(window_config.into(), move || 
        {
            Context::new(Box::new(state()), pen_config);
            Box::new(Ctx)
        });
    }
}


#[derive(Debug)]
pub struct WindowConfig
{
    /// Title of the window, defaults to an empty string.
    pub title: String,

    /// The preferred width / height of the window
    /// 
    /// Default: (960, 540)
    pub size : Point2,
    
    /// Whether the rendering canvas is full-resolution on HighDPI displays.
    ///
    /// Default: false
    pub high_dpi: bool,
    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fullscreen: bool,
    /// MSAA sample count
    ///
    /// Default: 1
    pub sample_count: i32,

    /// Determines if the application user can resize the window
    pub resizable: bool,

    /// Miniquad allows to change the window icon programmatically.
    /// The icon will be used as
    /// - taskbar and titlebar icons on Windows.
    /// - dock and titlebar icon on  MacOs.
    /// - TODO: favicon on HTML5
    /// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
    pub icon: Option<Icon>,

    /// Platform specific settings. Hints to OS for context creation, driver-specific
    /// settings etc.
    pub platform: Platform,
}
impl WindowConfig
{
    pub fn new() -> Self { ___() }

    pub fn title(mut self, title : impl Into<String>) -> Self { self.title = title.into(); self }

    /// Whether the rendering canvas is full-resolution on HighDPI displays.
    ///
    /// Default: false
    pub fn high_dpi(mut self, high_dpi : bool) -> Self { self.high_dpi = high_dpi; self }

    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fn fullscreen(mut self, fullscreen : bool) -> Self { self.fullscreen = fullscreen; self }

    /// MSAA sample count
    ///
    /// Default: 1
    pub fn sample_count(mut self, sample_count : i32) -> Self { self.sample_count = sample_count; self }

    /// Determines if the application user can resize the window
    pub fn resizeable(mut self, window_resizable : bool) -> Self { self.resizable = window_resizable; self }

    /// The icon will be used as
    /// - taskbar and titlebar icons on Windows.
    /// - dock and titlebar icon on  MacOs.
    /// - TODO: favicon on HTML5
    /// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
    pub fn icon(mut self, icon : Option<Icon>) -> Self { self.icon = icon; self }

    /// Platform specific settings. Hints to OS for context creation, driver-specific
    /// settings etc.
    pub fn platform(mut self, platform : Platform) -> Self { self.platform = platform; self }
}

impl From<WindowConfig> for miniquad::conf::Conf
{
    fn from(value: WindowConfig) -> Self {
        let WindowConfig{ title, size, high_dpi, fullscreen, sample_count, resizable, icon, platform } = value;
        miniquad::conf::Conf { window_title : title, window_width : size.x as _, window_height : size.y as _, high_dpi, fullscreen, sample_count, window_resizable : resizable, icon : icon.map(|v| v.into()), platform }
    }
}
impl Default for WindowConfig
{
    fn default() -> Self {
        Self { title: "hexga project".to_owned(), size : point2(960, 540), high_dpi: false, fullscreen: false, sample_count: 1, resizable: true, icon: None, platform: Platform::default() }
    }
}


/// Icon image in three levels of detail.
#[derive(Clone)]
pub struct Icon {
    /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
    pub small: [u8; 16 * 16 * 4],
    /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
    pub medium: [u8; 32 * 32 * 4],
    /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
    pub big: [u8; 64 * 64 * 4],
}
impl From<Icon> for miniquad::conf::Icon
{
    fn from(value: Icon) -> Self
    {
        let Icon { small, medium, big } = value;
        Self{ small, medium, big }
    }
}

impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon").finish()
    }
}