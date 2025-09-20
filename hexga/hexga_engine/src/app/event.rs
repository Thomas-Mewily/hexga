use super::*;

pub(crate) type WinitWindowEvent = winit::event::WindowEvent;

pub(crate) enum AppInternalEvent<E> where E:IEvent
{
    Gpu(GpuEvent),
    Event(AppEvent<E>),
}
impl<E> From<AppEvent<E>> for AppInternalEvent<E> where E:IEvent{ fn from(value: AppEvent<E>) -> Self { Self::Event(value) } }

/* 
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AppMessage<E> where E:IEvent
{
    //Flow(FlowMessage),
    Event(AppEvent<E>),
}
*/

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AppEvent<E> where E:IEvent
{
    Input(InputEvent),
    Custom(E)
}

/* 
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FlowMessage
{
    Resumed,
    Paused,
    Update,
    Draw,
    Exit,
}

impl<E> From<FlowMessage> for AppEvent<E> where E:IEvent
{
    fn from(flow: FlowMessage) -> Self {
        Self::Flow(flow)
    }
}*/
impl<E> From<InputEvent> for AppEvent<E> where E:IEvent
{
    fn from(input: InputEvent) -> Self {
        Self::Input(input)
    }
}
impl<E> From<KeyEvent> for AppEvent<E> where E:IEvent
{
    fn from(key: KeyEvent) -> Self {
        Self::Input(key.into())
    }
}