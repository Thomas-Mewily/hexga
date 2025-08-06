use super::*;

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}


pub trait App<UserEvent=()> where UserEvent:IUserEvent
{
    fn update(&mut self) {}
    fn draw(&mut self) {}
    fn handle_event(&mut self, event : AppEvent<UserEvent>) { let _ = event; }

    fn exit(&mut self) {}

    fn pause(&mut self) {}
    fn resume(&mut self) {}
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
pub enum AppEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent)
}
impl<UserEvent> Debug for AppEvent<UserEvent> where UserEvent: IUserEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DResult {
        match self
        {
            Self::UserEvent(v) => write!(f, "{:?}", v),
            Self::Window(v) => write!(f, "{:?}", v),
        }
    }
}
impl<U> From<WindowEvent> for AppEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }