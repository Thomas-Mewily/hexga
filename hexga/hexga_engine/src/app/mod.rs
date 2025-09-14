use super::*;

pub(crate) type WinitKeyEvent = winit::event::KeyEvent;


pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalEvent<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppEvent<U>> ;

pub(crate) type WinitWindow = winit::window::Window;
pub(crate) type WinitKeyCode = winit::keyboard::KeyCode;
pub(crate) type WinitKeyPhysical = winit::keyboard::PhysicalKey;
pub(crate) type WinitKeyNativeCode = winit::keyboard::NativeKeyCode;
pub(crate) type EventLoopActive = winit::event_loop::ActiveEventLoop;
pub(crate) type EventLoop<T> = winit::event_loop::EventLoop<T>;
pub(crate) type EventLoopProxy<T> = winit::event_loop::EventLoopProxy<T>;
pub(crate) type WinitWindowEvent = winit::event::WindowEvent;
pub(crate) type WinitWindowID = winit::window::WindowId;
pub(crate) type WinitStartCause = winit::event::StartCause;


mod event;
pub use event::*;

mod futur;
pub use futur::*;

mod runner;
pub use runner::*;



pub trait App : 'static
{
    type UserEvent : IUserEvent;

    fn handle_event(&mut self, event: AppEvent<Self::UserEvent>) { self.dispatch_event(event); }
    fn dispatch_event(&mut self, event: AppEvent<Self::UserEvent>)
    {
        match event
        {
            AppEvent::User(msg) => self.event_user(msg),
            AppEvent::Update(dt) => self.update(dt),
            AppEvent::Draw => self.draw(),
            AppEvent::Key(key_event) => self.event_key(key_event),
            AppEvent::Unknow => {},
        }
    }

    fn event_key(&mut self, key_event:KeyEvent) { let _ = key_event; }
    fn event_user(&mut self, msg: Self::UserEvent) { let _ = msg; }

    fn update(&mut self, dt: DeltaTime) { let _ = dt; }
    fn draw(&mut self) {}
}