use super::*;

pub trait AppRun
{
    fn run(self) -> Result<(), ()>;
}
impl<A> AppRun for A where A:App
{
    fn run(self) -> Result<(), ()> 
    {
        Ctx::init();
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
    pub fn new(app : A, ctx : &'static mut Context, proxy : EvLoopProxy<A::UserEvent>) -> Self { Self { app, ctx, proxy, last_update: Time::now() }}

    pub fn update(&mut self)
    {
        let time = Time::now();
        let delta_time = time - self.last_update;
        self.last_update = time;
        self.app.update(delta_time);
    }
}

impl<A> ApplicationHandler<AppInternalMessage<A::UserEvent>> for AppRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) 
    {
        if self.ctx.winit.is_none() 
        {
            #[allow(unused_mut)]
            let mut win_attr = Window::default_attributes().with_title("wgpu winit example");
            
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
            self.ctx.winit = Some(window.clone());
            ContextGpu::request(window, self.proxy.clone()).unwrap();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppInternalMessage<A::UserEvent>) {
        match event
        {
            AppInternalMessage::Message(app_message) => {},
            AppInternalMessage::ContextGpu(context_wgpu) => 
            {
                Gpu::replace(Some(context_wgpu.unwrap()));
                self.ctx.winit.as_ref().map(|w| w.request_redraw());
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) 
    {
        if !Gpu::is_init() { return; }

        match event 
        {
            WindowEvent::CloseRequested =>  { event_loop.exit(); }
            WindowEvent::Resized(new_size) => {
                if let Some(window) = self.ctx.winit.as_ref()
                {
                    Gpu.resize([new_size.width as _, new_size.height as _].into());
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => 
            {
                if event.physical_key == winit::keyboard::PhysicalKey::Code(KeyCode::Escape)
                {
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        // FIXME: The draw() should not be here
        Ctx.winit.as_mut().map(|window| window.request_redraw());
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        Ctx::destroy();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.update();
        // FIXME: The draw() should not be here
        //self.draw();
    }
}



impl<A> AppRunner<A> where A:App
{
    pub fn draw(&mut self)
    {
        Gpu.begin_draw();

        self.app.draw();

        Gpu.end_draw();
        //Gpu.draw_remove_me();
    }
}
