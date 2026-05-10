use super::*;

mod keycode;
pub use keycode::*;

mod key;
pub use key::*;

mod keyboard;
pub use keyboard::*;

mod event;
pub use event::*;

pub mod prelude
{
    pub use super::{KeyEvent, KeyCode, KeyAction, KeyShortcut, KeyModifiers, KeyModifiersFlags};
    pub(crate) use super::{Keyboard, KeyCodeNative, KeyState, KeyRepeat, Key, KeyLocation};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{
        KeyboardShortcuts,
        KeyConstants,
        KeyModifiersExtension,
    };
}