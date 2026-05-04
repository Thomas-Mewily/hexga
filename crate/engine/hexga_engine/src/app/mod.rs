use super::*;

pub mod prelude
{
    pub use super::{App,AppRunner,Ctx};
}

#[derive(Default)]
pub struct Ctx;

impl AppCtx for Ctx
{

}

pub trait AppCtx
{

}

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub trait App<Ctx=crate::app::Ctx>
{
    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx);
    fn draw(&mut self, ctx: &mut Ctx);

    fn resumed(&mut self, ctx: &mut Ctx) {}
    fn suspended(&mut self, ctx: &mut Ctx) {}
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppParam
{
    name: String,
}

pub type AppResult = Result;

pub trait AppRunner<Ctx> : Sized
    where Ctx: AppCtx
{
    fn run(self) -> AppResult where Ctx: Default { self.run_with_param(AppParam::default()) }
    fn run_with_param(self, param : AppParam) -> AppResult where Ctx: Default { self.run_with_param_and_ctx(param, ___()) }
    fn run_with_param_and_ctx(self, param : AppParam, ctx: Ctx) -> AppResult ;
}

impl<F,A> AppRunner<Ctx> for F 
    where 
    F: AppInit<A> + Fn() -> A + Async,
    A: App,
    Ctx: AppCtx
{
    fn run_with_param_and_ctx(self, param : AppParam, ctx: Ctx) -> AppResult where Ctx: AppCtx
    {
        let mut runner = Runner
        {
            ctx,
            app: None,
            init: self,
            param,
        };

        let event_loop = winit::event_loop::EventLoop::with_user_event().build().map_err(|_|())?;

        // Todo handle wasm32
        event_loop.run_app(&mut runner).map_err(|_|());

        Ok(())
    }
}

pub(crate) struct Runner<F, A, Ctx>
    where 
    F: AppInit<A>,
    A: App<Ctx>,
    Ctx: AppCtx
{
    ctx : Ctx,
    app : Option<A>,
    init: F,
    param : AppParam,
}


impl<F, A, Ctx> winit::application::ApplicationHandler<()> for Runner<F, A, Ctx>
    where 
        F: AppInit<A>,
        A: App<Ctx>,
        Ctx: AppCtx
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(app) = &mut self.app
        {
            app.resumed(&mut self.ctx);
        }
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(app) = &mut self.app
        {
            app.suspended(&mut self.ctx);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }
}