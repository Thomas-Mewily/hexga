use super::*;

mod param;
pub use param::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

pub use hexga_event_loop::app::{
    EventLoop, AppError, AppResult, 
    AppMessage, AppControlFlow, AppEvent, 
    EventLoopParam, AppShortcut, AppProxy,
};
pub use hexga_event_loop::app::{AsyncSpawn, AppSendEvent, IAppEvent, App, AppRunRaw, WithAppParam, WithAppShortcut};


pub mod prelude
{
    pub use super::{
        EventLoop, AppError, AppResult, 
        AppMessage, AppControlFlow, AppEvent, 
        AppShortcut,
        AppCtx, AppParam
    };
    pub(crate) use super::{AppInternalEventLoop,AppCustomEvent,AppInternalProxy, AppInternalEvent};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{AsyncSpawn, AppSendEvent, IAppEvent, App, WithAppParam, WithAppShortcut};
    pub use super::AppRun;
}