use super::*;

mod winit_convert;
pub use winit_convert::*;

mod app_window;
pub use app_window::*;

mod cursor;
pub use cursor::*;

mod event;
pub use event::*;

pub mod prelude
{
    pub use super::app_window::{Window, WindowParam, WindowResult};
    /*
    pub(crate) use super::{Window, WindowParam, WindowLevel, WinitWindowID, WinitWindowShared, ExternLibConvert};
    pub use super::{WindowAttribute, WindowEvent};
    */
    //pub use super::{WindowEvent};
    pub(crate) use super::winit_convert::ExternLibConvert;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::app_window::{WindowAttribute, WindowManager, Windowable, WindowableSurface};
}
