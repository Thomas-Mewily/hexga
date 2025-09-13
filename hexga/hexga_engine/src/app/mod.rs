use super::*;

use winit::{
    application::ApplicationHandler,
    event::*,
};


pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitKeyCode = winit::keyboard::KeyCode;
pub(crate) type WinitKeyPhysical = winit::keyboard::PhysicalKey;
pub(crate) type WinitKeyNativeCode = winit::keyboard::NativeKeyCode;
pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
pub(crate) type EventLoop<T> = winit::event_loop::EventLoop<T>;
pub(crate) type EventLoopProxy<T> = winit::event_loop::EventLoopProxy<T>;


mod message;
pub use message::*;

mod futur;
pub use futur::*;

mod runner;
pub use runner::*;



pub trait App : 'static
{
    type UserEvent : IUserEvent;

    fn handle_message(&mut self, message: AppMessage<Self::UserEvent>) { self.dispatch_message(message); }
    fn dispatch_message(&mut self, message: AppMessage<Self::UserEvent>)
    {
        match message
        {
            AppMessage::UserEvent(msg) => self.user_message(msg),
            AppMessage::Update(dt) => self.update(dt),
            AppMessage::Draw => self.draw(),
        }
    }

    fn user_message(&mut self, msg: Self::UserEvent) { let _ = msg; }
    fn update(&mut self, dt: DeltaTime) { let _ = dt; }
    fn draw(&mut self) {}
}