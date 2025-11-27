use super::*;

mod application;
pub use application::*;

mod event;
pub use event::*;

mod runner;
pub use runner::*;

mod core;
pub use core::*;

mod futur;
pub use futur::*;

mod scoped;
pub(crate) use scoped::*;


pub mod prelude
{
    pub use super::application::*;
    pub use super::event::*;
    pub use super::runner::*;
    pub use super::core::*;
    pub use super::futur::*;

    pub(crate) use super::scoped::*;

    pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type EventLoop = winit::event_loop::EventLoop<AppInternalEvent>;
    pub(crate) type EventLoopProxy = winit::event_loop::EventLoopProxy<AppInternalEvent>;

    pub(crate) type WinitWindowEvent = winit::event::WindowEvent;

}
