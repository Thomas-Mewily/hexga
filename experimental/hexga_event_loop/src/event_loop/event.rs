use super::*;

pub trait IPlatformEvent: Async {}
impl<E> IPlatformEvent for E where E: Async {}

#[derive(Debug, Clone, PartialEq)]
pub enum PlatformEvent
{
    Key(KeyEvent),

    // The comment was taken from the notan codebase
    /// Text cut to the clipboard
    Cut,
    /// Text pasted from the clipboard
    Paste(String),
    /// Text copied to the clipboard
    Copy,

    // Window:
    Resize(Vec2I),
    Move(Vec2I),
    Open,
    Close,
    Destroy,
    //Custom(Ev),
}

impl From<KeyEvent> for PlatformEvent
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}

pub enum PlatformMessage<Ev>
{
    Event(PlatformEvent),
    Flow(PlatformFlow),
    Custom(Ev),
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum PlatformFlow
{
    Update(Duration),
    Draw,
    Resume,
    Pause,
    Exit,
}

/*
impl<Ev> PlatformMessage<Ev>
{
    pub fn as_platform(&self) -> Option<&PlatformEvent>
    {
        match self
        {
            PlatformMessage::Platform(platform_event) => Some(platform_event),
            _ => None,
        }
    }
    pub fn as_platform_mut(&mut self) -> Option<&mut PlatformEvent>
    {
        match self
        {
            PlatformMessage::Platform(platform_event) => Some(platform_event),
            _ => None,
        }
    }

    pub fn as_custom(&self) -> Option<&Ev>
    {
        match self
        {
            PlatformMessage::Custom(custom_event) => Some(custom_event),
            _ => None,
        }
    }
    pub fn as_custom_mut(&mut self) -> Option<&mut Ev>
    {
        match self
        {
            PlatformMessage::Custom(custom_event) => Some(custom_event),
            _ => None,
        }
    }
}
*/

impl<Ev> PlatformMessage<Ev>
{
    pub fn replace_custom_event_from_fn<Ev2, F>(self, init: F) -> (PlatformMessage<Ev2>, Option<Ev>)
    where
        F: FnOnce() -> Ev2,
    {
        match self
        {
            PlatformMessage::Event(v) => (PlatformMessage::Event(v), None),
            PlatformMessage::Flow(v) => (PlatformMessage::Flow(v), None),
            PlatformMessage::Custom(c) => (PlatformMessage::Custom(init()), Some(c)),
        }
    }
}

impl<E, Ev> From<E> for PlatformMessage<Ev>
where
    E: Into<PlatformEvent>,
{
    fn from(event: E) -> Self { Self::Event(event.into()) }
}
