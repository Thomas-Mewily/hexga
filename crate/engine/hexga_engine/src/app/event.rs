use super::*;


pub trait AppUserEvent : Async {}
impl<E> AppUserEvent for E where E: Async {}

/*
#[derive(Debug, Clone, PartialEq)]
pub enum AppMessage<Event=AppEvent>
{
    Event(Event),
    Flow(AppFlow),
}
    

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
*/

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent<User=UserEvent>
{
    Input(InputEvent),
    Window(WindowEvent),
    User(User),
    //Custom(AppCustomEvent),
}

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

