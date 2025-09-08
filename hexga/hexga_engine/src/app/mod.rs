use super::*;

pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

mod app;
mod message;
mod futur;
mod runner;

pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::message::prelude::*;
    pub use super::futur::prelude::*;
    pub use super::runner::prelude::*;
}