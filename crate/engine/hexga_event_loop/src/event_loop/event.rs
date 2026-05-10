use super::*;


pub trait PlatformCustomEvent : Async {}
impl<E> PlatformCustomEvent for E where E: Async {}


#[derive(Debug, Clone, PartialEq)]
pub enum Event<Ev>
{
    Key(KeyEvent),

    /// Clipboard Cut
    Cut,
    /// Clipboard Paste
    Paste(String),
    /// Clipboard Copy
    Copy,

    // Window:
    Resize(Point2),
    Move(Point2),
    Open,
    Close,
    Destroy,

    Custom(Ev),
}

impl<User> From<KeyEvent> for Event<User>
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}
