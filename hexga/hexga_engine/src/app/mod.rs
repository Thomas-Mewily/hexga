use super::*;

mod event;
pub use event::*;

mod application;
pub use application::*;

mod app_core;
pub use app_core::*;

mod spawn;
pub use spawn::*;

mod runner;
pub use runner::*;

mod result;
pub use result::*;

mod param;
pub use param::*;

pub mod prelude
{
    pub use super::{
        app_core::app, application::Application, event::AppEvent, param::AppParam, result::*,
        spawn::AsyncSpawn,
    };

    pub(crate) use super::app_core::APP;
    pub(crate) use super::{AppInternalEvent, FlowMessage};

    pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type EventLoop = winit::event_loop::EventLoop<AppInternalEvent>;
    pub(crate) type EventLoopProxy = winit::event_loop::EventLoopProxy<AppInternalEvent>;

    pub(crate) type WinitWindowEvent = winit::event::WindowEvent;
}
