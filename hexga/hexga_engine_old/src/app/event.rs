use super::*;


pub(crate) enum AppInternalEvent<U> where U: IUserEvent
{
    Event(AppEvent<U>),
    ContextGpu(Result<ContextGpu,String>),
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent<U> where U: IUserEvent
{
    User(U),
    Key(KeyEvent),
    Update(DeltaTime),
    Draw,
    Unknow,
}

impl<U> AppEvent<U> where U: IUserEvent
{
    pub fn is_unknow(&self) -> bool { matches!(self, AppEvent::Unknow) }
}

pub trait IUserEvent : 'static + Debug + Send {}
impl<T> IUserEvent for T where T: 'static + Debug + Send {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    pub code  : KeyCode,
    pub repeat: ButtonRepeat,
    pub state : ButtonState,
    pub char  : Option<char>,
}
impl Debug for KeyEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "KeyEvent {{ KeyCode::{:?}, ButtonState::{:?} {:?}{:?} }}", self.code, self.state, self.repeat.fmt_if_not_defaut(), self.char.fmt_if_not_defaut())
    }
}

