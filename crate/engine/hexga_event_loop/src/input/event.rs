use super::*;

/*
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum InputEvent
{
    Key(KeyEvent),

    /// Clipboard Cut
    Cut,
    /// Clipboard Paste
    Paste(String),
    /// Clipboard Copy
    Copy,
}
impl From<KeyEvent> for InputEvent
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}*/

pub type KeyState = ButtonState;
pub type KeyRepeat = ButtonRepeat;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyAction
{
    pub code: KeyCode,
    pub state: KeyState,
}

impl Has<KeyCode> for KeyAction
{
    fn retrieve(&self) -> KeyCode { self.code }
}
impl Has<KeyState> for KeyAction
{
    fn retrieve(&self) -> KeyState { self.state }
}
impl KeyAction
{
    pub const fn shortcut(self) -> KeyShortcut
    {
        KeyShortcut { code: self.code, modifiers: KeyModifiersFlags::EMPTY }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyShortcut
{
    pub code : KeyCode,
    pub modifiers : KeyModifiersFlags,
}
impl Matches<KeyCode> for KeyShortcut
{
    type Output=bool;
    fn matches(&self, lexem: &KeyCode) -> Self::Output {
        self.code == *lexem
    }
}
impl Matches<KeyEvent> for KeyShortcut
{
    type Output=bool;
    fn matches(&self, lexem: &KeyEvent) -> Self::Output {
        self.code == lexem.code && self.modifiers.matches(&lexem.modifiers)
    }
}

impl From<KeyCode> for KeyShortcut
{
    fn from(code: KeyCode) -> Self {
        Self::from_code(code)
    }
}
impl From<KeyAction> for KeyShortcut
{
    fn from(value: KeyAction) -> Self {
        Self::from_action(value)
    }
}
impl KeyShortcut
{
    pub const fn from_code(code: KeyCode) -> Self {
        Self{ code, modifiers: KeyModifiersFlags::EMPTY }
    }

    pub const fn from_action(value: KeyAction) -> Self {
        value.shortcut()
    }

    pub const fn with_modifier(mut self, modifiers: KeyModifiersFlags) -> Self { self.modifiers = modifiers; self }
}

impl Has<KeyCode> for KeyShortcut
{
    fn retrieve(&self) -> KeyCode { self.code }
}
impl Has<KeyModifiersFlags> for KeyShortcut
{
    fn retrieve(&self) -> KeyModifiersFlags { self.modifiers }
}
impl From<KeyShortcut> for KeyAction
{
    fn from(value: KeyShortcut) -> Self {
        value.action()
    }
}
impl KeyShortcut
{
    pub const fn action(self) -> KeyAction { KeyAction { code: self.code, state: KeyState::Down }}
}


// winit KeyEvent wrapper
/// The text equivalent of KeyState
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    /// Physical Key.
    ///
    /// Represents the position of a key independent of the currently active layout.
    ///
    /// It also uniquely identifies the physical key (i.e. it's mostly synonymous with a scancode).
    /// The most prevalent use case for this is games. For example the default keys for the player
    /// to move around might be the W, A, S, and D keys on a US layout. The position of these keys
    /// is more important than their label, so they should map to Z, Q, S, and D on an "AZERTY"
    /// layout. (This value is `KeyCode::KeyW` for the Z key on an AZERTY layout.)
    ///
    /// ## Caveats
    ///
    /// - Certain niche hardware will shuffle around physical key positions, e.g. a keyboard that
    ///   implements DVORAK in hardware (or firmware)
    /// - Your application will likely have to handle keyboards which are missing keys that your
    ///   own keyboard has.
    /// - Certain `KeyCode`s will move between a couple of different positions depending on what
    ///   layout the keyboard was manufactured to support.
    ///
    ///  **Because of these caveats, it is important that you provide users with a way to configure
    ///  most (if not all) keybinds in your application.**
    ///
    /// ## `Fn` and `FnLock`
    ///
    /// `Fn` and `FnLock` key events are *exceedingly unlikely* to be emitted by Hexga (backed by Winit). These keys
    /// are usually handled at the hardware or OS level, and aren't surfaced to applications.
    pub code    : KeyCode,

    /// Whether the key is being pressed or released.
    ///
    /// See the [`ElementState`] type for more details.
    pub state     : KeyState,
    pub modifiers : KeyModifiersFlags,
    pub repeat    : KeyRepeat,
    /// Logical Key.
    /// 
    /// This value is affected by all modifiers except <kbd>Ctrl</kbd>.
    ///
    /// This has two use cases:
    /// - Allows querying whether the current input is a Dead key.
    /// - Allows handling key-bindings on platforms which don't support [`key_without_modifiers`].
    ///
    /// If you use this field (or [`key_without_modifiers`] for that matter) for keyboard
    /// shortcuts, **it is important that you provide users with a way to configure your
    /// application's shortcuts so you don't render your application unusable for users with an
    /// incompatible keyboard layout.**
    ///
    /// ## Platform-specific
    /// - **Web:** Dead keys might be reported as the real key instead of `Dead` depending on the
    ///   browser/OS.
    pub key     : Key,

    /// Contains the location of this key on the keyboard.
    ///
    /// Certain keys on the keyboard may appear in more than once place. For example, the "Shift"
    /// key appears on the left side of the QWERTY keyboard as well as the right side. However,
    /// both keys have the same symbolic value. Another example of this phenomenon is the "1"
    /// key, which appears both above the "Q" key and as the "Keypad 1" key.
    ///
    /// This field allows the user to differentiate between keys like this that have the same
    /// symbolic value but different locations on the keyboard.
    ///
    /// See the [`KeyLocation`] type for more details.
    ///
    /// [`KeyLocation`]: crate::keyboard::KeyLocation
    pub location  : KeyLocation,

    /// Contains the text produced by this keypress.
    ///
    /// In most cases this is identical to the content
    /// of the `Character` variant of `logical_key`.
    /// However, on Windows when a dead key was pressed earlier
    /// but cannot be combined with the character from this
    /// keypress, the produced text will consist of two characters:
    /// the dead-key-character followed by the character resulting
    /// from this keypress.
    ///
    /// An additional difference from `logical_key` is that
    /// this field stores the text representation of any key
    /// that has such a representation. For example when
    /// `logical_key` is `Key::Named(NamedKey::Enter)`, this field is `Some("\r")`.
    ///
    /// This is `None` if the current keypress cannot
    /// be interpreted as text.
    ///
    /// See also: `text_with_all_modifiers()`
    pub text      : Option<KeyText>,
}

pub type KeyText = winit::keyboard::SmolStr;

impl Has<KeyState> for KeyEvent
{
    fn retrieve(&self) -> KeyState {
        self.state
    }
}
impl Has<KeyCode> for KeyEvent
{
    fn retrieve(&self) -> KeyCode {
        self.code
    }
}
impl Has<KeyRepeat> for KeyEvent
{
    fn retrieve(&self) -> KeyRepeat {
        self.repeat
    }
}
impl Has<KeyModifiersFlags> for KeyEvent
{
    fn retrieve(&self) -> KeyModifiersFlags {
        self.modifiers
    }
}

impl From<winit::event::KeyEvent> for KeyEvent
{
    fn from(event: winit::event::KeyEvent) -> Self
    {
        let code = KeyCode::from(event.physical_key);
        let pressed = event.state.is_pressed();

        let repeat = if event.repeat
        {
            KeyRepeat::Repeated
        }
        else
        {
            KeyRepeat::NotRepeated
        };
        let state = if pressed
        {
            KeyState::Down
        }
        else
        {
            KeyState::Up
        };

        
        KeyEvent 
        {
            code,
            state,
            modifiers: KeyModifiersFlags::EMPTY,
            key: event.logical_key.into(),
            location: event.location.into(),
            text: event.text,
            repeat,
        }
    }
}



pub trait KeyboardShortcuts
{
    /// `Alt + F4`
    fn is_exit(&self) -> bool;
    /// `F5`
    fn is_reload(&self) -> bool;
    
