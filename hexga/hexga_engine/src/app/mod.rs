use super::*;

pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

mod message;
pub use message::*;

mod futur;
pub use futur::*;

mod runner;
pub use runner::*;


pub trait App : 'static
{
    type UserEvent : IUserEvent;

    fn update(&mut self) {}
    fn draw(&mut self) {}
}

pub mod prelude
{
    pub use super::futur::prelude::*;
    pub use super::{IUserEvent,App,AppRun};
}