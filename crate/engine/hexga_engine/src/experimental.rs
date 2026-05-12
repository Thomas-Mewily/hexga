use super::*;
use hexga_event_loop::window::{UserAttentionType, WindowButtonFlags, WindowLevel};

pub(crate) static WINDOW: Singleton<WindowType> = Singleton::uninit();
//pub(crate) static GRAPHICS : Singleton<Graphics> = Singleton::uninit();
