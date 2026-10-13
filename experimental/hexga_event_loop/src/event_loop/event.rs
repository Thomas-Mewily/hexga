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



pub enum PlatformCustomEvent<Ev>
{
    Platform(PlatformEvent),
    Custom(Ev),
}

impl<Ev> PlatformCustomEvent<Ev>
{
    pub fn as_platform(&self) -> Option<&PlatformEvent>
    {
        match self
        {
            PlatformCustomEvent::Platform(platform_event) => Some(platform_event),
            PlatformCustomEvent::Custom(_) => None,
        }
    }
    pub fn as_platform_mut(&mut self) -> Option<&mut PlatformEvent>
    {
        match self
        {
            PlatformCustomEvent::Platform(platform_event) => Some(platform_event),
            PlatformCustomEvent::Custom(_) => None,
        }
    }

    pub fn as_custom(&self) -> Option<&Ev>
    {
        match self
        {
            PlatformCustomEvent::Platform(custom_event) => None,
            PlatformCustomEvent::Custom(custom_event) => Some(custom_event),
        }
    }
    pub fn as_custom_mut(&mut self) -> Option<&mut Ev>
    {
        match self
        {
            PlatformCustomEvent::Platform(custom_event) => None,
            PlatformCustomEvent::Custom(custom_event) => Some(custom_event),
        }
    }
}

impl<Ev> PlatformCustomEvent<Ev>
{
    pub fn replace_custom_event_from_fn<Ev2, F>(self, init: F) -> (PlatformCustomEvent<Ev2>, Option<Ev>)
    where
        F: FnOnce() -> Ev2,
    {
        match self
        {
            PlatformCustomEvent::Platform(platform_event) => (PlatformCustomEvent::Platform(platform_event), None),
            PlatformCustomEvent::Custom(c) => (PlatformCustomEvent::Custom(init()), Some(c)),
        }
    }
}

impl<E,Ev> From<E> for PlatformCustomEvent<Ev> where E: Into<PlatformEvent>
{
    fn from(event: E) -> Self { Self::Platform(event.into()) }
}