use super::*;

pub trait AsyncDebug : Async + Debug {}
impl<T> AsyncDebug for T where T: Async + Debug {}

pub type AppCustomEvent = Box<dyn AsyncDebug>;

#[derive(Debug)]
pub enum AppEvent
{
    Input(InputEvent),
    Window(WindowEvent),
    Custom(AppCustomEvent),
}
impl From<AppCustomEvent> for AppEvent { fn from(custom: AppCustomEvent) -> Self { Self::Custom(custom) } }
impl From<InputEvent> for AppEvent { fn from(input: InputEvent) -> Self { Self::Input(input) } }
impl From<KeyEvent> for AppEvent { fn from(key: KeyEvent) -> Self { Self::Input(key.into()) } }
impl From<WindowEvent> for AppEvent { fn from(window: WindowEvent) -> Self { Self::Window(window.into()) } }


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FlowMessage
{
    Resumed,
    Paused,
    Update(DeltaTime),
    Draw,
    //Exit,
}

pub(crate) enum AppInternalEvent
{
    Exit,
    //Gpu(GpuEvent),
    //Custom(CustomEvent),
}