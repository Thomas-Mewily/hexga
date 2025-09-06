use super::*;


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
impl IUserEvent for () {}