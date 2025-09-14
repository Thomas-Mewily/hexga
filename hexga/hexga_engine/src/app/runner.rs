use super::*;


pub trait AppRun
{
    fn run(self) -> Result<(), ()>;
}
impl<A> AppRun for A where A:App
{
    fn run(self) -> Result<(), ()> 
    {
        Ctx::init_default();
        let ctx = Ctx::try_as_mut().ok_or_void()?;

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        let proxy = event_loop.create_proxy();

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self, ctx, proxy);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                event_loop.run_app(&mut runner)
            }));

            Ctx::destroy();

            if let Ok(Ok(_)) = result
            {
                Ok(())
            }else
            {
                Err(())
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}



pub(crate) struct AppRunner<A> where A:App
{
    app : A,
    ctx : &'static mut Context,
    last_update : Time,
    proxy : EvLoopProxy<A::UserEvent>,
}
impl<A> AppRunner<A> where A:App
{
    pub fn new(app : A, ctx : &'static mut Context, proxy : EvLoopProxy<A::UserEvent>) -> Self { Self { app, ctx, proxy, last_update: Time::since_launch() }}
}

impl<A> winit::application::ApplicationHandler<AppInternalEvent<A::UserEvent>> for AppRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &EventLoopActive) 
    {
        if self.ctx.window.is_none() 
        {
            #[allow(unused_mut)]
            let mut win_attr = WinitWindow::default_attributes().with_title("wgpu winit example");
            
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.ctx.window = Some(window.clone());
            ContextGpu::request(window, self.proxy.clone()).unwrap();
            Ctx.resumed();
        }
    }

    fn user_event(&mut self, event_loop: &EventLoopActive, event: AppInternalEvent<A::UserEvent>) {
        match event
        {
            AppInternalEvent::Event(app_message) => {},
            AppInternalEvent::ContextGpu(context_wgpu) => 
            {
                Gpu::replace(Some(context_wgpu.unwrap()));
                self.ctx.window.as_ref().map(|w| w.request_redraw());
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &EventLoopActive,
        window_id: WinitWindowID,
        event: WinitWindowEvent,
    ) 
    {
        if !Gpu::is_init() { return; }


        match event 
        {
            WinitWindowEvent::CloseRequested =>  { event_loop.exit(); }
            WinitWindowEvent::Resized(new_size) => {
                if let Some(window) = self.ctx.window.as_ref()
                {
                    Gpu.resize([new_size.width as _, new_size.height as _].into());
                    window.request_redraw();
                }
            }
            WinitWindowEvent::RedrawRequested => self.draw(),
            WinitWindowEvent::KeyboardInput { device_id, event, is_synthetic } => 
            {
                let code = KeyCode::from(event.physical_key);
                let repeat = if event.repeat { ButtonRepeat::Repeated } else { ButtonRepeat::NotRepeated };
                let state = if event.state.is_pressed() { ButtonState::Down } else { ButtonState::Up };
                if code == KeyCode::Escape
                {
                    event_loop.exit();
                }

                let char: Option<char> = match &event.logical_key {
                    winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
                    _ => None,
                };
                let key = KeyEvent{ code, repeat, state, char };
                
                Input.keyboard.handle_key_event(key);
                self.app.handle_event(AppEvent::Key(key));
                
                //self.app.handle_message()
            }
            _ => (),
        }
    }

    fn new_events(&mut self, event_loop: &EventLoopActive, cause: WinitStartCause) {
        // FIXME: The draw() should not be here
        Ctx.window.as_mut().map(|window| window.request_redraw());
    }

    fn exiting(&mut self, event_loop: &EventLoopActive) 
    {
        Ctx::destroy();
    }

    fn suspended(&mut self, event_loop: &EventLoopActive) {
        Ctx.suspended();
    }

    fn about_to_wait(&mut self, event_loop: &EventLoopActive) {
        self.update();
        // FIXME: The draw() should not be here
        //self.draw();
    }
}



impl<A> AppRunner<A> where A:App
{
    pub fn update(&mut self)
    {
        Ctx.scoped_update
        (
            || 
            { 
                let time = Time::since_launch();
                let delta_time = time - self.last_update;
                self.last_update = time;
                self.app.handle_event(AppEvent::Update(delta_time));
            }
        );
    }

    pub fn draw(&mut self)
    {
        Ctx.scoped_draw(
            ScopedDrawParam{ window_size: Ctx.window_size() },
            || 
            { 
                self.app.handle_event(AppEvent::Draw); 
            }
        );
    }
}
