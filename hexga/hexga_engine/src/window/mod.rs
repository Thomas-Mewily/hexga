pub use super::*;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitWindowShared = Arc<WinitWindow>;


mod app_window;
pub use app_window::*;

mod convert;
pub use convert::*;


#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent
{
    Resize(Vec2i),
    Move(Vec2i),
    Open,
    Close,
    Destroy,
    Draw,
}