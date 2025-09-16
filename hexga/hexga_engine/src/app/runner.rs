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
}

impl<A> winit::application::ApplicationHandler for AppRunner<A> where A:App
{
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) 
    {
        self.app.resumed(&mut self.ctx);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) 
    {
        dbg!(event);
    }
}