use super::*;

pub trait AppInit<A> : Fn() -> A + Async where A: AppMessageHandler {}
impl<S,A> AppInit<A> for S where S: Fn() -> A + Async, A: AppMessageHandler {}

pub trait AppMessageHandler: MessageHandler<AppMessage> + 'static {}
impl<S> AppMessageHandler for S where S: MessageHandler<AppMessage> + 'static {}

pub trait AppRun : Sized
{
    /// The app will be created when the graphics context will be available.
    /// This way, loading texture inside the function will work as normal.
    fn run<F,A>(init_app: F) -> AppResult
        where
        F: AppInit<A>,
        A: AppMessageHandler
    {
        Self::run_with_param(init_app, ___())
    }
    /// The app will be created when the graphics context will be available.
    /// This way, loading texture inside the function will work as normal.
    fn run_with_param<F,A>(init_app: F, param: AppParam) -> AppResult
        where
        F: AppInit<A>,
        A: AppMessageHandler;
}

/// Run the application and init the App
pub(crate) struct AppRunner<A,F>
    where
    F: AppInit<A>,
    A: AppMessageHandler
{
    init: F,
    // Some = all the context (graphic and more) are ready to be used
    app: Option<A>,
    is_running: bool,
    last_update : Time,
}
impl<A,F> AppRunner<A,F>
    where
    F: AppInit<A>,
    A: AppMessageHandler
{
    pub fn new(init: F) -> Self { Self { init, app: None, last_update: Time::since_launch(), is_running: false } }

    pub(crate) fn is_ready_to_run(&self) -> bool
    {
        Pen::is_init()
    }
    #[inline(always)]
    pub(crate) fn is_not_ready_to_run(&self) -> bool
    {
        !self.is_ready_to_run()
    }
}

