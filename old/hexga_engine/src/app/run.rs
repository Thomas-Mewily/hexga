use super::*;



impl<T, UserEvent> AppRun<UserEvent> for T where T: App<UserEvent>, UserEvent: IUserEvent {}
pub trait AppRun<UserEvent> : App<UserEvent> where UserEvent: IUserEvent, Self: Sized
{
    fn run(self) { self.run_with_param(___()); }

    fn run_with_param(self, mut param: AppParam)
    {
        init_ctx_if_needed();
        let ctx = ctx_mut();

        ctx.windows.init_main_window(std::mem::take(&mut param.window));

        let event_loop = winit::event_loop::EventLoop::<AppInternalEvent<UserEvent>>::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        AppRunner::new(self, proxy).run_with_param(event_loop, param);
    }
}

pub(crate) struct AppRunner<A,UserEvent=()> where A: App<UserEvent>, UserEvent:IUserEvent
{
    app : A,
    proxy : EventLoopProxy<UserEvent>,
}

impl <A,UserEvent> AppRunner<A,UserEvent> where A:App<UserEvent>, UserEvent:IUserEvent
{
    fn new(app : A, proxy : EventLoopProxy<UserEvent>) -> Self { Self { app, proxy }}

    fn run(self, event_loop : WinitEventLoop<AppInternalEvent<UserEvent>>) { self.run_with_param(event_loop, ___()) }
    
    fn run_with_param(#[allow(unused_mut)] mut self, event_loop : WinitEventLoop<AppInternalEvent<UserEvent>>, _param : AppParam)
    {
        init_logger_if_needed();

        #[cfg(target_arch = "wasm32")]
        {
            // Runs the app async via the browsers event loop
            use winit::platform::web::EventLoopExtWebSys;
            wasm_bindgen_futures::spawn_local(async move {
                event_loop.spawn_app(self);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Runs the app on the current thread.
            let _ = event_loop.run_app(&mut self);
        }

        unreachable!()
    }

    //fn run_with_param_and_ctx(...)

    fn send_event(&mut self, event: impl Into<AppInternalEvent<UserEvent>>)
    {
        let r = self.proxy.send_event(event.into());
        assert!(r.is_ok(), "Failed to send event");
    }
    fn send_window_event(&mut self, id : WindowID, event: impl Into<WindowEventKind>)
    {
        self.send_event(WindowEvent::new(id, event.into()));
    }
    fn send_internal_window_event(&mut self, id : WindowID, event: impl Into<WindowInternalEventKind>)
    {
        self.send_event(WindowInternalEvent::new(id, event.into()));
    }
    fn send_state_event(&mut self, state : StateEvent){
        self.send_event(state)
    }
}

#[allow(unused_variables)]
impl<A,UserEvent> WinitApp<AppInternalEvent<UserEvent>> for AppRunner<A,UserEvent> where A: App<UserEvent>, UserEvent:IUserEvent
{
    fn resumed(&mut self, event_loop: &WinitActiveEventLoop) {
        Ctx.resumed();
        self.send_state_event(StateEvent::Resumed);
    }

    fn suspended(&mut self, event_loop: &WinitActiveEventLoop) {
        Ctx.paused();
        self.send_state_event(StateEvent::Paused);
    }

    fn about_to_wait(&mut self, event_loop: &WinitActiveEventLoop) 
    {
        Ctx.begin_update(event_loop, &self.proxy);
        self.app.update();
        Ctx.end_update(&event_loop, &self.proxy);

        Ctx.begin_draw(event_loop, &self.proxy);
        self.app.draw();
        Ctx.end_draw(&event_loop, &self.proxy);
    }

    fn window_event(
        &mut self,
        event_loop: &WinitActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) 
    {

    }
}