//! implementation of the Hexga Engine using MiniQuad
#![allow(dead_code)]
pub use hexga_engine::*;

use events::prelude::*;





pub(crate) trait ToHexgaEngine
{
    type Output;
    fn to_hexga(self) -> Self::Output;
}

impl ToHexgaEngine for miniquad::TouchPhase
{
    type Output = TouchPhase;

    fn to_hexga(self) -> Self::Output {
        match self
        {
            miniquad::TouchPhase::Started => TouchPhase::Begin,
            miniquad::TouchPhase::Moved => TouchPhase::Move,
            miniquad::TouchPhase::Ended => TouchPhase::End,
            miniquad::TouchPhase::Cancelled => TouchPhase::Cancel,
        }
    }
}

impl ToHexgaEngine for miniquad::MouseButton
{
    type Output = MouseButton;

    fn to_hexga(self) -> Self::Output {
        match self
        {
            miniquad::MouseButton::Left => MouseButton::Left,
            miniquad::MouseButton::Middle => MouseButton::Middle,
            miniquad::MouseButton::Right => MouseButton::Right,
            miniquad::MouseButton::Unknown => MouseButton::Unknown,
        }
    }
}

impl ToHexgaEngine for miniquad::KeyMods
{
    type Output = KeyMods;

    fn to_hexga(self) -> Self::Output {
        let miniquad::KeyMods{ shift, ctrl, alt, logo } = self;
        KeyMods { shift, ctrl, alt, logo }
    }
}

impl ToHexgaEngine for miniquad::KeyCode
{
    type Output = KeyCode;