    /// `F11`
    fn is_fullscreen(&self) -> bool;

    /// `Control + C`
    fn is_copy(&self) -> bool;
    /// `Control + V`
    fn is_paste(&self) -> bool;
    /// `Control + X`
    fn is_cut(&self) -> bool;

    /// `Control + O`
    fn is_open(&self) -> bool;
    /// `Control + S`
    fn is_save(&self) -> bool;

    /// `Control + N`
    fn is_new(&self) -> bool;
    /// `Control + P`
    fn is_print(&self) -> bool;

    /// `Control + F`
    fn is_search(&self) -> bool;

    /// `Control + Z`
    fn is_undo(&self) -> bool;
    /// `Control + Y`
    fn is_redo(&self) -> bool;
}

impl<S> KeyboardShortcuts for S where S: KeyConstants + PartialEq
{
    fn is_exit(&self) -> bool {
        *self == Self::EXIT
    }

    fn is_reload(&self) -> bool {
        *self == Self::RELOAD
    }

    fn is_fullscreen(&self) -> bool 
    {
        *self == Self::FULLSCREEN
    }

    fn is_copy(&self) -> bool {
        *self == Self::COPY
    }

    fn is_paste(&self) -> bool {
        *self == Self::PASTE
    }

    fn is_cut(&self) -> bool {
        *self == Self::CUT
    }

