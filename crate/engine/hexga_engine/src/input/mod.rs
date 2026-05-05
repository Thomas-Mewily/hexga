use super::*;

mod keycode;
pub use keycode::*;

mod event;
pub use event::*;

pub mod prelude
{
    pub use super::{KeyCode, InputEvent, KeyEvent};
}