    fn to_hexga(self) -> Self::Output {
        match self
        {
            miniquad::KeyCode::Space => KeyCode::Space,
            miniquad::KeyCode::Apostrophe => KeyCode::Apostrophe,
            miniquad::KeyCode::Comma => KeyCode::Comma,
            miniquad::KeyCode::Minus => KeyCode::Minus,
            miniquad::KeyCode::Period => KeyCode::Period,
            miniquad::KeyCode::Slash => KeyCode::Slash,
            miniquad::KeyCode::Key0 => KeyCode::Key0,
            miniquad::KeyCode::Key1 => KeyCode::Key1,
            miniquad::KeyCode::Key2 => KeyCode::Key2,
            miniquad::KeyCode::Key3 => KeyCode::Key3,
            miniquad::KeyCode::Key4 => KeyCode::Key4,
            miniquad::KeyCode::Key5 => KeyCode::Key5,
            miniquad::KeyCode::Key6 => KeyCode::Key6,
            miniquad::KeyCode::Key7 => KeyCode::Key7,
            miniquad::KeyCode::Key8 => KeyCode::Key8,
            miniquad::KeyCode::Key9 => KeyCode::Key9,
            miniquad::KeyCode::Semicolon => KeyCode::Semicolon,
            miniquad::KeyCode::Equal => KeyCode::Equal,
            miniquad::KeyCode::A => KeyCode::A,
            miniquad::KeyCode::B => KeyCode::B,
            miniquad::KeyCode::C => KeyCode::C,
            miniquad::KeyCode::D => KeyCode::D,
            miniquad::KeyCode::E => KeyCode::E,
            miniquad::KeyCode::F => KeyCode::F,
            miniquad::KeyCode::G => KeyCode::G,
            miniquad::KeyCode::H => KeyCode::H,
            miniquad::KeyCode::I => KeyCode::I,
            miniquad::KeyCode::J => KeyCode::J,
            miniquad::KeyCode::K => KeyCode::K,
            miniquad::KeyCode::L => KeyCode::L,
            miniquad::KeyCode::M => KeyCode::M,
            miniquad::KeyCode::N => KeyCode::N,
            miniquad::KeyCode::O => KeyCode::O,
            miniquad::KeyCode::P => KeyCode::P,
            miniquad::KeyCode::Q => KeyCode::Q,
            miniquad::KeyCode::R => KeyCode::R,
            miniquad::KeyCode::S => KeyCode::S,
            miniquad::KeyCode::T => KeyCode::T,
            miniquad::KeyCode::U => KeyCode::U,
            miniquad::KeyCode::V => KeyCode::V,
            miniquad::KeyCode::W => KeyCode::W,
            miniquad::KeyCode::X => KeyCode::X,
            miniquad::KeyCode::Y => KeyCode::Y,
            miniquad::KeyCode::Z => KeyCode::Z,
            miniquad::KeyCode::LeftBracket => KeyCode::LeftBracket,
            miniquad::KeyCode::Backslash => KeyCode::Backslash,
            miniquad::KeyCode::RightBracket => KeyCode::RightBracket,
            miniquad::KeyCode::GraveAccent => KeyCode::GraveAccent,
            miniquad::KeyCode::World1 => KeyCode::World1,
            miniquad::KeyCode::World2 => KeyCode::World2,
            miniquad::KeyCode::Escape => KeyCode::Escape,
            miniquad::KeyCode::Enter => KeyCode::Enter,
            miniquad::KeyCode::Tab => KeyCode::Tab,
            miniquad::KeyCode::Backspace => KeyCode::Backspace,
            miniquad::KeyCode::Insert => KeyCode::Insert,
            miniquad::KeyCode::Delete => KeyCode::Delete,
            miniquad::KeyCode::Right => KeyCode::Right,
            miniquad::KeyCode::Left => KeyCode::Left,
            miniquad::KeyCode::Down => KeyCode::Down,
            miniquad::KeyCode::Up => KeyCode::Up,
            miniquad::KeyCode::PageUp => KeyCode::PageUp,
            miniquad::KeyCode::PageDown => KeyCode::PageDown,
            miniquad::KeyCode::Home => KeyCode::Home,
            miniquad::KeyCode::End => KeyCode::End,
            miniquad::KeyCode::CapsLock => KeyCode::CapsLock,
            miniquad::KeyCode::ScrollLock => KeyCode::ScrollLock,
            miniquad::KeyCode::NumLock => KeyCode::NumLock,
            miniquad::KeyCode::PrintScreen => KeyCode::PrintScreen,
            miniquad::KeyCode::Pause => KeyCode::Pause,
            miniquad::KeyCode::F1 => KeyCode::F1,
            miniquad::KeyCode::F2 => KeyCode::F2,
            miniquad::KeyCode::F3 => KeyCode::F3,
            miniquad::KeyCode::F4 => KeyCode::F4,
            miniquad::KeyCode::F5 => KeyCode::F5,
            miniquad::KeyCode::F6 => KeyCode::F6,
            miniquad::KeyCode::F7 => KeyCode::F7,
            miniquad::KeyCode::F8 => KeyCode::F8,
            miniquad::KeyCode::F9 => KeyCode::F9,
            miniquad::KeyCode::F10 => KeyCode::F10,
            miniquad::KeyCode::F11 => KeyCode::F11,
            miniquad::KeyCode::F12 => KeyCode::F12,
            miniquad::KeyCode::F13 => KeyCode::F13,
            miniquad::KeyCode::F14 => KeyCode::F14,
            miniquad::KeyCode::F15 => KeyCode::F15,
            miniquad::KeyCode::F16 => KeyCode::F16,
            miniquad::KeyCode::F17 => KeyCode::F17,
            miniquad::KeyCode::F18 => KeyCode::F18,
            miniquad::KeyCode::F19 => KeyCode::F19,
            miniquad::KeyCode::F20 => KeyCode::F20,
            miniquad::KeyCode::F21 => KeyCode::F21,
            miniquad::KeyCode::F22 => KeyCode::F22,
            miniquad::KeyCode::F23 => KeyCode::F23,
            miniquad::KeyCode::F24 => KeyCode::F24,
            miniquad::KeyCode::F25 => KeyCode::F25,
            miniquad::KeyCode::Kp0 => KeyCode::Kp0,
            miniquad::KeyCode::Kp1 => KeyCode::Kp1,
            miniquad::KeyCode::Kp2 => KeyCode::Kp2,
            miniquad::KeyCode::Kp3 => KeyCode::Kp3,
            miniquad::KeyCode::Kp4 => KeyCode::Kp4,
            miniquad::KeyCode::Kp5 => KeyCode::Kp5,
            miniquad::KeyCode::Kp6 => KeyCode::Kp6,
            miniquad::KeyCode::Kp7 => KeyCode::Kp7,
            miniquad::KeyCode::Kp8 => KeyCode::Kp8,
            miniquad::KeyCode::Kp9 => KeyCode::Kp9,
            miniquad::KeyCode::KpDecimal => KeyCode::KpDecimal,
            miniquad::KeyCode::KpDivide => KeyCode::KpDivide,
            miniquad::KeyCode::KpMultiply => KeyCode::KpMultiply,
            miniquad::KeyCode::KpSubtract => KeyCode::KpSubtract,
            miniquad::KeyCode::KpAdd => KeyCode::KpAdd,
            miniquad::KeyCode::KpEnter => KeyCode::KpEnter,
            miniquad::KeyCode::KpEqual => KeyCode::KpEqual,
            miniquad::KeyCode::LeftShift => KeyCode::LeftShift,
            miniquad::KeyCode::LeftControl => KeyCode::LeftControl,
            miniquad::KeyCode::LeftAlt => KeyCode::LeftAlt,
            miniquad::KeyCode::LeftSuper => KeyCode::LeftSuper,
            miniquad::KeyCode::RightShift => KeyCode::RightShift,
            miniquad::KeyCode::RightControl => KeyCode::RightControl,
            miniquad::KeyCode::RightAlt => KeyCode::RightAlt,
            miniquad::KeyCode::RightSuper => KeyCode::RightSuper,
            miniquad::KeyCode::Menu => KeyCode::Menu,
            miniquad::KeyCode::Back => KeyCode::Back,
            miniquad::KeyCode::Unknown => KeyCode::Unknown,
        }
    }
}