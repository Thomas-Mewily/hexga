use super::*;

pub trait IUserEvent : 'static {}
impl IUserEvent for () {}


pub trait App<UserEvent=()> where UserEvent:IUserEvent
{
    fn pause(&mut self) {}
    fn resume(&mut self) {}

    fn handle_event(&mut self, event : AppEvent<UserEvent>) { let _ = event; }

    fn update(&mut self) {}
    fn draw(&mut self) {}
}

#[derive(Debug)]
pub(crate) enum AppInternalEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent),
    WindowInternal(WindowInternalEvent),
}
impl<U> From<WindowInternalEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowInternalEvent) -> Self { Self::WindowInternal(value) } }
impl<U> From<WindowEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }

#[non_exhaustive]
#[derive(Debug)]
pub enum AppEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent)
}
impl<U> From<WindowEvent> for AppEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }