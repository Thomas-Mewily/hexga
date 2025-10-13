use super::*;




pub trait AppRun : Sized
{
    fn run(self) -> Result<(), ()> { self.run_with_param(___()) }
    fn run_with_param(self, param: AppParam) -> Result<(), ()>;
}


/// Run the application and init the App
pub(crate) struct AppRunner<A> where A:Application
{
    app: A,
}
impl<A> AppRunner<A> where A:Application
{
    pub const fn new(app: A) -> Self { Self { app } }

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


impl<A> AppRun for A where A:Application
{
    fn run_with_param(self, param: AppParam) -> Result<(), ()>
    {
        log::init_logger();


        assert!(App::is_not_init(), "Can't run two app at the same time, App is a singleton");

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        App::replace(Some(AppCore::new(param, proxy)));

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let r = event_loop.run_app(&mut runner);
            r.ok_or_void()
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}


impl<A> Application for AppRunner<A> where A:Application
{
    fn resumed(&mut self)
    {
        App.scoped_flow(FlowMessage::Resumed, |_| self.app.resumed());
    }

    fn paused(&mut self)
    {
        App.scoped_flow(FlowMessage::Paused, |_| self.app.paused());
    }

    fn draw(&mut self)
    {
        if self.is_not_ready_to_run() { return; }
        App.scoped_flow(FlowMessage::Draw, |_| self.app.draw());
    }

    fn update(&mut self)
    {
        App.scoped_flow(FlowMessage::Update, |_| self.app.update());
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
        self.app.event(ev);
    }
}

impl<A> winit::application::ApplicationHandler<AppInternalEvent> for AppRunner<A> where A:Application
{
    fn resumed(&mut self, active: &EventLoopActive)
    {
        App.window.begin_resumed_with_active_loop(active);
        Application::resumed(self);
    }

    fn suspended(&mut self, active: &EventLoopActive) {
        Application::paused(self);
    }

    fn about_to_wait(&mut self, active: &EventLoopActive) {
        Application::update(self);
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        App::destroy();
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

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: AppInternalEvent) {
        match event
        {
            AppInternalEvent::Gpu(gpu) =>
            {
                App.pen = Some(gpu.unwrap());
                App.window.request_draw();
            },
        }
    }
}
