use super::*;

#[derive(Default)]
pub struct AppParam
{

}

pub trait AppRun : Sized
{
    fn run(self) -> Result<(), ()> { self.run_with_param(___()) }
    fn run_with_param(self, param: AppParam) -> Result<(), ()>;
}





impl<A> AppRun for A where A:App
{
    fn run_with_param(self, _param: AppParam) -> Result<(), ()>
    {
        log::init_logger();

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        let proxy = event_loop.create_proxy();

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self, Ctx::new(proxy));

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

pub(crate) struct AppRunner<A> where A:App
{
    app : A,
    ctx : Ctx,
}
impl<A> AppRunner<A> where A:App
{
    pub fn new(app : A, ctx : Ctx) -> Self 
    {
        Self { app, ctx }
    }


    pub(crate) fn handle_event(&mut self, ev: AppEvent<A::CustomEvent>)
    {
        self.app.handle_event(ev, &mut self.ctx);
    }

    pub(crate) fn update(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Update));
    }

    pub(crate) fn draw(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Draw));
    }

    pub(crate) fn exit(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Exit));
    }

    pub(crate) fn resumed(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Resumed));
    }
    
    pub(crate) fn paused(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Paused));
    }
}


impl<A> winit::application::ApplicationHandler<CtxEvent> for AppRunner<A> where A:App
{
    fn resumed(&mut self, _event_loop: &EventLoopActive) 
    {
        self.resumed();
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.paused();
    }

    fn window_event(
        &mut self,
        event_loop: &EventLoopActive,
        _window_id: WinitWindowID,
        event: WinitWindowEvent,
    ) 
    {
        match event
        {
            WinitWindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>
            {
                let k = KeyEvent::from(event);
                self.ctx.keyboard.handle_key(k);
                self.app.handle_event(k.into(), &mut self.ctx);
            }
            WinitWindowEvent::CloseRequested => { event_loop.exit(); },
            WinitWindowEvent::RedrawRequested => { self.draw(); }
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: CtxEvent) 
    {
        match event
        {
            CtxEvent::Gpu(gpu) => { self.ctx.gpu = Some(gpu.expect("Failed to connect to the gpu")); },
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) 
    {
        self.update();
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) 
    {
        self.exit();
    }
}