use super::*;

pub trait KeyboardShortcuts
{
    fn is_alt_f4(&self) -> bool;

    /// `Control + C` just pressed
    fn is_copy(&self) -> bool;
    /// `Control + V` just pressed
    fn is_paste(&self) -> bool;
}

impl KeyboardShortcuts for Keyboard
{
    fn is_alt_f4(&self) -> bool
    {
        (KeyCode::AltLeft.is_down() || KeyCode::AltRight.is_down()) && KeyCode::F4.is_pressed()
    }

    fn is_copy(&self) -> bool
    {
        ((KeyCode::ControlLeft.is_down() || KeyCode::ControlRight.is_down()) && KeyCode::C.is_pressed())
        || KeyCode::Copy.is_pressed()
    }

    fn is_paste(&self) -> bool
    {
        ((KeyCode::ControlLeft.is_down() || KeyCode::ControlRight.is_down()) && KeyCode::V.is_pressed())
        || KeyCode::Paste.is_pressed()
    }
}