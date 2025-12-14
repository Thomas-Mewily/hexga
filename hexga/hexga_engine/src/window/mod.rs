pub use super::*;

mod app_window;
pub use app_window::*;

mod winit_convet;
pub use winit_convet::*;


#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent
{
    Resize(Point2),
    Move(Point2),
    Open,
    Close,
    Destroy,
    Draw,
}

pub mod prelude
{
    pub use super::{WindowEvent,WindowExtension,WindowParamBuilder};
    pub(crate) use super::*;
}