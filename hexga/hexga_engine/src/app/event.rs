use super::*;

pub trait AsyncDebug: Async + Debug {}
impl<T> AsyncDebug for T where T: Async + Debug {}

//pub type AppCustomEvent = Box<dyn AsyncDebug>;

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent
{
    Input(InputEvent),
    Window(WindowEvent),
    //Custom(AppCustomEvent),
}
//impl From<AppCustomEvent> for AppEvent { fn from(custom: AppCustomEvent) -> Self { Self::Custom(custom) } }
impl From<InputEvent> for AppEvent
{
    fn from(input: InputEvent) -> Self { Self::Input(input) }
}
impl From<KeyEvent> for AppEvent
{
    fn from(key: KeyEvent) -> Self { Self::Input(key.into()) }
}
impl From<WindowEvent> for AppEvent
{
    fn from(window: WindowEvent) -> Self { Self::Window(window.into()) }
}

/*
#[derive(Debug, Clone)]
pub struct LibAppMessage<'a>
{
    pub event_loop: &'a EventLoopActive,
    pub message: AppMessage,
}
*/

#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage
{
    Event(AppEvent),
    Flow(FlowMessage),
}
impl From<AppEvent> for AppMessage
{
    fn from(value: AppEvent) -> Self { AppMessage::Event(value) }
}
impl From<FlowMessage> for AppMessage
{
    fn from(value: FlowMessage) -> Self { AppMessage::Flow(value) }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowMessage
{
    Resumed,
    Suspended,
    Update(DeltaTime),
    Draw,
    //Exit,
}

pub(crate) enum AppInternalEvent
{
    Gpu(GpuMessage),
    //Custom(CustomEvent),
}
