use super::*;

pub(crate) type WinitWindowEvent = winit::event::WindowEvent;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AppEvent
{
    Resumed,
    Paused,
    Update,
    Draw,
    Exit,
    Input(InputEvent),
}
impl From<InputEvent> for AppEvent
{
    fn from(input: InputEvent) -> Self {
        Self::Input(input)
    }
}
impl From<KeyEvent> for AppEvent
{
    fn from(key: KeyEvent) -> Self {
        Self::Input(key.into())
    }
}