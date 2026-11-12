use super::*;

pub trait IAppEvent: Async {}
impl<E> IAppEvent for E where E: Async {}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent
{
    Key(KeyEvent),
    KeyModifiers(KeyModifiersFlags),

    MouseMove(Vec2),
    MouseFocus(bool),
    MouseWheel(MouseWheel),
    MouseClick(MouseClick),

    Theme(AppTheme),

    Touch(Touch),

    // The comment was taken from the notan codebase
    /// Text cut to the clipboard
    ClipboardCut,
    /// Text pasted from the clipboard
    ClipboardPaste(String),
    /// Need to copy the text/image/selection to the clipboard
    ClipboardCopy,

    FileDrop(PathBuf),
    FileHover(PathBuf),
    FileHoveredCancel,

    // Window:
    /// The window was resized
    WindowResize(Vec2i),
    /// The window was moved
    WindowMove(Vec2i),
    /// The window is open
    WindowOpen,
    /// The window will be closed
    WindowClose,
    /// The window has been destroyed
    WindowDestroy,
    /// The window gain/lose focus
    WindowFocus(bool),
    MouseButtonEvent {},
    //Custom(Ev),
}

impl From<KeyEvent> for AppEvent
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppMessage<Ev = ()>
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
