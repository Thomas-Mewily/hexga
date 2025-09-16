use super::*;

pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
pub(crate) type EventLoop<T=()> = winit::event_loop::EventLoop<T>;
pub(crate) type EventLoopProxy<T=()> = winit::event_loop::EventLoopProxy<T>;

mod event;
pub use event::*;
mod app;
pub use app::*;
mod runner;
pub use runner::*;
mod futur;
pub use futur::*;