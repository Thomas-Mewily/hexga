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
    pub use super::
    {
        application::Application,
        event::AppEvent,
        spawn::AsyncSpawn,
        param::AppParam,
        app_core::{App},
        result::*,
    };

    pub(crate) use super::{FlowMessage,AppInternalEvent};

    pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type EventLoop = winit::event_loop::EventLoop<AppInternalEvent>;
    pub(crate) type EventLoopProxy = winit::event_loop::EventLoopProxy<AppInternalEvent>;

    pub(crate) type WinitWindowEvent = winit::event::WindowEvent;
}