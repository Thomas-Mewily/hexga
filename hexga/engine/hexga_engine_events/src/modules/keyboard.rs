use crate::*;




// pub struct KeyEventPredicate

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyEvent
{
    pub character : Option<char>,
    pub key       : KeyCode,
    pub modifiers : KeyModsFlags,

    pub repeat  : bool,
    pub action  : EventAction,
}
impl KeyEvent
{
    pub fn char(&self) -> Option<char> { self.character }
    pub fn key (&self) -> KeyCode { self.key }
    pub fn is_repeat(&self) -> bool { self.repeat }
    pub fn is_not_repeat(&self) -> bool { !self.repeat }
    pub fn action(&self) -> EventAction { self.action }
}

pub trait KeyboardShortcuts
{
    fn is_alt_f4(&self) -> bool;

    /// `Control + C` just pressed
    fn is_copy(&self) -> bool;
    /// `Control + V` just pressed
    fn is_paste(&self) -> bool;
}

impl KeyboardShortcuts for KeyEvent
{
    fn is_alt_f4(&self) -> bool
    {
        self.action.is_press() && self.modifiers.contains_any(KeyMods::Alt) && self.key == KeyCode::F4 && self.is_not_repeat()
    }

    fn is_copy(&self) -> bool
    {
        self.action.is_press() &&
        (
            self.modifiers.contains_any(KeyMods::Control) && self.key == KeyCode::C && self.is_not_repeat() ||
            self.key == KeyCode::Copy
        )
    }

    fn is_paste(&self) -> bool
    {
        self.action.is_press() &&
        (
            self.modifiers.contains_any(KeyMods::Control) && self.key == KeyCode::V ||
            self.key == KeyCode::Paste
        )
    }
}
impl IEventAction for KeyEvent
{
    fn is_press(&self) -> bool { self.action.is_not_default() }
}

impl Debug for KeyEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyEvent").field("keycode", &self.key)
            .field_if_not_default("modifiers", &self.modifiers)
            .field("key", &self.key)
            .field_if_not_default("char", &self.character)
            .field_if_not_default("repeat", &self.repeat)
            .field("action", &self.action).finish()
    }
}


#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[bitindex]
#[repr(u8)]
pub enum KeyMods
{
    ShiftLeft,
    ShiftRight,
    Shift = Self::ShiftLeft | Self::ShiftRight,

    ControlLeft,
    ControlRight,
    Control = Self::ControlLeft | Self::ControlRight,

    AltLeft,
    AltRight,
    Alt = Self::AltLeft | Self::AltRight,

    SuperLeft,
    SuperRight,
    Super = Self::SuperLeft | Self::SuperRight,
}

impl KeyMods
{
    pub fn from_keycode(key : KeyCode) -> Option<Self> { Self::try_from(key).ok() }
}

impl TryFrom<KeyCode> for KeyMods
{
    type Error = ();
    fn try_from(value: KeyCode) -> Result<KeyMods, Self::Error> {
        match value
        {
            KeyCode::ShiftLeft => Ok(Self::ShiftLeft),
            KeyCode::ShiftRight => Ok(Self::ShiftRight),
            KeyCode::ControlLeft => Ok(Self::ControlLeft),
            KeyCode::ControlRight => Ok(Self::ControlRight),
            KeyCode::AltLeft => Ok(Self::AltLeft),
            KeyCode::AltRight => Ok(Self::AltRight),
            KeyCode::SuperLeft => Ok(Self::SuperLeft),
            KeyCode::SuperRight => Ok(Self::SuperRight),
            _ => Err(()),
        }
    }
}

/// Keymodifiers changed
pub type ModifierEvent = KeyModsFlags;

