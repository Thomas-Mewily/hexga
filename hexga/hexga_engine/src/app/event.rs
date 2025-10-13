use super::*;

pub trait ICustomEvent : 'static + Debug + Send + Any {}
impl<T> ICustomEvent for T where T: 'static + Debug + Send + Any {}

pub type CustomEvent = Box<dyn ICustomEvent>;


pub(crate) enum AppInternalEvent
{
    Gpu(GpuEvent),
    //Custom(CustomEvent),
}


#[derive(Debug)]
pub enum AppEvent
{
    Input(InputEvent),
    Window(WindowEvent),
    Custom(CustomEvent),
}

impl From<CustomEvent> for AppEvent
{
    fn from(custom: CustomEvent) -> Self {
        Self::Custom(custom)
    }
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
impl From<WindowEvent> for AppEvent
{
    fn from(window: WindowEvent) -> Self {
        Self::Window(window.into())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FlowMessage
{
    Resumed,
    Paused,
    Update(DeltaTime),
    Draw,
    //Exit,
}