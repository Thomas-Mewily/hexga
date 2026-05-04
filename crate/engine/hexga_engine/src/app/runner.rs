use super::*;

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub trait AppRunner<Ctx> : Sized
    where Ctx: AppContext
{
    fn run(self) -> AppResult where Ctx: Default { self.run_with_param(AppParam::default()) }
    fn run_with_param(self, param : AppParam) -> AppResult where Ctx: Default { self.run_with_param_and_ctx(param, ___()) }
    fn run_with_param_and_ctx(self, param : AppParam, ctx: Ctx) -> AppResult ;
}

impl<F,A> AppRunner<AppCtx> for F 
    where 
    F: AppInit<A> + Fn() -> A + Async,
    A: App<AppEvent,AppCtx>,
    AppCtx: AppContext
{
    fn run_with_param_and_ctx(self, param : AppParam, ctx: AppCtx) -> AppResult where AppCtx: AppContext
    {
        free_fn::init();

        let event_loop = WinitEventLoop::with_user_event().build().map_err(|_|())?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        let mut runner = Runner
        {
            ctx,
            app: LazyFnValue::new(self),
            param,
            proxy
        };

        // Todo handle wasm32
        event_loop.run_app(&mut runner).map_err(|_|());

        Ok(())
    }
}

pub(crate) struct Runner<F, A, Ctx>
    where 
    F: AppInit<A>,
    A: App<AppEvent,Ctx>,
    Ctx: AppContext
{
    ctx : Ctx,
    app : LazyFnValue<A, F>,
    param : AppParam,
    proxy: WinitEventLoopProxy,
}

impl<F, A, Ctx> App<AppEvent,()> for Runner<F, A, Ctx>
    where 
        F: AppInit<A>,
        A: App<AppEvent,Ctx>,
        Ctx: AppContext
{
    fn message(&mut self, msg: AppMessage, ctx: &mut ()) {
        
    }
}


impl<F, A, Ctx> winit::application::ApplicationHandler<AppInternalEvent> for Runner<F, A, Ctx>
    where 
        F: AppInit<A>,
        A: App<AppEvent,Ctx>,
        Ctx: AppContext
{
    fn resumed(&mut self, event_loop: &WinitEventLoopActive) 
    {
        if self.ctx.window().init_window_if_needed(event_loop)
        {
            if self.ctx.graphics().is_none()
            {
                let shared_window = self.ctx.window().window.as_ref().unwrap().clone();

                AppGraphics::init(
                    shared_window,
                    self.param.gpu.clone(),
                    self.proxy.clone(),
                )
                .expect("failed to init the gpu");
            }
        }

        self.message(AppMessage::Flow(AppFlow::Resumed), &mut ());
    }

    fn suspended(&mut self, event_loop: &WinitEventLoopActive) {
        self.message(AppMessage::Flow(AppFlow::Suspended), &mut ());
    }

    fn window_event(
        &mut self,
        event_loop: &WinitEventLoopActive,
        window_id: WinitWindowID,
        event: winit::event::WindowEvent,
    ) {
        
    }

    fn new_events(&mut self, event_loop: &WinitEventLoopActive, cause: winit::event::StartCause)
    {
        
    }

    fn exiting(&mut self, event_loop: &WinitEventLoopActive) {  }

    fn about_to_wait(&mut self, event_loop: &WinitEventLoopActive) {
        
    }
}
