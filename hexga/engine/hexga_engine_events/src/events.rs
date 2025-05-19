//! mainly inspired by miniquad
use std::fmt::Debug;
use hexga::prelude::*;

pub trait LoopEvent
{
    fn handle_event(&mut self, event : &Event) -> bool;
}
impl LoopEvent for () { fn handle_event(&mut self, _ : &Event) -> bool { true } }


#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub enum Event
{
    Window(WindowEvent),
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Touch(TouchEvent),
}
impl Debug for Event
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Window(v) => write!(f, "{:?}", v),
            Self::Mouse(v) => write!(f, "{:?}", v),
            Self::Keyboard(v) => write!(f, "{:?}", v),
            Self::Touch(v) => write!(f, "{:?}", v),
        }
    }
}

impl From<WindowEvent> for Event { fn from(value: WindowEvent) -> Self { Self::Window(value) } }
impl From<DropFileEvent> for Event { fn from(value: DropFileEvent) -> Self { Self::from(WindowEvent::DropFile(value)) } }
impl From<MouseEvent> for Event { fn from(value: MouseEvent) -> Self { Self::Mouse(value) } }
impl From<MouseButtonEvent> for Event { fn from(value: MouseButtonEvent) -> Self { Self::from(MouseEvent::Button(value)) } }
impl From<KeyboardEvent> for Event { fn from(value: KeyboardEvent) -> Self { Self::Keyboard(value) } }
impl From<CharEvent> for Event { fn from(value: CharEvent) -> Self { Self::from(KeyboardEvent::CharEvent(value)) } }
impl From<KeyEvent> for Event { fn from(value: KeyEvent) -> Self { Self::from(KeyboardEvent::KeyEvent(value)) } }
impl From<TouchEvent> for Event { fn from(value: TouchEvent) -> Self { Self::Touch(value) } }

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchEvent
{
    pub phase    : TouchPhase,
    pub id       : TouchID,
    pub position : Vec2,
}

pub type TouchID = u64;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase 
{
    Begin,
    Move,
    End,
    Cancel,
}

