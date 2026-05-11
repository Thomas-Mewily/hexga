use super::*;
use hexga_event_loop::window::{UserAttentionType, WindowButtonFlags, WindowLevel};

pub static WINDOW : Singleton<WindowType> = Singleton::uninit();
pub type WindowType = hexga_event_loop::window::Window<GpuSurface<'static>>;
