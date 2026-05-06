use super::*;

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub trait AppRun<Ctx=AppDefaultCtx> : Sized
{
    fn run(self) -> AppResult where Ctx: Default { self.run_with_param(AppParam::default()) }
    fn run_with_param(self, param : AppParam) -> AppResult where Ctx: Default { self.run_with_param_and_ctx(param, ___()) }
    fn run_with_param_and_ctx(self, param : AppParam, ctx: Ctx) -> AppResult;
    /*
    {
        <AppWithCtx<Self, Ctx> as AppRunRaw>::run_raw(AppWithCtx::new(self, ctx), param) 
    }*/
}

impl<Ctx,F,A> AppRun<Ctx> for F 
    where F: Fn() -> A, A: App<Ctx>, Ctx: App<AppInitOnResume<F,A,Ctx>> + App<A>
{
    fn run_with_param_and_ctx(self, param : AppParam, ctx: Ctx) -> AppResult {
        let app = AppInitOnResume::new(self);
        AppRunRaw::run_raw(AppWithCtx::new(app, ctx), param)
    }
}


// Lazily create the app on App::resume()
pub struct AppInitOnResume<F,A,Ctx>
where
    F: Fn() -> A,
    A: App<Ctx>,
    Ctx: App<A>
{
    app : LazyFnValue<A,F>,
    phantom: PhantomData<Ctx>
}
impl<F,A,Ctx> AppInitOnResume<F,A,Ctx>
where
    F: Fn() -> A,
    A: App<Ctx>,
    Ctx: App<A>
{
    pub fn new(init: F) -> Self { Self { app: LazyFnValue::new(init), phantom: PhantomData }}
}

impl<F,A,Ctx> App<Ctx> for AppInitOnResume<F,A,Ctx>
where
    F: Fn() -> A,
    A: App<Ctx>,
    Ctx: App<A>
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<Ctx>) -> Option<AppEvent> 
    { 
        match self.app.observe_mut()
        {
            Some(app) => app.event(ev, ctx),
            None => None,
        }
    }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<Ctx>) 
    {
        match self.app.observe_mut()
        {
            Some(app) => app.update(dt, ctx),
            None => {},
        }
    }

    fn draw(&mut self, ctx: &mut AppCtx<Ctx>) 
    {
        match self.app.observe_mut()
        {
            Some(app) => app.draw(ctx),
            None => {},
        }
    }

    fn resumed(&mut self, ctx: &mut AppCtx<Ctx>)
    {
        self.app.as_mut().resumed(ctx);
    }

    fn paused(&mut self, ctx: &mut AppCtx<Ctx>)
    {
        match self.app.observe_mut()
        {
            Some(app) => app.paused(ctx),
            None => {},
        }
    }
}




pub trait AppRunRaw
{
    /// Run the app without wrapping it
    fn run_raw(app : Self, param: AppParam) -> AppResult where Self: App<()>
    {
        free_fn::init();

        let event_loop = WinitEventLoop::with_user_event().build().map_err(|_|())?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();

        let mut runner = AppRunner
        {
            app,
            param,
            proxy,
            event_loop: AppEventLoopInner::new(),
        };

        // Todo handle wasm32
        event_loop.run_app(&mut runner).map_err(|_|());

        Ok(())
    }
}
impl<A> AppRunRaw for A where A: App<()> {}

/*
impl<F,A> AppRunner<AppCtx> for F 
    where 
    F: AppInit<A> + Fn() -> A + Async,
    A: App<AppEvent,AppCtx>,
    AppCtx: AppContext<A>
{
    fn run_with_param_and_ctx(self, param : AppParam, ctx: AppCtx) -> AppResult where AppCtx: AppContext<A>
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
            proxy,
            unhandled_event: Vec::new(),
        };

        // Todo handle wasm32
        event_loop.run_app(&mut runner).map_err(|_|());

        Ok(())
    }
}
*/

/// Interface between the winit Application trait <=> App trait
pub(crate) struct AppRunner<A>
    where 
    A: App<()>,
    //Ctx: AppContext<A>
{
    app: A,
    param : AppParam,
    proxy: WinitEventLoopProxy,
    event_loop: AppEventLoopInner,
    /*
    ctx : Ctx,
    app : LazyFnValue<A, F>,
    param : AppParam,
    proxy: WinitEventLoopProxy,
    unhandled_event: Vec<AppEvent>,
    */
}

impl<A> winit::application::ApplicationHandler<AppInternalEvent> for AppRunner<A>
    where 
        A: App<()>,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app.resumed(&mut AppCtx::new(&mut (), &mut AppEventLoop::new(event_loop, &mut self.event_loop)));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}

