use super::*;

mod keycode;
pub use keycode::*;

mod key;
pub use key::*;

mod keyboard;
pub use keyboard::*;

mod event;
pub use event::*;

mod clipboard;
pub use clipboard::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        Clipboard, KeyAction, KeyCode, KeyEvent, KeyModifiers, KeyModifiersFlags, KeyShortcut,
    };
    pub(crate) use super::{Key, KeyCodeNative, KeyLocation, KeyRepeat, KeyState, Keyboard};
}

pub mod traits
{
    pub use super::{Clipboardable, KeyConstants, KeyModifiersExtension, KeyboardShortcuts};
}
