use super::*;

mod param;
pub use param::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

pub use hexga_event_loop::app::{App, AppRunRaw, AppSendEvent, AsyncSpawn, IAppEvent, WithAppParam, WithAppShortcut};
pub use hexga_event_loop::app::{AppControlFlow, AppError, AppEvent, AppFlow, AppMessage, AppProxy, AppResult, AppShortcut, EventLoop, EventLoopParam};

pub mod prelude
{
    pub(crate) use super::event::{AppCustomEvent, AppInternalEventLoop, AppInternalMessage, AppInternalProxy};
    pub use super::param::AppParam;
    pub use super::runner::AppCtx;
    pub use super::traits::*;
    pub use super::{AppControlFlow, AppError, AppEvent, AppFlow, AppMessage, AppResult, AppShortcut, EventLoop};
}

pub mod traits
{
    pub use super::runner::AppRun;
    pub use hexga_event_loop::app::traits::*;
}
