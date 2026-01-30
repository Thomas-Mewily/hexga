use super::*;

pub trait AppInit<A>: FnOnce() -> A + Async {}
impl<S, A> AppInit<A> for S where S: FnOnce() -> A + Async {}

pub trait AppMessageHandler: MessageHandler<AppMessage> + 'static {}
impl<S> AppMessageHandler for S where S: MessageHandler<AppMessage> + 'static {}

/// Run the application and init the App
pub(crate) struct AppRunner<A, F>
where
    F: AppInit<A>,
    A: AppMessageHandler,
{
    app: LazyFnOnceValue<A, F>,
    //is_running: bool,
    last_update: Time,
}
impl<A, F> AppRunner<A, F>
where
    F: AppInit<A>,
    A: AppMessageHandler,
{
    pub(crate) fn new(init: F) -> Self
    {
        Self {
            app: LazyFnOnceValue::new(init),
            last_update: Time::since_launch(),
        }
    }

    #[inline(always)]
    pub(crate) fn is_ready_to_run(&self) -> bool { app().graphics.is_some() }
    #[inline(always)]
    pub(crate) fn is_not_ready_to_run(&self) -> bool { !self.is_ready_to_run() }
    pub(crate) fn force_app_mut(&mut self) -> &mut A { self.app.as_mut() }
    pub(crate) fn app_mut(&mut self) -> Option<&mut A>
    {
        if self.is_ready_to_run()
        {
            Some(self.force_app_mut())
        }
        else
        {
            None
        }
    }
}

impl<A, F, P> Runner<F, P> for AppRunner<A, F>
where
    A: AppMessageHandler,
    F: AppInit<A>,
    P: Into<AppParamInternal>,
{
    type Output = AppResult;

    fn run_with_param(init_app: F, param: P) -> Self::Output
    {
        // Can't run two app at the same time
        let mut app = app();
        if app.already_init
        {
            return Err(AppError::AlreadyInit);
        }

        log::init();

        // init panic
        {
            let default_hook = std::panic::take_hook();

            std::panic::set_hook(Box::new(move |info| {
                /*
                #[cfg(not(target_arch = "wasm32"))]
                {
                    eprintln!("panic occurred: {info}");
                }
                */

                #[cfg(target_arch = "wasm32")]
                {
                    // Use the console_error_panic_hook for WASM
                    console_error_panic_hook::hook(info);
                }

                default_hook(info);
            }));
        }

        let event_loop = EventLoop::with_user_event().build()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        app.init(param.into(), proxy);
        drop(app);

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(init_app);

        // Wrap the entire run in catch_unwind
        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = event_loop.run_app(&mut runner);
            }));

            runner.exit();
            return result.map_err(|e| AppError::Panics(e));
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move {
                let _ = event_loop.run_app(&mut runner);
            }
            .spawn();
            return Ok(());
        }
    }
}

impl<A, F> AppRunner<A, F>
where
    F: AppInit<A>,
    A: AppMessageHandler,
{
    pub(crate) fn exit(&mut self)
    {
        self.event(AppEvent::Window(WindowEvent::Destroy));
        app().exit();
    }

    pub(crate) fn app_event(&mut self, ev: AppEvent) { self.app_message(AppMessage::Event(ev)); }
    pub(crate) fn app_message(&mut self, msg: AppMessage)
    {
        self.app_mut().map(|a| a.message(msg));
    }
}

impl<A, F> Application for AppRunner<A, F>
where
    F: AppInit<A>,
    A: AppMessageHandler,
{
    fn event(&mut self, ev: AppEvent)
    {
        match &ev
        {
            AppEvent::Input(input) => match input
            {
                InputEvent::Key(k) =>
                {
                    app().input().keyboard().key_event(*k);
                    self.app_event(ev);
                }
            },
            AppEvent::Window(window) => match window
            {
                WindowEvent::Resize(size) =>
                {
                    app().window().configure_surface();
                    self.app_event(ev);
                }
                WindowEvent::Move(pos) =>
                {
                    app().window().set_pos(*pos);
                    self.app_event(ev);
                }
                WindowEvent::Open => self.app_event(ev),
                WindowEvent::Close => self.app_event(ev),
                WindowEvent::Destroy =>
                {
                    self.app_event(ev);
                    app().window().destroy();
                }
                WindowEvent::Draw =>
                {
                    app().window().request_draw();
                    self.app_event(ev);
                }
            },
        }
    }
}

impl<A, F> winit::application::ApplicationHandler<AppInternalEvent> for AppRunner<A, F>
where
    F: AppInit<A>,
    A: AppMessageHandler,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop)
    {
        let mut app = app();
        if app.window().init_if_needed(event_loop)
        {
            if app.graphics.is_none()
            {
                AppGraphics::init(
                    app.window().main_window().window.clone(),
                    app.param.gpu.take().unwrap_or_default(),
                    app.proxy().clone(),
                )
                .expect("failed to init the gpu");
            }
            drop(app);
            self.event(AppEvent::Window(WindowEvent::Open));
        }
        self.app_mut()
            .map(|a| a.message(AppMessage::Flow(FlowMessage::Resumed)));
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop)
    {
        match app().window().try_main_window_mut()
        {
            Some(w) => w.surface = None,
            None =>
            {}
        }
        self.app_mut()
            .map(|a| a.message(AppMessage::Flow(FlowMessage::Suspended)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event
        {
            WinitWindowEvent::Resized(physical_size) =>
            {
                Application::event(self, WindowEvent::Resize(physical_size.convert()).into())
            }
            winit::event::WindowEvent::CloseRequested =>
            {
                Application::event(self, WindowEvent::Close.into());
                event_loop.exit();
            }
            winit::event::WindowEvent::Destroyed =>
            {
                Application::event(self, WindowEvent::Destroy.into())
            }
            WinitWindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } =>
            {
                let code = KeyCode::from(event.physical_key);
                let repeat = if event.repeat
                {
                    ButtonRepeat::Repeated
                }
                else
                {
                    ButtonRepeat::NotRepeated
                };
                let state = if event.state.is_pressed()
                {
                    ButtonState::Down
                }
                else
                {
                    ButtonState::Up
                };

                if code == KeyCode::Escape
                // TODO make it debug/cfg/option<Binding> to force exit
                {
                    event_loop.exit();
                }
                let char: Option<char> = match &event.logical_key
                {
                    winit::keyboard::Key::Character(s) if s.chars().count() == 1 =>
                    {
                        s.chars().next()
                    }
                    _ => None,
                };
                let key = KeyEvent {
                    code,
                    repeat,
                    state,
                    char,
                };
                Application::event(self, AppEvent::Input(InputEvent::Key(key)))
            }
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
            winit::event::WindowEvent::RedrawRequested =>
            {
                Application::event(self, WindowEvent::Draw.into())
            }
            _ => (),
        }
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) { self.exit(); }

    fn new_events(&mut self, event_loop: &EventLoopActive, cause: winit::event::StartCause)
    {
        // FIXME: The draw() should not be here, or limit it to a fixed fps
        app().window.request_draw();
    }

    fn user_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: AppInternalEvent,
    )
    {
        let mut app = app();
        match event
        {
            AppInternalEvent::Gpu(app_graphics) =>
            {
                app.graphics = Some(app_graphics.expect("failed to init the gpu"));
                app.window.init_surface_if_needed();
            }
        }
    }
}
