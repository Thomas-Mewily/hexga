use crate::*;

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action
{
    Press,
    Release,
}
impl Action
{
    pub fn is_press(self) -> bool { matches!(self, Action::Press) }
    pub fn is_release(self) -> bool { matches!(self, Action::Release) }
}
impl From<bool> for Action
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
    Touch   (TouchEvent ),
    Device  (DeviceEvent)
}

impl Debug for Event
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Window(v) => write!(f, "{:?}", v),
            Event::Mouse(v) => write!(f, "{:?}", v),
            Event::Key(v) => write!(f, "{:?}", v),
            Event::Touch(v) => write!(f, "{:?}", v),
            Event::Device(v) => write!(f, "{:?}", v),
        }
    }
}

impl From<WindowEvent> for Event { fn from(value: WindowEvent) -> Self { Self::Window(value) } }
impl From<MouseMoveEvent  > for Event { fn from(value: MouseMoveEvent) -> Self { Self::Mouse(MouseEvent::Move(value)) } }
impl From<MouseButtonEvent> for Event { fn from(value: MouseButtonEvent) -> Self { Self::from(MouseEvent::Button(value)) } }
impl From<MouseEvent > for Event { fn from(value: MouseEvent) -> Self { Self::Mouse(value) } }
impl From<KeyEvent   > for Event { fn from(value: KeyEvent) -> Self { Self::from(Event::Key(value)) } }
impl From<TouchEvent > for Event { fn from(value: TouchEvent) -> Self { Self::Touch(value) } }
impl From<DeviceEvent> for Event { fn from(value: DeviceEvent) -> Self { Self::Device(value) } }
