use super::*;

mod event;
pub use event::*;

mod application;
pub use application::*;

mod app_core;
pub use app_core::*;

mod futur;
pub use futur::*;

mod runner;
pub use runner::*;

mod scoped;
pub use scoped::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::
    {
        application::Application,
        event::AppEvent,
        futur::SpawnFutur,
        runner::AppRun,
        app_core::{App,AppParam},
        result::*,
    };

    pub(crate) use super::scoped::*;
    pub(crate) use super::{FlowMessage,AppInternalEvent};

    pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type EventLoop = winit::event_loop::EventLoop<AppInternalEvent>;
    pub(crate) type EventLoopProxy = winit::event_loop::EventLoopProxy<AppInternalEvent>;

    pub(crate) type WinitWindowEvent = winit::event::WindowEvent;
}