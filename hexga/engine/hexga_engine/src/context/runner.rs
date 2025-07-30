use winit::event_loop::{EventLoop, EventLoopProxy};

use super::*;


pub trait AppRun<UserEvent=()> : App<UserEvent> where UserEvent:IUserEvent
{
    fn run(self) where Self:Sized { self.run_with_param(___()); }
    fn run_with_param(self, mut param : AppParam) where Self:Sized
    {
        init_ctx_if_needed();
        let ctx = ctx_mut();

        ctx.window.init_main_window(std::mem::take(&mut param.window));

        let event_loop = winit::event_loop::EventLoop::<AppInternalEvent<UserEvent>>::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        AppRunner::new(self, proxy).run_with_param(event_loop, param);
    }
}
impl<T,UserEvent> AppRun<UserEvent> for T where T:App<UserEvent>, UserEvent:IUserEvent {}

pub(crate) struct AppRunner<A,UserEvent=()> where A: App<UserEvent>, UserEvent:IUserEvent
{
    app : A,
    proxy : EventLoopProxy<AppInternalEvent<UserEvent>>,
}


impl <A,UserEvent> AppRunner<A,UserEvent> where A:App<UserEvent>, UserEvent:IUserEvent
{
    fn new(app : A, proxy : EventLoopProxy<AppInternalEvent<UserEvent>>) -> Self { Self { app, proxy }}

    fn run(self, event_loop : EventLoop<AppInternalEvent<UserEvent>>) { self.run_with_param(event_loop, ___()) }
    fn run_with_param(mut self, event_loop : EventLoop<AppInternalEvent<UserEvent>>, _param : AppParam)
    {
        init_logger_if_needed();

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
}

pub(crate) type WinitActiveEventLoop = winit::event_loop::ActiveEventLoop;

impl<A,UserEvent> winit::application::ApplicationHandler<AppInternalEvent<UserEvent>> for AppRunner<A,UserEvent> where A: App<UserEvent>, UserEvent:IUserEvent
{
    fn resumed(&mut self, _event_loop: &WinitActiveEventLoop) {
        self.app.resume();
    }

    fn suspended(&mut self, _event_loop: &WinitActiveEventLoop) {
        self.app.pause();
    }

    fn window_event(
        &mut self,
        _event_loop: &WinitActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        let ctx = ctx_mut();
        let id = ctx.window.winit_id_to_window_id(window_id);

        match event
        {
            winit::event::WindowEvent::Resized(physical_size) => self.send_window_event(id, WindowEventKind::Resize(physical_size.convert())),
            winit::event::WindowEvent::Moved(physical_position) => self.send_window_event(id, WindowEventKind::Move(physical_position.convert())),
            winit::event::WindowEvent::CloseRequested => self.send_window_event(id, WindowEventKind::Close),
            winit::event::WindowEvent::Destroyed => self.send_window_event(id, WindowEventKind::Destroy),
            /*
            // TODO: interesting event to handle:
            winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            winit::event::WindowEvent::Focused(_) => todo!(),
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            winit::event::WindowEvent::ThemeChanged(theme) => todo!(),
            winit::event::WindowEvent::Occluded(_) => todo!()
            */
            winit::event::WindowEvent::RedrawRequested => self.send_window_event(id, WindowEventKind::Draw),
            _ => {},
        }

    }

    fn about_to_wait(&mut self, event_loop: &WinitActiveEventLoop)
    {
        Windows.update_dirty(&ctx_mut().graphics, event_loop, &self.proxy);
        self.app.update();
    }

    fn user_event(&mut self, _event_loop: &WinitActiveEventLoop, event: AppInternalEvent<UserEvent>)
    {
        match event
        {
            AppInternalEvent::UserEvent(e) => self.app.handle_event(AppEvent::UserEvent(e)),
            AppInternalEvent::Window(e) => Windows.handle_event(e),
            AppInternalEvent::WindowInternal(e) => Windows.handle_internal_event(e),
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop)
    {
        {
            // Just to make sure this drop() is call first, before the ctx
            self.app.exit();
        }
        reset_ctx();
    }
}