/*
impl<A> Runner<A>
    where 
        A: App<>,
{
    fn event(&mut self, ev: impl Into<AppEvent>, ev_loop: &WinitEventLoopActive) -> Option<AppEvent> { self.app.event(ev.into(), &mut AppEventLoop::new(ev_loop)) }
    
    fn update_app(&mut self)
    {
        let now = Time::since_launch();
        let last_time = self.ctx.time().current;
        let mut dt = now - last_time;
        
        let (step_dt, consume_dt_rest) = match self.ctx.time().strategy {
            TimeStrategy::Variable => 
            {
                if dt > DeltaTime::ZERO 
                {
                    self.update(dt, &mut ());
                }
                return;
            }
            TimeStrategy::Fixed(step_dt) => (step_dt, false),
            TimeStrategy::Capped(max_dt) => (dt.min_partial(max_dt), true)
        };

        if step_dt.is_negative_or_zero() { return; }
        
        while dt >= step_dt 
        {
            self.update(step_dt, &mut ());
            dt -= step_dt;
        }
        
        if consume_dt_rest && dt > DeltaTime::ZERO
        {
            self.update(dt, &mut ());
            dt = DeltaTime::ZERO;
        }
    }
    /*
    fn message(&mut self, msg: impl Into<AppMessage>) 
    { 
        match msg.into()
        {
            AppMessage::Event(event) => { self.event(event); },
            AppMessage::Flow(flow) => self.dispatch_flow(flow),
        }
    }

    fn dispatch_flow(&mut self, flow: AppFlow) 
    {
        match flow
        {
            AppFlow::Resumed => self.resumed(&mut ()),
            AppFlow::Suspended => self.suspended(&mut ()),
            AppFlow::Update(dt) => self.update(dt, &mut ()),
            AppFlow::Draw => self.draw(&mut ()),
        }
    }

    fn init_app_if_needed(&mut self)
    {
        if self.app.is_init() { return; }
        
        let time = Time::since_launch();
        self.ctx.time().current = time;
        self.ctx.time().last = time;
        self.ctx.time().dt = zero();
        //self.ctx.time().tick = 0;

        let app = self.app.as_mut();
        app.resumed(&mut self.ctx);
        for ev in self.unhandled_event.drain(..)
        {
            app.event(ev, &mut self.ctx);
        }
    }

    fn update_app(&mut self)
    {
        if !self.app.is_init() { return; }
        
        let now = Time::since_launch();
        let last_time = self.ctx.time().current;
        let mut dt = now - last_time;
        
        let (step_dt, consume_dt_rest) = match self.ctx.time().strategy {
            TimeStrategy::Variable => 
            {
                if dt > DeltaTime::ZERO 
                {
                    self.update(dt, &mut ());
                }
                return;
            }
            TimeStrategy::Fixed(step_dt) => (step_dt, false),
            TimeStrategy::Capped(max_dt) => (dt.min_partial(max_dt), true)
        };

        if step_dt.is_negative_or_zero() { return; }
        
        while dt >= step_dt 
        {
            self.update(step_dt, &mut ());
            dt -= step_dt;
        }
        
        if consume_dt_rest && dt > DeltaTime::ZERO
        {
            self.update(dt, &mut ());
            dt = DeltaTime::ZERO;
        }
    }
    */
}
*/
/*
impl<F, A, Ctx> App<AppEvent,()> for Runner<F, A, Ctx>
    where 
        F: AppInit<A>,
        A: App<AppEvent,Ctx>,
        Ctx: AppContext
{
    fn draw(&mut self, ctx: &mut ()) 
    {
        self.ctx.draw(&mut ());
        if self.app.is_init()
        {
            self.app.as_mut().draw(&mut self.ctx);
        }
        self.ctx.end_draw();
    }

    fn update(&mut self, dt: DeltaTime, _ctx: &mut ()) 
    {
        self.ctx.time().last = self.ctx.time().current;
        self.ctx.time().current += dt;
        self.ctx.time().dt = dt;
        self.ctx.time().tick += 1;

        self.ctx.update(dt, &mut ());
        if self.app.is_init() { self.app.as_mut().update(dt, &mut self.ctx); }
        self.ctx.end_update(dt);
        
    }

    fn resumed(&mut self, ctx: &mut ()) 
    {
        self.ctx.resumed(ctx);
        if self.app.is_init()
        {
            self.app.as_mut().resumed(&mut self.ctx);
        }
    }

    fn suspended(&mut self, ctx: &mut ()) {
        self.ctx.suspended(&mut ());
        if self.app.is_init()
        {
            self.app.as_mut().suspended(&mut self.ctx);
        }
    }

    fn event(&mut self, ev: AppEvent, ctx: &mut ()) -> Option<AppEvent> 
    {
        self.ctx.ev
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
            if self.ctx.try_graphics().is_none()
            {
                let shared_window = self.ctx.window().window.as_ref().unwrap().clone();

                Graphics::init(
                    shared_window,
                    self.param.gpu.clone(),
                    None,
                    self.proxy.clone(),
                )
                .expect("failed to init the gpu");
                self.event(AppEvent::Window(WindowEvent::Open));
            }
        }

        self.message(AppMessage::Flow(AppFlow::Resumed));
    }

    fn suspended(&mut self, event_loop: &WinitEventLoopActive) 
    {
        self.message(AppMessage::Flow(AppFlow::Suspended));
        // Do I need to destroy the window / graphics on some platform ?
    }

    fn window_event(
        &mut self,
        event_loop: &WinitEventLoopActive,
        window_id: WinitWindowID,
        event: winit::event::WindowEvent,
    ) {
        match event
        {
            WinitWindowEvent::Resized(physical_size) =>
            {
                self.event(WindowEvent::Resize(physical_size.convert()));
            }
            winit::event::WindowEvent::CloseRequested =>
            {
                self.event(WindowEvent::Close);
                event_loop.exit();
            }
            winit::event::WindowEvent::Destroyed =>
            {
                self.event(WindowEvent::Destroy);
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
                self.event(AppEvent::Input(InputEvent::Key(key).into()));
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
                self.update_app();
                self.message(AppMessage::Flow(AppFlow::Draw));
            }
            _ => (),
        }
    }

    fn new_events(&mut self, event_loop: &WinitEventLoopActive, cause: winit::event::StartCause)
    {
        
    }

    fn exiting(&mut self, event_loop: &WinitEventLoopActive) 
    { 
        self.dispatch_flow(AppFlow::Suspended);
    }

    fn about_to_wait(&mut self, event_loop: &WinitEventLoopActive) {
        // Todo: remove the update_app()
        self.update_app();
        self.ctx.window().undirty_if_needed();
    }

    fn user_event(&mut self, event_loop: &WinitEventLoopActive, event: AppInternalEvent) 
    {
        match event
        {
            AppInternalEvent::Gpu(app_graphics) =>
            {
                *self.ctx.try_graphics() = Some(app_graphics.expect("failed to init the gpu"));
                //app.graphics = Some(app_graphics.expect("failed to init the gpu"));
                self.ctx.window().init_surface_if_needed();
                self.init_app_if_needed();
                //app.window.init_surface_if_needed();
            }
        }
    }

}
*/

