use crate::*;


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq)]
pub enum Event
{
    Window  (WindowEvent),
    Mouse   (MouseEvent ),
    Key     (KeyEvent   ),
    Touch   (TouchEvent ),
}

pub struct LocalizedEvent
{
    window : WindowID,
    event  : Event,
}

pub enum AnyEvent
{
    Event(Event),
    Draw,
}

/*
pub enum DeviceEvent
{
    AddedDevice,
    RemovedDevice,
    Mouse(MouseEvent)
}
*/

impl Debug for Event
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Window(v) => write!(f, "{:?}", v),
            Self::Mouse(v) => write!(f, "{:?}", v),
            Self::Key(v) => write!(f, "{:?}", v),
            Self::Touch(v) => write!(f, "{:?}", v),
        }
    }
}

impl From<WindowEvent> for Event { fn from(value: WindowEvent) -> Self { Self::Window(value) } }
//impl From<DropFileEvent> for Event { fn from(value: DropFileEvent) -> Self { Self::from(WindowEvent::DropFile(value)) } }
impl From<MouseMove> for Event { fn from(value: MouseMove) -> Self { Self::Mouse(MouseEvent::Move(value)) } }
impl From<MouseEvent> for Event { fn from(value: MouseEvent) -> Self { Self::Mouse(value) } }
impl From<MouseButtonEvent> for Event { fn from(value: MouseButtonEvent) -> Self { Self::from(MouseEvent::Button(value)) } }
impl From<KeyboardEvent> for Event { fn from(value: KeyboardEvent) -> Self { Self::Key(value) } }
impl From<CharEvent> for Event { fn from(value: CharEvent) -> Self { Self::from(KeyboardEvent::CharEvent(value)) } }
impl From<KeyEvent> for Event { fn from(value: KeyEvent) -> Self { Self::from(KeyboardEvent::KeyEvent(value)) } }
impl From<TouchEvent> for Event { fn from(value: TouchEvent) -> Self { Self::Touch(value) } }
