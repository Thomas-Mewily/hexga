use super::*;

mod param;
pub use param::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

pub use hexga_event_loop::app::{
    EventLoop, AppError, AppResult, 
    AppMessage, AppControlFlow, AppEvent, AppFlow,
    EventLoopParam, AppShortcut, AppProxy,
};
pub use hexga_event_loop::app::{AsyncSpawn, AppSendEvent, IAppEvent, App, AppRunRaw, WithAppParam, WithAppShortcut};


pub mod prelude
{
    pub use super::{
        EventLoop, AppError, AppResult, 
        AppMessage, AppControlFlow, AppEvent, AppFlow,
        AppShortcut,
    };
    pub use super::param::AppParam;
    pub use super::runner::AppCtx;
    pub(crate) use super::event::{AppCustomEvent, AppInternalEventLoop, AppInternalMessage, AppInternalProxy};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::runner::AppRun;
    pub use hexga_event_loop::app::traits::*;
}