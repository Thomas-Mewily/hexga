use super::*;

use winit::{
    application::ApplicationHandler,
    event::*,
};


pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

mod app;
mod message;
mod futur;
mod runner;

pub mod prelude
{
    pub(crate) type WinitWindow = winit::window::Window;
    pub(crate) type WinitKeyCode = winit::keyboard::KeyCode;
    pub(crate) type WinitKeyPhysical = winit::keyboard::PhysicalKey;
    pub(crate) type WinitKeyNativeCode = winit::keyboard::NativeKeyCode;
    pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
    pub(crate) type EventLoop<T> = winit::event_loop::EventLoop<T>;
    pub(crate) type EventLoopProxy<T> = winit::event_loop::EventLoopProxy<T>;

    pub use super::app::prelude::*;
    pub use super::message::prelude::*;
    pub use super::futur::prelude::*;
    pub use super::runner::prelude::*;
}