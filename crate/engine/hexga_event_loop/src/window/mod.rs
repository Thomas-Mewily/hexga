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
    pub use super::{WindowParam, Window, WindowResult};
    /*
    pub(crate) use super::{Window, WindowParam, WindowLevel, WinitWindowID, WinitWindowShared, ExternLibConvert};
    pub use super::{WindowAttribute, WindowEvent};
    */
    //pub use super::{WindowEvent};
    pub(crate) use super::{ExternLibConvert};
    pub use super::traits::*;
}


pub mod traits
{
    pub use super::{WindowManager, WindowAttribute, Windowable, WindowableSurface};
}