impl TouchPhase
{
    pub fn is_start (&self) -> bool { matches!(self, Self::Begin ) }
    pub fn is_move  (&self) -> bool { matches!(self, Self::Move  ) }
    pub fn is_end   (&self) -> bool { matches!(self, Self::End   ) }
    pub fn is_cancel(&self) -> bool { matches!(self, Self::Cancel) }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent
{
    Resize(Vec2),
    Minimized,
    Restored,
    Quit,
    DropFile(DropFileEvent),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropFileEvent
{
    pub path  : std::path::PathBuf,
    pub bytes : Vec<u8>,
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardEvent
{
    CharEvent(CharEvent),
    KeyEvent(KeyEvent),
}
impl Debug for KeyboardEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self 
        {
            Self::CharEvent(v) => write!(f, "{:?}", v),
            Self::KeyEvent(v) => write!(f, "{:?}", v),
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent
{
    pub keycode : KeyCode,
    pub keymods : KeyMods,
    pub repeat  : bool,
    pub press   : bool,
}
impl Debug for KeyEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyEvent").field("keycode", &self.keycode)
            .field_if_not_default("keymods", &self.keymods)
            .field_if_not_default("repeat", &self.repeat).field("press", &self.press).finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharEvent
{
    pub character: char,
    pub keymods  : KeyMods,
    pub repeat   : bool
}
impl Debug for CharEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CharEvent").field("char", &self.character).field_if_not_default("keymods", &self.keymods).field_if_not_default("repeat", &self.repeat).finish()
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEvent
{
    Wheel(Vec2),
    Move(Vec2),
    Button(MouseButtonEvent),

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    RawMove(Vec2),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseButtonEvent
{
    pub position : Vec2,
    pub button   : MouseButton,
    pub press    : bool,
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum MouseButton
{
    Left = 0,
    Middle = 1,
    Right = 2,
    Unknown = 255,
}
impl MouseButton
{
    pub fn is_left   (&self) -> bool { matches!(self, Self::Left   ) }
    pub fn is_right  (&self) -> bool { matches!(self, Self::Right  ) }
    pub fn is_middle (&self) -> bool { matches!(self, Self::Middle ) }
    pub fn is_unknown(&self) -> bool { matches!(self, Self::Unknown) }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Default)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl : bool,
    pub alt  : bool,
    pub logo : bool,
}


impl Debug for KeyMods
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        if self.is_default() { return Ok(()); }
        f.debug_struct("KeyMods").field_if_not_default("shift", &self.shift).field_if_not_default("ctrl", &self.ctrl).field_if_not_default("alt", &self.alt).field_if_not_default("logo", &self.logo).finish()
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u16)]
/// These keycode values are based off of X11's `keysymdef.h`.
/// Missing keycodes from that list are given the prefix 0x01.
pub enum KeyCode {
    Space = 0x0020,
    Apostrophe = 0x0027,
    Comma = 0x002c,
    Minus = 0x002d,
    Period = 0x002e,
    Slash = 0x002f,
    Key0 = 0x0030,
    Key1 = 0x0031,
    Key2 = 0x0032,
    Key3 = 0x0033,
    Key4 = 0x0034,
    Key5 = 0x0035,
    Key6 = 0x0036,
    Key7 = 0x0037,
    Key8 = 0x0038,
    Key9 = 0x0039,
    Semicolon = 0x003b,
    Equal = 0x003d,
    A = 0x0041,
    B = 0x0042,
    C = 0x0043,
    D = 0x0044,
    E = 0x0045,
    F = 0x0046,
    G = 0x0047,
    H = 0x0048,
    I = 0x0049,
    J = 0x004a,
    K = 0x004b,
    L = 0x004c,
    M = 0x004d,
    N = 0x004e,
    O = 0x004f,
    P = 0x0050,
    Q = 0x0051,
    R = 0x0052,
    S = 0x0053,
    T = 0x0054,
    U = 0x0055,
    V = 0x0056,
    W = 0x0057,
    X = 0x0058,
    Y = 0x0059,
    Z = 0x005a,
    LeftBracket = 0x005b,
    Backslash = 0x005c,
    RightBracket = 0x005d,
    GraveAccent = 0x0060,
    World1 = 0x0100,
    World2 = 0x0101,
    Escape = 0xff1b,
    Enter = 0xff0d,
    Tab = 0xff09,
    Backspace = 0xff08,
    Insert = 0xff63,
    Delete = 0xffff,
    Right = 0xff53,
    Left = 0xff51,
    Down = 0xff54,
    Up = 0xff52,
    PageUp = 0xff55,
    PageDown = 0xff56,
    Home = 0xff50,
    End = 0xff57,
    CapsLock = 0xffe5,
    ScrollLock = 0xff14,
    NumLock = 0xff7f,
    PrintScreen = 0xfd1d,
    Pause = 0xff13,
    F1 = 0xffbe,
    F2 = 0xffbf,
    F3 = 0xffc0,
    F4 = 0xffc1,
    F5 = 0xffc2,
    F6 = 0xffc3,
    F7 = 0xffc4,
    F8 = 0xffc5,
    F9 = 0xffc6,
    F10 = 0xffc7,
    F11 = 0xffc8,
    F12 = 0xffc9,
    F13 = 0xffca,
    F14 = 0xffcb,
    F15 = 0xffcc,
    F16 = 0xffcd,
    F17 = 0xffce,
    F18 = 0xffcf,
    F19 = 0xffd0,
    F20 = 0xffd1,
    F21 = 0xffd2,
    F22 = 0xffd3,
    F23 = 0xffd4,
    F24 = 0xffd5,
    F25 = 0xffd6,
    Kp0 = 0xffb0,
    Kp1 = 0xffb1,
    Kp2 = 0xffb2,
    Kp3 = 0xffb3,
    Kp4 = 0xffb4,
    Kp5 = 0xffb5,
    Kp6 = 0xffb6,
    Kp7 = 0xffb7,
    Kp8 = 0xffb8,
    Kp9 = 0xffb9,
    KpDecimal = 0xffae,
    KpDivide = 0xffaf,
    KpMultiply = 0xffaa,
    KpSubtract = 0xffad,
    KpAdd = 0xffab,
    KpEnter = 0xff8d,
    KpEqual = 0xffbd,
    LeftShift = 0xffe1,
    LeftControl = 0xffe3,
    LeftAlt = 0xffe9,
    LeftSuper = 0xffeb,
    RightShift = 0xffe2,
    RightControl = 0xffe4,
    RightAlt = 0xffea,
    RightSuper = 0xffec,
    Menu = 0xff67,
    // Android back button
    Back = 0xff04,
    Unknown = 0x01ff,
}