/*
impl<A> winit::application::ApplicationHandler<AppInternalEvent> for Runner<A>
    where 
    A: App<>,
{
    fn resumed(&mut self, event_loop: &WinitEventLoopActive) 
    {
        self.app.resumed(&mut AppEventLoop::new(event_loop));
    }

    fn suspended(&mut self, event_loop: &WinitEventLoopActive) 
    {
        self.app.resumed(&mut AppEventLoop::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &WinitEventLoopActive,
        window_id: WinitWindowID,
        event: winit::event::WindowEvent,
    ) {
        match event
        {
            WinitWindowEvent::Resized(physical_size) =>
            {
                self.event(WindowEvent::Resize(physical_size.convert()), event_loop);
            }
            winit::event::WindowEvent::CloseRequested =>
            {
                self.event(WindowEvent::Close, event_loop);
                //event_loop.exit();
            }
            winit::event::WindowEvent::Destroyed =>
            {
                self.event(WindowEvent::Destroy, event_loop);
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
                self.event(AppEvent::Input(InputEvent::Key(key).into()), event_loop);
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
                self.update_app();
                self.message(AppMessage::Flow(AppFlow::Draw));
            }
            _ => (),
        }
    }

    fn new_events(&mut self, event_loop: &WinitEventLoopActive, cause: winit::event::StartCause)
    {
        
    }

    fn exiting(&mut self, event_loop: &WinitEventLoopActive) 
    { 
        self.dispatch_flow(AppFlow::Suspended);
    }

    fn about_to_wait(&mut self, event_loop: &WinitEventLoopActive) {
        // Todo: remove the update_app()
        self.update_app();
        self.ctx.window().undirty_if_needed();
    }

    fn user_event(&mut self, event_loop: &WinitEventLoopActive, event: AppInternalEvent) 
    {
        match event
        {
            AppInternalEvent::Gpu(app_graphics) =>
            {
                *self.ctx.try_graphics() = Some(app_graphics.expect("failed to init the gpu"));
                //app.graphics = Some(app_graphics.expect("failed to init the gpu"));
                self.ctx.window().init_surface_if_needed();
                self.init_app_if_needed();
                //app.window.init_surface_if_needed();
            }
        }
    }
}
*/