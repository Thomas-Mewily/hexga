use super::*;

pub(crate) type WinitWindowEvent = winit::event::WindowEvent;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AppEvent<C> where C:IAppEvent
{
    Flow(FlowEvent),
    Input(InputEvent),
    Custom(C)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FlowEvent
{
    Resumed,
    Paused,
    Update,
    Draw,
    Exit,
}

impl<C> From<FlowEvent> for AppEvent<C> where C:IAppEvent
{
    fn from(flow: FlowEvent) -> Self {
        Self::Flow(flow)
    }
}
impl<C> From<InputEvent> for AppEvent<C> where C:IAppEvent
{
    fn from(input: InputEvent) -> Self {
        Self::Input(input)
    }
}
impl<C> From<KeyEvent> for AppEvent<C> where C:IAppEvent
{
    fn from(key: KeyEvent) -> Self {
        Self::Input(key.into())
    }
}