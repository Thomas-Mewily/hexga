use super::*;

mod keycode;
pub use keycode::*;

mod keyboard;
pub use keyboard::*;

mod event;
pub use event::*;

pub mod prelude
{
    pub use super::{KeyCode, InputEvent, KeyEvent};
    pub(crate) use super::Keyboard;
}
