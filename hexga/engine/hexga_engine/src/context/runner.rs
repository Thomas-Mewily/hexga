use winit::event_loop::{EventLoop, EventLoopProxy};

use super::*;


pub trait AppRun<UserEvent=()> : App<UserEvent> where UserEvent:IUserEvent
{
    fn run(self) where Self:Sized
    {
        let r = init_ctx(Some(___()));
        assert!(r.is_none(), "Context already created");

        let event_loop = winit::event_loop::EventLoop::<UserEvent>::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        AppRunner::new(self, proxy).run(event_loop);
    }
}
impl<T,UserEvent> AppRun<UserEvent> for T where T:App<UserEvent>, UserEvent:IUserEvent {}

pub(crate) struct AppRunner<A,UserEvent=()> where A: App<UserEvent>, UserEvent:IUserEvent
{
    app : A,
    proxy : EventLoopProxy<UserEvent>,
}
impl <A,UserEvent> AppRunner<A,UserEvent> where A:App<UserEvent>, UserEvent:IUserEvent
{
    fn new(app : A, proxy : EventLoopProxy<UserEvent>) -> Self { Self { app, proxy }}

    fn run(mut self, event_loop : EventLoop<UserEvent>)
    {
        init_logger();

        #[cfg(target_arch = "wasm32")]
        {
            // Runs the app async via the browsers event loop
            use winit::platform::web::EventLoopExtWebSys;
            wasm_bindgen_futures::spawn_local(async move {
                event_loop.spawn_app(app);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Runs the app on the current thread.
            let _ = event_loop.run_app(&mut self);
        }
    }
}

impl<A,UserEvent> winit::application::ApplicationHandler<UserEvent> for AppRunner<A,UserEvent> where A: App<UserEvent>, UserEvent:IUserEvent
{
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.resume();
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.pause();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {

    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.update();
    }
}