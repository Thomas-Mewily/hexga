use super::*;


pub mod prelude
{
    pub use super::AppMessage;
    pub(crate) use super::{AppInternalMessage, IUserEvent};
}

pub(crate) enum AppInternalMessage<U> where U: IUserEvent
{
    Message(AppMessage<U>),
    ContextGpu(Result<ContextGpu,String>),
}

pub enum AppMessage<U> where U: IUserEvent
{
    UserEvent(U)
}


pub trait IUserEvent : 'static + Debug + Send {}
impl<T> IUserEvent for T where T: 'static + Debug + Send {}