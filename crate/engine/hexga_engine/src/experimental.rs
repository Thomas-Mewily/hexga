use super::*;
use hexga_event_loop::window::{UserAttentionType, WindowButtonFlags, WindowLevel};

pub static WINDOW: Singleton<WindowType> = Singleton::uninit();
pub static GRAPHICS : Singleton<Graphics> = Singleton::uninit();
