use super::*;

pub(crate) enum AppInternalEvent
{
    Gpu(GpuMessage),
    //Custom(CustomEvent),
}


#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage<Event=AppEvent>
{
    Event(Event),
    Flow(AppFlow),
}

pub struct AppEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppFlow
{
    Resumed,
    Suspended,
    Update(DeltaTime),
    Draw,
    //Exit,
}

impl From<AppEvent> for AppMessage
{
    fn from(value: AppEvent) -> Self { AppMessage::Event(value) }
}
impl<Ev> From<AppFlow> for AppMessage<Ev>
{
    fn from(value: AppFlow) -> Self { AppMessage::Flow(value) }
}