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



/// The text equivalent of KeyState
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    pub action: KeyActionMods,
    pub repeat: ButtonRepeat,
    pub char: Option<char>,
}
impl Deref for KeyEvent
{
    type Target = KeyActionMods;
    fn deref(&self) -> &Self::Target { &self.action }
}
impl DerefMut for KeyEvent
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.action }
}
impl Has<ButtonState> for KeyEvent
{
    fn retrieve(&self) -> ButtonState {
        self.action.retrieve()
    }
}
impl Has<KeyCode> for KeyEvent
{
    fn retrieve(&self) -> KeyCode {
        self.action.retrieve()
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
        let char: Option<char> = match &event.logical_key
        {
            winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };

        let state = if event.state.is_pressed()
            {
                ButtonState::Down
            }
            else
            {
                ButtonState::Up
            };
        let repeat = if event.repeat
            {
                ButtonRepeat::Repeated
            }
            else
            {
                ButtonRepeat::NotRepeated
            };

        let code = event.physical_key.into();

        let action = KeyActionMods{ code, state, mods: KeyModsFlags::EMPTY };

        Self {
            action,
            repeat,
            char,
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