impl Default for KeyModsFlags
{
    fn default() -> Self { Self::ZERO }
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyCodeUnknow
{
    /// Unknow
    Unknow,
    /// An Android "scancode".
    Android(u32),
    /// A macOS "scancode".
    MacOS(u16),
    /// A Windows "scancode".
    Windows(u16),
    /// An XKB "keycode".
    Xkb(u32),
}
impl std::fmt::Debug for KeyCodeUnknow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use KeyCodeUnknow::{Android, MacOS, Unknow, Windows, Xkb};
        let mut debug_tuple;
        match self {
            Unknow => {
                debug_tuple = f.debug_tuple("Unknow");
            },
            Android(code) => {
                debug_tuple = f.debug_tuple("Android");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            MacOS(code) => {
                debug_tuple = f.debug_tuple("MacOS");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            Windows(code) => {
                debug_tuple = f.debug_tuple("Windows");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
            Xkb(code) => {
                debug_tuple = f.debug_tuple("Xkb");
                debug_tuple.field(&format_args!("0x{code:04X}"));
            },
        }
        debug_tuple.finish()
    }
}

// Based on
// - [Winit KeyCode](https://docs.rs/winit/0.30.11/winit/keyboard/enum.KeyCode.html)
// - [Winit-gtk](https://docs.rs/winit-gtk/latest/winit/event/enum.VirtualKeyCode.html)
// - [Miniquad](https://docs.rs/miniquad/latest/miniquad/enum.KeyCode.html)
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyCode
{
    Unknow(KeyCodeUnknow),

          /// <kbd>0</kbd> on a US keyboard.
    Key0, /// <kbd>1</kbd> on a US keyboard.
    Key1, /// <kbd>2</kbd> on a US keyboard.
    Key2, /// <kbd>3</kbd> on a US keyboard.
    Key3, /// <kbd>4</kbd> on a US keyboard.
    Key4, /// <kbd>5</kbd> on a US keyboard.
    Key5, /// <kbd>6</kbd> on a US keyboard.
    Key6, /// <kbd>7</kbd> on a US keyboard.
    Key7, /// <kbd>8</kbd> on a US keyboard.
    Key8, /// <kbd>9</kbd> on a US keyboard.
    Key9,

       /// <kbd>a</kbd> on a US keyboard. Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    A, /// <kbd>b</kbd> on a US keyboard.
    B, /// <kbd>c</kbd> on a US keyboard.
    C, /// <kbd>d</kbd> on a US keyboard.
    D, /// <kbd>e</kbd> on a US keyboard.
    E, /// <kbd>f</kbd> on a US keyboard.
    F, /// <kbd>g</kbd> on a US keyboard.
    G, /// <kbd>h</kbd> on a US keyboard.
    H, /// <kbd>i</kbd> on a US keyboard.
    I, /// <kbd>j</kbd> on a US keyboard.
    J, /// <kbd>k</kbd> on a US keyboard.
    K, /// <kbd>l</kbd> on a US keyboard.
    L, /// <kbd>m</kbd> on a US keyboard.
    M, /// <kbd>n</kbd> on a US keyboard.
    N, /// <kbd>o</kbd> on a US keyboard.
    O, /// <kbd>p</kbd> on a US keyboard.
    P, /// <kbd>q</kbd> on a US keyboard. Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    Q, /// <kbd>r</kbd> on a US keyboard.
    R, /// <kbd>s</kbd> on a US keyboard.
    S, /// <kbd>t</kbd> on a US keyboard.
    T, /// <kbd>u</kbd> on a US keyboard.
    U, /// <kbd>v</kbd> on a US keyboard.
    V, /// <kbd>w</kbd> on a US keyboard. Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    W, /// <kbd>x</kbd> on a US keyboard.
    X, /// <kbd>y</kbd> on a US keyboard. Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    Y, /// <kbd>z</kbd> on a US keyboard. Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a QWERTZ (e.g., German) keyboard.
    Z,

           /// <kbd> </kbd> (space)
    Space, /// <kbd>←</kbd>
    ArrowRight, /// <kbd>↑</kbd>
    ArrowLeft,  /// <kbd>→</kbd>
    ArrowUp,    /// <kbd>↓</kbd>
    ArrowDown,

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

    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,

    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Pause Break</kbd>
    Pause,

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

    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,

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


    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,

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

impl KeyCode
{
    pub fn is_modifier_key(&self) -> bool
    {
        matches!(self, Self::ShiftLeft | Self::ShiftRight | Self::ControlLeft | Self::ControlRight | Self::AltLeft | Self::AltRight | Self::SuperLeft | Self::SuperRight)
    }
}