impl AppRun for App
{
    fn run_with_param<F,A>(init_app: F, param: AppParam) -> AppResult
        where
        F: AppInit<A>,
        A: AppMessageHandler
    {
        // "Can't run two app at the same time"
        if App.already_init { return Err(AppError::AlreadyInit); }

        log::init_logger();

        // init panic
        {
            let default_hook = std::panic::take_hook();

            std::panic::set_hook(Box::new(move |info| {

                /*
                App.exit();

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

        App.init(param, proxy);

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(init_app);

        // Wrap the entire run in catch_unwind
        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = event_loop.run_app(&mut runner);
                //App.exit();
            }));

            App.exit();
            return result.map_err(|e| AppError::Panics(e));
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            return Ok(())
        }
    }
}
/*
impl<A> AppRun for A where A: EventHandler<AppMessage>
{
    fn run_with_param(self, param: AppParam) -> Result<(), ()>
    {
        // "Can't run two app at the same time"
        if App.already_init { return Err(()); }

        log::init_logger();

        // init panic
        {
            let default_hook = std::panic::take_hook();

            std::panic::set_hook(Box::new(move |info| {

                /*
                App.exit();

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

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        App.init(param, proxy);

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self);

        // Wrap the entire run in catch_unwind
        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _ = event_loop.run_app(&mut runner);
                //App.exit();
            }));

            App.exit();

            result.ok_or_void()
        }


        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     let r = event_loop.run_app(&mut runner);
        //     r.ok_or_void()
        // }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}
*/

impl<A,F> Application for AppRunner<A,F>
    where
    F: AppInit<A>,
    A: AppMessageHandler + 'static
{
    fn resumed(&mut self)
    {
        self.is_running = true;

        match self.app.as_mut()
        {
            Some(app) => App.scoped_flow(FlowMessage::Resumed, app),
            None => App.scoped_flow_action(FlowMessage::Resumed, |_|{}),
        }
    }

    fn paused(&mut self)
    {
        self.is_running = false;
        match self.app.as_mut()
        {
            Some(app) => App.scoped_flow(FlowMessage::Paused, app),
            None => App.scoped_flow_action(FlowMessage::Paused, |_|{}),
        }
    }

    fn draw(&mut self)
    {
        match self.app.as_mut()
        {
            Some(app) => App.scoped_flow(FlowMessage::Draw, app),
            None => App.scoped_flow_action(FlowMessage::Draw, |_|{}),
        }
    }

    fn update(&mut self, dt: DeltaTime)
    {
        match self.app.as_mut()
        {
            Some(app) => App.scoped_flow(FlowMessage::Update(dt), app),
            None => App.scoped_flow_action(FlowMessage::Update(dt), |_|{}),
        }
    }

    fn event(&mut self, ev: AppEvent)
    {
        match &ev
        {
            AppEvent::Input(input) =>
            {
                match input
                {
                    InputEvent::Key(k) =>
                    {
                        App.input.keyboard.key_event(*k);
                    },
                }
            },
            AppEvent::Window(window) => match window
            {
                WindowEvent::Resize(size) =>
                {
                    if Pen::is_init()
                    {
                        Pen.resize(*size);
                    }
                    Window.request_draw();
                },
                WindowEvent::Destroy => App.window.destroy(),
                WindowEvent::Draw => self.draw(),
                _ => {},
            },
            _ => {}
        }
        if let Some(app) = self.app.as_mut()
        {
            app.message(ev.into());
        }
    }
}

impl<A,F> winit::application::ApplicationHandler<AppInternalEvent> for AppRunner<A,F>
    where
    F: AppInit<A>,
    A: AppMessageHandler
{
    fn resumed(&mut self, active: &EventLoopActive)
    {
        App.window.begin_resumed_with_active_loop(active);
        Application::resumed(self);
    }

    fn suspended(&mut self, active: &EventLoopActive) {
        Application::paused(self);
    }

    fn about_to_wait(&mut self, active: &EventLoopActive)
    {
        let time = Time::since_launch();
        let dt = time - self.last_update;
        self.last_update = time;
        Application::update(self, dt);
    }

    fn exiting(&mut self, event_loop: &EventLoopActive) {
        let _ = App.exit();
    }

    fn new_events(&mut self, event_loop: &EventLoopActive, cause: winit::event::StartCause) {
        // FIXME: The draw() should not be here, or limit it to a fixed fps
        App.window.request_draw();
    }

    fn window_event(
        &mut self,
        event_loop: &EventLoopActive,
        window_id: WinitWindowID,
        event: winit::event::WindowEvent,
    ) {
        match event
        {
            WinitWindowEvent::Resized(physical_size) => Application::event(self, WindowEvent::Resize(physical_size.convert()).into()),
            winit::event::WindowEvent::CloseRequested =>
            {
                Application::event(self, WindowEvent::Close.into());
                event_loop.exit();
            },
            winit::event::WindowEvent::Destroyed => Application::event(self, WindowEvent::Destroy.into()),
            WinitWindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>
            {
                let code = KeyCode::from(event.physical_key);
                let repeat = if event.repeat { ButtonRepeat::Repeated } else { ButtonRepeat::NotRepeated };
                let state = if event.state.is_pressed() { ButtonState::Down } else { ButtonState::Up };

                if code == KeyCode::Escape // TODO make it debug/cfg/option<Binding> to force exit
                {
                    event_loop.exit();
                }
                let char: Option<char> = match &event.logical_key {
                    winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
                    _ => None,
                };
                let key = KeyEvent{ code, repeat, state, char };
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
            winit::event::WindowEvent::RedrawRequested => Application::event(self, WindowEvent::Draw.into()),
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &EventLoopActive, event: AppInternalEvent) {
        match event
        {
            AppInternalEvent::Gpu(gpu) =>
            {
                App.graphics = Some(gpu.unwrap());
                //let _ = Asset::<Texture>::manager().hot_reload();
                App.window.request_draw();
            },
            AppInternalEvent::Exit =>
            {
                event_loop.exit();
                let _ = App.exit();
            }
        }
    }
}
