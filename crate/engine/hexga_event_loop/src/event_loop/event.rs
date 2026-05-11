use super::*;


pub trait PlatformCustomEvent : Async {}
impl<E> PlatformCustomEvent for E where E: Async {}


#[derive(Debug, Clone, PartialEq)]
pub enum PlatformEvent<Ev=()>
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
    Resize(Point2),
    Move(Point2),
    Open,
    Close,
    Destroy,

    Custom(Ev),
}

impl<Ev> PlatformEvent<Ev>
{
    pub fn replace_custom_event<Ev2,F>(self, init: F) -> (PlatformEvent<Ev2>, Option<Ev>)
        where F: FnOnce() -> Ev2,
    {
        match self
        {
            PlatformEvent::Key(key_event) => (PlatformEvent::Key(key_event), None),
            PlatformEvent::Cut => (PlatformEvent::Cut, None),
            PlatformEvent::Paste(paste) => (PlatformEvent::Paste(paste), None),
            PlatformEvent::Copy => (PlatformEvent::Copy, None),
            PlatformEvent::Resize(size) => (PlatformEvent::Resize(size), None),
            PlatformEvent::Move(pos) => (PlatformEvent::Move(pos), None),
            PlatformEvent::Open => (PlatformEvent::Open, None),
            PlatformEvent::Close => (PlatformEvent::Close, None),
            PlatformEvent::Destroy => (PlatformEvent::Destroy, None),
            PlatformEvent::Custom(c) => (PlatformEvent::Custom(init()), Some(c)),
        }
    }
}

impl<User> From<KeyEvent> for PlatformEvent<User>
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}
