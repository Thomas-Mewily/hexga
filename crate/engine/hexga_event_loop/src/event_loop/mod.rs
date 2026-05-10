use super::*;

mod event_handler;
pub use event_handler::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

mod spawn;
pub use spawn::*;

mod event_loop;
pub use event_loop::*;

pub mod experimental
{
    use super::*;

    pub type WinitEventLoopProxy<CustomEvent> = ::winit::event_loop::EventLoopProxy<PlatformEvent<CustomEvent>>;

    pub(crate) type WinitEventLoopActive = ::winit::event_loop::ActiveEventLoop;
    pub(crate) type WinitEventLoop<User> = ::winit::event_loop::EventLoop<User>;
    pub(crate) type WinitWindowEvent = ::winit::event::WindowEvent;
}


pub mod prelude
{
    pub use super::{
        EventLoop,PlatformEvent,
    };
    //pub use super::{HasMutWindow,HasMutKeyboard,HasMutClipboard,HasMutGraphics};
    //pub(crate) use super::{AppDefaultUserEventInner};
    pub(crate) use super::experimental::*;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{
        PlatformEventHandler,PlatformEventHandlerExtension,
        PlatformCustomEvent,EventLoopSendEvent,
        AsyncSpawn,
    };
}

pub type EventLoopResult<T=()> = Result<T>;