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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyAction
{
    pub code: KeyCode,
    pub state: ButtonState,
}

impl Has<KeyCode> for KeyAction
{
    fn retrieve(&self) -> KeyCode { self.code }
}
impl Has<ButtonState> for KeyAction
{
    fn retrieve(&self) -> ButtonState { self.state }
}

/*
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyActionMods
{
    pub code : KeyCode,
    pub state: ButtonState,
    pub mods : KeyModsFlags,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyActionModsBinding
{
    pub code : KeyCode,
    pub state: ButtonState,
    pub mods : KeyModsFlags,
}

impl PartialEq<KeyActionModsBinding> for KeyActionMods
{
    fn eq(&self, other: &KeyActionModsBinding) -> bool {
        other.eq(self)
    }
}
impl PartialEq<KeyActionMods> for KeyActionModsBinding
{
    fn eq(&self, other: &KeyActionMods) -> bool 
    {
        self.code == other.code  && self.state == other.state && self.mods.matches(other.mods) 
    }
}

impl Has<KeyCode> for KeyActionMods
{
    fn retrieve(&self) -> KeyCode { self.code }
}
impl Has<ButtonState> for KeyActionMods
{
    fn retrieve(&self) -> ButtonState { self.state }
}
impl Has<KeyModsFlags> for KeyActionMods
{
    fn retrieve(&self) -> KeyModsFlags { self.mods }
}
impl KeyActionMods
{
    pub fn action(&self) -> KeyAction { KeyAction{ code: self.code, state: self.state }}
}
*/


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
    pub state     : ButtonState,
    pub modifiers : KeyModifiersFlags,
    pub repeat    : ButtonRepeat,
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
    ///
    /// [`key_without_modifiers`]: crate::platform::modifier_supplement::KeyEventExtModifierSupplement::key_without_modifiers
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

impl Has<ButtonState> for KeyEvent
{
    fn retrieve(&self) -> ButtonState {
        self.state
    }
}
impl Has<KeyCode> for KeyEvent
{
    fn retrieve(&self) -> KeyCode {
        self.code
    }
}
impl Has<ButtonRepeat> for KeyEvent
{
    fn retrieve(&self) -> ButtonRepeat {
        self.repeat
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
            ButtonRepeat::Repeated
        }
        else
        {
            ButtonRepeat::NotRepeated
        };
        let state = if pressed
        {
            ButtonState::Down
        }
        else
        {
            ButtonState::Up
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

pub trait KeyConstant
{
    const EXIT : Self;
    const COPY : Self;
    const PASTE : Self;
    const CUT : Self;
    const SAVE : Self;
    const UNDO : Self;
    const REDO : Self;
}