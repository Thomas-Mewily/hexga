use super::*;


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
    Window(WindowEvent),
}
impl<UserEvent> Debug for AppEvent<UserEvent> where UserEvent: IUserEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self
        {
            Self::UserEvent(v) => write!(f, "{:?}", v),
            Self::Window(v) => write!(f, "{:?}", v),
        }
    }
}
impl<U> From<WindowEvent> for AppEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }

