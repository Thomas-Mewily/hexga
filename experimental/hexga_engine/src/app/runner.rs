//use crate::window::WindowInitGpu;
use super::*;

//pub type AppResult = AppResult;
//pub type AppError = AppError;

// Todo wrap app run for the engine

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub trait AppRun: for<'a> App<(), AppCtx<'a>> + Sized + 'static //where CustomEvent: IAppEvent,
{
    fn run<F>(init: F) -> AppResult
    where
        F: AppInit<Self>,
    {
        Self::run_with_param(init, ___())
    }

    fn run_with_param<F>(init: F, param: AppParam) -> AppResult
    where
        F: AppInit<Self>,
    {
        let p = param.event_loop.clone();
        AppRunner::<F, Self>::run_raw_with_param(|proxy| AppRunner { init, app: None, param, proxy }, p)
        /*
        AppRunner
        {
            init,
            app: None,
            param,
            proxy: todo!(),
        }
        */
    }
}
impl<S> AppRun for S where S: for<'a> App<(), AppCtx<'a>> + Sized + 'static {} //, CustomEvent: IAppEvent {}

pub type AppCtx<'a> = ();

pub(crate) struct AppRunner<F, A>
where
    F: AppInit<A>,
    A: for<'a> App<(), AppCtx<'a>>,
{
    init: F,
    app: Option<A>,
    param: AppParam,
    proxy: AppInternalProxy,
}

impl<F, A> AppRunner<F, A>
where
    F: AppInit<A>,
    A: for<'a> App<(), AppCtx<'a>>,
{
    fn exit(&mut self)
    {
        WINDOW.reset();
        self.app = None;
    }

    fn init_stuff_if_needed(&mut self)
    {
        self.init_graphics_if_needed();
        self.init_app_if_needed();
    }

    fn init_graphics_if_needed(&mut self)
    {
        if CurrentWindow::is_not_init() || Gpu::is_not_init()
        {
            return;
        }

        if GRAPHICS.try_get_mut().is_err()
        {
            let mut w = WINDOW.get_mut();
            if let Some(surface) = w.surface()
            {
                GRAPHICS
                    .init_from_fn(|| Graphics::new(surface.surface(), w.size()))
                    .ok_or_void()
                    .expect("Can't init the graphics");
            }
        }
    }

    fn init_app_if_needed(&mut self)
    {
        if self.app.is_some() || Gpu::is_not_init()
        {
            return;
        }

        match WINDOW.try_get_mut()
        {
            Ok(mut w) =>
            {
                if w.surface().is_some()
                {
                    let mut app = (self.init)();
                    app.message(AppMessage::Event(AppEvent::WindowOpen), &mut ());
                    app.message(AppMessage::Flow(AppFlow::Resume), &mut ());
                    self.app = Some(app);
                    w.request_draw();
                }
            }
            Err(e) =>
            {}
        }
    }
}

impl<F, A> Drop for AppRunner<F, A>
where
    F: AppInit<A>,
    A: for<'a> App<(), AppCtx<'a>>,
{
    fn drop(&mut self) { self.exit(); }
}

impl<'a, F, A> App<AppCustomEvent, AppInternalEventLoop<'a>> for AppRunner<F, A>
where
    F: AppInit<A>,
    A: App<(), AppCtx<'a>>,
{
    fn update(&mut self, dt: Duration, _event_loop: &mut AppInternalEventLoop<'a>)
    {
        if let Some(app) = &mut self.app { app.message(AppMessage::Flow(AppFlow::Update(dt)), &mut ()); }
    }

    fn draw(&mut self, _event_loop: &mut AppInternalEventLoop<'a>)
    {
        if let Some(app) = &mut self.app
        {
            if WINDOW.try_get_mut().ok().and_then(|window| window.surface().map(|_| ())).is_none() { return; }
            if let Ok(mut graphics) = GRAPHICS.try_get_mut() { graphics.begin_draw(); } else { return; }
            app.message(AppMessage::Flow(AppFlow::Draw), &mut ());
            if let Ok(mut graphics) = GRAPHICS.try_get_mut() { graphics.end_draw(); }
        }
    }

    fn message(&mut self, message: AppInternalMessage, event_loop: &mut AppInternalEventLoop<'a>) -> Option<AppInternalMessage>
    {
        match message
        {
            AppMessage::Custom(AppCustomEvent::SurfaceReady(surface)) =>
            {
                let mut window = WINDOW.try_get_mut().ok()?;
                let size = window.size();
                window.replace_surface(Some(GpuSurfaceConfigured::from_surface(surface, size)));
                window.configure_surface();
                window.request_draw();
                drop(window);
                self.init_stuff_if_needed();
                None
            }
            AppMessage::Custom(AppCustomEvent::GpuReady(gpu)) =>
            {
                hexga_graphics::gpu::experimental::GPU.init(gpu).ok_or_void().expect("Can't init the gpu");
                assert!(Gpu::is_init());
                self.init_stuff_if_needed();
                None
            }
            AppMessage::Custom(AppCustomEvent::GpuError(_)) => panic!("Can't init the gpu"),
            message => self.dispatch_message(message, event_loop),
        }
    }

    fn event(&mut self, ev: AppEvent, event_loop: &mut AppInternalEventLoop<'a>) -> Option<AppEvent>
    {
        if matches!(ev, AppEvent::WindowResize(_))
            && let Ok(mut window) = WINDOW.try_get_mut()
        {
            window.configure_surface();
        }
        let Some(event) = self.app.as_mut()?.message(ev.into(), &mut ()) else {
            return None;
        };
        match event
        {
            AppMessage::Event(event) => Some(event),
            _ => None,
        }
    }

    fn resume(&mut self, event_loop: &mut AppInternalEventLoop<'a>)
    {
        let mut created = false;
        let mut window = WINDOW
            .init_from_fn(|| {
                created = true;
                event_loop.create_window(self.param.window.clone()).expect("failed to create main window")
            })
            .ok_or_void()
            .expect("can't init the main window");

        if created || window.surface().is_none() { window.initialize_surface(&self.param.gpu, event_loop).expect("failed to init the surface"); }
        if let Some(app) = &mut self.app { app.message(AppMessage::Flow(AppFlow::Resume), &mut ()); }
    }

    fn pause(&mut self, _event_loop: &mut AppInternalEventLoop<'a>)
    {
        if let Some(app) = &mut self.app { app.message(AppMessage::Flow(AppFlow::Pause), &mut ()); }
        if let Ok(mut window) = WINDOW.try_get_mut() { window.destroy_surface(); }
    }

    fn exit(&mut self, _event_loop: &mut AppInternalEventLoop<'a>)
    {
        if let Some(app) = &mut self.app { app.message(AppMessage::Flow(AppFlow::Exit), &mut ()); }
        AppRunner::exit(self);
    }
}
