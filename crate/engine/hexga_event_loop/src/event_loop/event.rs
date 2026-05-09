use super::*;


pub trait PlatformCustomEvent : Async {}
impl<E> PlatformCustomEvent for E where E: Async {}


#[derive(Debug, Clone, PartialEq)]
pub enum PlatformEvent<Ev>
{
    Input(InputEvent),
    Window(WindowEvent),
    Custom(Ev),
}

impl<User> From<InputEvent> for PlatformEvent<User>
{
    fn from(input: InputEvent) -> Self { Self::Input(input) }
}
impl<User> From<KeyEvent> for PlatformEvent<User>
{
    fn from(key: KeyEvent) -> Self { Self::Input(key.into()) }
}
impl<User> From<WindowEvent> for PlatformEvent<User>
{
    fn from(window: WindowEvent) -> Self { Self::Window(window.into()) }
}

