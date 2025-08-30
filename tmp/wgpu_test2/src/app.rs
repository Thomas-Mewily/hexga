use super::*;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};


pub trait IUserEvent : 'static
{

}
impl IUserEvent for () {}

pub trait App 
{
    type UserEvent : IUserEvent;
}

pub trait AppRun
{
    fn run(self) -> Result<(), ()>;
}
impl<A> AppRun for A where A:App
{
    fn run(self) -> Result<(), ()> 
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
        #[cfg(target_arch = "wasm32")]
        {
            console_log::init_with_level(log::Level::Info).ok_or_void()?;
        }

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;

        let proxy = event_loop.create_proxy();

        event_loop.run_app(&mut AppRunner::new(self, proxy)).ok_or_void()
    }
}

pub enum AppMessage<U> where U: IUserEvent
{
    UserEvent(U)
}

pub type EvLoopProxy<U> = EventLoopProxy<AppMessage<U>>;
pub type EvLoop<U> = EventLoop<AppMessage<U>> ;

pub(crate) struct AppRunner<A> where A:App
{
    app : A,
    proxy : EvLoopProxy<A::UserEvent>
}
impl<A> AppRunner<A> where A:App
{
    pub fn new(app : A, event_loop : EvLoopProxy<A::UserEvent>) -> Self { Self { app, proxy: event_loop }}
}

impl<A> ApplicationHandler<AppMessage<A::UserEvent>> for AppRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) 
    {

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) 
    {
        
    }
}
