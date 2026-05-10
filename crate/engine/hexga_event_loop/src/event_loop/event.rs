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

impl<User> From<KeyEvent> for PlatformEvent<User>
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}
