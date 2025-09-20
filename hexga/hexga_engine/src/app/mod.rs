use super::*;

pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
pub(crate) type EventLoop<T=()> = winit::event_loop::EventLoop<T>;
pub(crate) type EventLoopProxy<E=()> = winit::event_loop::EventLoopProxy<AppInternalEvent<E>>;




mod event;
pub use event::*;

mod application;
pub use application::*;

mod app_ctx;
pub use app_ctx::*;

mod app;
pub use app::*;

mod runner;
pub use runner::*;

mod futur;
pub use futur::*;

mod scoped;
pub use scoped::*;