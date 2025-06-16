use std::default;

use crate::*;

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventAction
{
    Press,
    #[default]
    Release,
}
impl IEventAction for EventAction
{
    fn is_press(&self) -> bool { matches!(self, EventAction::Press) }
    fn is_release(&self) -> bool { matches!(self, EventAction::Release) }
}

pub trait IEventAction
{
    fn is_press(&self) -> bool;
    fn is_release(&self) -> bool { !self.is_press()}
}

impl From<bool> for EventAction
{
    fn from(value: bool) -> Self {
        if value { Self::Press } else { Self::Release }
    }
}
/*
pub enum Predicate<T>
{
    Value(T),
    Always, // True
    Never, // False
}

pub enum ActionPredicate?
{
    Action,
    Any(An)
}
*/

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Event
{
    Window  (WindowEvent),
    Mouse   (MouseEvent ),
    Key     (KeyEvent   ),
    Modifier(ModifierEvent),
    Touch   (TouchEvent ),
}

impl Debug for Event
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Window(v) => write!(f, "{:?}", v),
            Event::Mouse(v) => write!(f, "{:?}", v),
            Event::Key(v) => write!(f, "{:?}", v),
            Event::Modifier(v) => write!(f, "{:?}", v),
            Event::Touch(v) => write!(f, "{:?}", v),
        }
    }
}

impl Event
{
    pub fn as_mouse(&self) -> Option<&MouseEvent> {  if let Event::Mouse(e) = self { Some(e) } else { None } }
    pub fn as_key(&self) -> Option<&KeyEvent> {  if let Event::Key(e) = self { Some(e) } else { None } }
    pub fn as_modifier(&self) -> Option<&ModifierEvent> {  if let Event::Modifier(e) = self { Some(e) } else { None } }
    pub fn as_touch(&self) -> Option<&TouchEvent> {  if let Event::Touch(e) = self { Some(e) } else { None } }
    pub fn as_window(&self) -> Option<&WindowEvent> {  if let Event::Window(e) = self { Some(e) } else { None } }

    pub fn is_mouse(&self) -> bool { self.as_mouse().is_some() }
    pub fn is_key(&self) -> bool { self.as_key().is_some() }
    pub fn is_modifier(&self) -> bool { self.as_modifier().is_some() }
    pub fn is_touch(&self) -> bool { self.as_touch().is_some() }
    pub fn is_window(&self) -> bool { self.as_window().is_some() }
}

impl KeyboardShortcuts for Event
{
    fn is_copy(&self) -> bool { self.as_key().map_or(false, |k| k.is_copy()) }
    fn is_paste(&self) -> bool { self.as_key().map_or(false, |k| k.is_paste()) }
    fn is_alt_f4(&self) -> bool { self.as_key().map_or(false, |k| k.is_alt_f4()) }
}

impl From<WindowEvent> for Event { fn from(value: WindowEvent) -> Self { Self::Window(value) } }
impl From<MouseMoveEvent  > for Event { fn from(value: MouseMoveEvent) -> Self { Self::Mouse(MouseEvent::Move(value)) } }
impl From<MouseButtonEvent> for Event { fn from(value: MouseButtonEvent) -> Self { Self::from(MouseEvent::Button(value)) } }
impl From<MouseEvent > for Event { fn from(value: MouseEvent) -> Self { Self::Mouse(value) } }
impl From<KeyEvent   > for Event { fn from(value: KeyEvent) -> Self { Self::from(Event::Key(value)) } }
impl From<ModifierEvent> for Event { fn from(value: ModifierEvent) -> Self { Self::from(Event::Modifier(value)) } }
impl From<TouchEvent > for Event { fn from(value: TouchEvent) -> Self { Self::Touch(value) } }
