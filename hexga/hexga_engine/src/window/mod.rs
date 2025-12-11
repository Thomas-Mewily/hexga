pub use super::*;

mod app_window;
pub use app_window::*;

mod convert;
pub use convert::*;


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
    pub use super::{WindowEvent,WindowExtension,WindowBuilder};
    pub(crate) use super::*;
}