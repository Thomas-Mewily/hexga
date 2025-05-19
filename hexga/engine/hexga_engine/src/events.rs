use crate::*;

pub use engine_core::events::
{
    LoopEvent,
    Event,
    WindowEvent, DropFileEvent,
    MouseEvent, MouseButtonEvent, MouseButton,
    KeyboardEvent, CharEvent, KeyMods, KeyEvent, KeyCode,
    TouchEvent, TouchPhase, TouchID
};

pub mod prelude
{
    use crate::*;
    pub use engine_core::events::prelude::*;
}