use super::*;

mod application;
pub use application::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

mod param;
pub use param::*;

mod spawn;
pub use spawn::*;

mod context;
pub use context::*;

mod defaut_context;
pub use defaut_context::*;

mod event_loop;
pub use event_loop::*;

pub mod winit
{
    use super::*;

    pub type WinitEventLoopProxy<User> = ::winit::event_loop::EventLoopProxy<User>;

    pub(crate) type WinitEventLoopActive = ::winit::event_loop::ActiveEventLoop;
    pub(crate) type WinitEventLoop<User> = ::winit::event_loop::EventLoop<User>;
    pub(crate) type WinitWindowEvent = ::winit::event::WindowEvent;
}


pub mod prelude
{
    pub use super::{
        App,AppRun,AppRunRaw,AppCtx,AppEventLoop,AppUserEvent,AppSendEvent,
        AsyncSpawn,
        AppDefaultUserEvent, AppDefaultCtx
    };
    pub use super::{HasMutWindow,HasMutKeyboard,HasMutClipboard,HasMutGraphics};
    pub(crate) use super::{AppDefaultUserEventInner};
    pub(crate) use super::winit::*;
}







pub type AppResult<T=()> = Result<T>;


