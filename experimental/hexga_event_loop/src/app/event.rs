use super::*;

pub trait IAppEvent: Async {}
impl<E> IAppEvent for E where E: Async {}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent
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
    Resize(Vec2i),
    Move(Vec2i),
    Open,
    Close,
    Destroy,
    //Custom(Ev),
}

impl From<KeyEvent> for AppEvent
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMessage<Ev=()>
{
    Event(AppEvent),
    Flow(AppFlow),
    Custom(Ev),
}


#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum AppFlow
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

impl<Ev> AppMessage<Ev>
{
    pub fn replace_custom_event_from_fn<Ev2, F>(self, init: F) -> (AppMessage<Ev2>, Option<Ev>)
    where
        F: FnOnce() -> Ev2,
    {
        match self
        {
            AppMessage::Event(v) => (AppMessage::Event(v), None),
            AppMessage::Flow(v) => (AppMessage::Flow(v), None),
            AppMessage::Custom(c) => (AppMessage::Custom(init()), Some(c)),
        }
    }
    
    pub fn remove_custom<Ev2>(self) -> Option<AppMessage<Ev2>>
    {
        match self
        {
            AppMessage::Event(ev) => Some(AppMessage::Event(ev)),
            AppMessage::Flow(flow) => Some(AppMessage::Flow(flow)),
            AppMessage::Custom(_) => None,
        }
    }
}

impl<Ev> From<AppEvent> for AppMessage<Ev>
{
    fn from(event: AppEvent) -> Self { Self::Event(event) }
}
impl<Ev> From<AppFlow> for AppMessage<Ev>
{
    fn from(flow: AppFlow) -> Self { Self::Flow(flow) }
}