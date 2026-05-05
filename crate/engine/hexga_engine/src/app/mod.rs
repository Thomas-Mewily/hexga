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


pub mod prelude
{
    pub use super::{App,AppRunner,AppCtx,AsyncSpawn};

    pub(crate) use super::{AppInternalEvent,application::experimental::AppContext};
    pub(crate) type WinitEventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type WinitEventLoop = winit::event_loop::EventLoop<AppInternalEvent>;
    pub(crate) type WinitEventLoopProxy = winit::event_loop::EventLoopProxy<AppInternalEvent>;

    pub(crate) type WinitWindowEvent = winit::event::WindowEvent;
}







pub type AppResult = Result;


