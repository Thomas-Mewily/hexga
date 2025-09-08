use super::*;

pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

mod app;
pub use app::*;

mod message;
pub use message::*;

mod futur;
pub use futur::*;

mod runner;
pub use runner::*;




pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::message::prelude::*;
    pub use super::futur::prelude::*;
    pub use super::runner::prelude::*;
}