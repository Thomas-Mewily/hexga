use super::*;

pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
pub(crate) type EventLoop<T=()> = winit::event_loop::EventLoop<T>;
pub(crate) type EventLoopProxy<T=CtxEvent> = winit::event_loop::EventLoopProxy<T>;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitWindowID = winit::window::WindowId;


mod event;
pub use event::*;
mod app;
pub use app::*;
mod runner;
pub use runner::*;
mod futur;
pub use futur::*;
mod scoped;
pub use scoped::*;