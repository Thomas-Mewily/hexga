use super::*;


#[derive(Debug)]
pub(crate) enum AppInternalEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent),
    WindowInternal(WindowInternalEvent),
    State(StateEvent),
}

impl<U> From<WindowInternalEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowInternalEvent) -> Self { Self::WindowInternal(value) } }
impl<U> From<WindowEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }
impl<U> From<StateEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: StateEvent) -> Self { Self::State(value) } }

#[non_exhaustive]
pub enum AppEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent),
    State(StateEvent),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StateEvent
{
    Paused, Resumed,
}

impl<UserEvent> Debug for AppEvent<UserEvent> where UserEvent: IUserEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self
        {
            AppEvent::UserEvent(v) => write!(f, "{:?}", v),
            AppEvent::Window(v) => write!(f, "{:?}", v),
            AppEvent::State(v) => write!(f, "{:?}", v),
        }
    }
}
impl<U> From<WindowEvent> for AppEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }

