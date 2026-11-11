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

mod mouse;
pub use mouse::*;

mod touch;
pub use touch::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::clipboard::Clipboard;
    pub use super::event::{KeyAction, KeyEvent, KeyRepeat, KeyShortcut, KeyState};
    pub use super::key::{Key, KeyModifiers, KeyModifiersFlags};
    pub use super::keycode::{KeyCode, KeyCodeNative, KeyLocation};
    pub use super::keyboard::Keyboard;
    pub use super::mouse::{MouseButton, MouseClick, MouseWheel, TouchPhase};
    pub use super::touch::Touch;
}

pub mod traits
{
    pub use super::clipboard::Clipboardable;
    pub use super::event::{KeyConstants, KeyboardShortcuts};
    pub use super::key::KeyModifiersExtension;
    pub use super::mouse::TouchPhaseExtension;
}
