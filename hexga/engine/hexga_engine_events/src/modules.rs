use crate::*;
use std::fmt::Debug;
use hexga_core::prelude::*;

use hexga_math::prelude::Vec2;

pub trait LoopEvent
{
    fn handle_event(&mut self, event : &Event) -> bool;
}
impl LoopEvent for () { fn handle_event(&mut self, _ : &Event) -> bool { true } }


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq)]
pub enum Event
{
    Window  (WindowEvent  ),
    Mouse   (MouseEvent   ),
    Keyboard(KeyboardEvent),
    Touch   (TouchEvent   ),
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
//impl From<DropFileEvent> for Event { fn from(value: DropFileEvent) -> Self { Self::from(WindowEvent::DropFile(value)) } }
impl From<MouseMove> for Event { fn from(value: MouseMove) -> Self { Self::Mouse(MouseEvent::Move(value)) } }
impl From<MouseEvent> for Event { fn from(value: MouseEvent) -> Self { Self::Mouse(value) } }
impl From<MouseButtonEvent> for Event { fn from(value: MouseButtonEvent) -> Self { Self::from(MouseEvent::Button(value)) } }
impl From<KeyboardEvent> for Event { fn from(value: KeyboardEvent) -> Self { Self::Keyboard(value) } }
impl From<CharEvent> for Event { fn from(value: CharEvent) -> Self { Self::from(KeyboardEvent::CharEvent(value)) } }
impl From<KeyEvent> for Event { fn from(value: KeyEvent) -> Self { Self::from(KeyboardEvent::KeyEvent(value)) } }
impl From<TouchEvent> for Event { fn from(value: TouchEvent) -> Self { Self::Touch(value) } }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchEvent
{
    pub phase    : TouchPhase,
    pub id       : TouchID,
    pub position : Vec2,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TouchID { pub index : u64 }
impl TouchID { pub const fn new(index : u64) -> Self { Self { index }}}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent
{
    Resize(Vec2),
    Minimized,
    Restored,
    Quit,
    DropFile, //(DropFileEvent),
}

/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropFileEvent
{
    pub path : std::path::PathBuf,
    pub data : Vec<u8>,
}
    */

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEvent
{
    pub character: char,

    pub keycode  : KeyCode,
    pub keymods  : KeyMods,

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEvent
{
    Wheel(Vec2),
    Move(MouseMove),
    Button(MouseButtonEvent),

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    RawMove(Vec2),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMove
{
    pub position : Vec2,
    // delta : Vec2
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseButtonEvent
{
    pub position : Vec2,
    pub button   : MouseButton,
    pub press    : bool,
}

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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

/// Contains the platform-native physical key identifier
///
/// The exact values vary from platform to platform (which is part of why this is a per-platform
/// enum), but the values are primarily tied to the key's physical location on the keyboard.
///
/// This enum is primarily used to store raw keycodes when Winit doesn't map a given native
/// physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
/// haven't mapped for you yet, this lets you use use [`KeyCode`] to:
///
/// - Correctly match key press and release events.
/// - On non-web platforms, support assigning keybinds to virtually any key through a UI.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NativeKeyCode
{
    /// Unidentified
    Unidentified,
    /// An Android "scancode".
    Android(u32),
    /// A macOS "scancode".
    MacOS(u16),
    /// A Windows "scancode".
    Windows(u16),
    /// An XKB "keycode".
    Xkb(u32),
}

/// Based on [Winit KeyCode](https://docs.rs/winit/0.30.11/winit/keyboard/enum.KeyCode.html)
/// https://docs.rs/winit-gtk/latest/winit/event/enum.VirtualKeyCode.html
/// https://docs.rs/miniquad/latest/miniquad/enum.KeyCode.html
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum KeyCode
{
              /// <kbd>0</kbd> on a US keyboard.
    Key0 = 0, /// <kbd>1</kbd> on a US keyboard.
    Key1 = 1, /// <kbd>2</kbd> on a US keyboard.
    Key2 = 2, /// <kbd>3</kbd> on a US keyboard.
    Key3 = 3, /// <kbd>4</kbd> on a US keyboard.
    Key4 = 4, /// <kbd>5</kbd> on a US keyboard.
    Key5 = 5, /// <kbd>6</kbd> on a US keyboard.
    Key6 = 6, /// <kbd>7</kbd> on a US keyboard.
    Key7 = 7, /// <kbd>8</kbd> on a US keyboard.
    Key8 = 8, /// <kbd>9</kbd> on a US keyboard.
    Key9 = 9,

            /// <kbd>a</kbd> on a US keyboard. Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    A = 10, /// <kbd>b</kbd> on a US keyboard.
    B = 11, /// <kbd>c</kbd> on a US keyboard.
    C = 12, /// <kbd>d</kbd> on a US keyboard.
    D = 13, /// <kbd>e</kbd> on a US keyboard.
    E = 14, /// <kbd>f</kbd> on a US keyboard.
    F = 15, /// <kbd>g</kbd> on a US keyboard.
    G = 16, /// <kbd>h</kbd> on a US keyboard.
    H = 17, /// <kbd>i</kbd> on a US keyboard.
    I = 18, /// <kbd>j</kbd> on a US keyboard.
    J = 19, /// <kbd>k</kbd> on a US keyboard.
    K = 20, /// <kbd>l</kbd> on a US keyboard.
    L = 21, /// <kbd>m</kbd> on a US keyboard.
    M = 22, /// <kbd>n</kbd> on a US keyboard.
    N = 23, /// <kbd>o</kbd> on a US keyboard.
    O = 24, /// <kbd>p</kbd> on a US keyboard.
    P = 25, /// <kbd>q</kbd> on a US keyboard. Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    Q = 26, /// <kbd>r</kbd> on a US keyboard.
    R = 27, /// <kbd>s</kbd> on a US keyboard.
    S = 28, /// <kbd>t</kbd> on a US keyboard.
    T = 29, /// <kbd>u</kbd> on a US keyboard.
    U = 30, /// <kbd>v</kbd> on a US keyboard.
    V = 31, /// <kbd>w</kbd> on a US keyboard. Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    W = 32, /// <kbd>x</kbd> on a US keyboard.
    X = 34, /// <kbd>y</kbd> on a US keyboard. Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    Y = 35, /// <kbd>z</kbd> on a US keyboard. Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a QWERTZ (e.g., German) keyboard.
    Z = 36, /// <kbd> </kbd> (space)
    Space,


    /// <kbd>←</kbd>
    Right,
    /// <kbd>↑</kbd>
    Left,
    /// <kbd>→</kbd>
    Up,
    /// <kbd>↓</kbd>
    Down,

    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,

    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Apostrophe,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd>
    /// (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote
    /// control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    // Legacy modifier key. Also called "Super" in certain places.
    Meta,
    // Legacy modifier key.
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,
}