    fn is_open(&self) -> bool {
        *self == Self::OPEN
    }

    fn is_save(&self) -> bool {
        *self == Self::SAVE
    }

    fn is_new(&self) -> bool {
        *self == Self::NEW
    }

    fn is_print(&self) -> bool {
        *self == Self::PRINT
    }

    fn is_search(&self) -> bool {
        *self == Self::SEARCH
    }

    fn is_undo(&self) -> bool {
        *self == Self::UNDO
    }

    fn is_redo(&self) -> bool {
        *self == Self::REDO
    }
}

pub trait KeyConstants
{
    /// `Alt + F4`
    const EXIT: Self;
    
    /// `F5`
    const RELOAD: Self;

    /// `F11`
    const FULLSCREEN: Self;
    
    /// `Control + C`
    const COPY: Self;
    
    /// `Control + V`
    const PASTE: Self;
    
    /// `Control + X`
    const CUT: Self;
    
    /// `Control + O`
    const OPEN: Self;
    
    /// `Control + S`
    const SAVE: Self;
    
    /// `Control + N`
    const NEW: Self;
    
    /// `Control + P`
    const PRINT: Self;
    
    /// `Control + F`
    const SEARCH: Self;
    
    /// `Control + Z`
    const UNDO: Self;
    
    /// `Control + Y`
    const REDO: Self;
}
impl KeyConstants for KeyShortcut
{
    const EXIT: Self = KeyCode::F4.shortcut().with_modifier(KeyModifiersFlags::Alt);
    const RELOAD: Self = KeyCode::F5.shortcut();
    const FULLSCREEN: Self = KeyCode::F11.shortcut();
    const COPY: Self = KeyCode::C.shortcut().with_modifier(KeyModifiersFlags::Control);
    const PASTE: Self = KeyCode::V.shortcut().with_modifier(KeyModifiersFlags::Control);
    const CUT: Self = KeyCode::X.shortcut().with_modifier(KeyModifiersFlags::Control);
    const OPEN: Self = KeyCode::O.shortcut().with_modifier(KeyModifiersFlags::Control);
    const SAVE: Self = KeyCode::S.shortcut().with_modifier(KeyModifiersFlags::Control);
    const NEW: Self = KeyCode::N.shortcut().with_modifier(KeyModifiersFlags::Control);
    const PRINT: Self = KeyCode::P.shortcut().with_modifier(KeyModifiersFlags::Control);
    const SEARCH: Self = KeyCode::F.shortcut().with_modifier(KeyModifiersFlags::Control);
    const UNDO: Self = KeyCode::Z.shortcut().with_modifier(KeyModifiersFlags::Control);
    const REDO: Self = KeyCode::Y.shortcut().with_modifier(KeyModifiersFlags::Control);
}