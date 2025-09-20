use super::*;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindowShared = Arc<WinitWindow>;
pub(crate) type WinitContext = Arc<WinitWindow>;