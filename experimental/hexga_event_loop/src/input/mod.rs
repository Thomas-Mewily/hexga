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
    pub use super::{Clipboard, KeyAction, KeyCode, KeyEvent, KeyModifiers, KeyModifiersFlags, KeyShortcut, Key, KeyCodeNative, KeyLocation, KeyRepeat, KeyState, Keyboard, MouseWheel, MouseButton, MouseClick, TouchPhase, Touch};
}

pub mod traits
{
    pub use super::{Clipboardable, KeyConstants, KeyModifiersExtension, KeyboardShortcuts, TouchPhaseExtension};
}
