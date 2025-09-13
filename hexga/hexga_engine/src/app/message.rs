use super::*;


pub(crate) enum AppInternalMessage<U> where U: IUserEvent
{
    Message(AppMessage<U>),
    ContextGpu(Result<ContextGpu,String>),
}

pub enum AppMessage<U> where U: IUserEvent
{
    UserEvent(U),
    Update(DeltaTime),
    Draw,
}


pub trait IUserEvent : 'static + Debug + Send {}
impl<T> IUserEvent for T where T: 'static + Debug + Send {}