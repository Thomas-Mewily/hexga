use crate::*;

pub mod prelude
{
    pub use super::
    {
        AppCtx,
        AppLoop,
        AppRun,
    };
}

#[derive(Default, Debug)]
pub struct AppContext
{
    window_context : WindowContext<WindowGraphicsData>,
    proxy : Option<EventLoopProxy<GraphicsEvent>>,
    graphics : Graphics,
}

impl IAppCtx for AppContext
{
    fn draw_window(&mut self, id : WindowID)
    {
        let Some(w) = self.window_context.window(id) else { return; };
        let d = *w.data();
        self.draw_surface(d);
    }
    fn draw_surface(&mut self, id : SurfaceID) { self.graphics.draw_surface(id); }
    fn draw(&mut self) { self.graphics.draw_all_window(); }
}

pub trait IAppCtx : Debug
{
    fn draw_window(&mut self, id : WindowID);
    fn draw_surface(&mut self, id : SurfaceID);
    fn draw(&mut self);
}


struct AppRunner<T> where T : AppLoop
{
    ctx  : AppContext,
    data : T,
}

pub type AppCtx<'a> = dyn IAppCtx + 'a;
pub type AppMessage = hexga_engine_window::event::EventMessage<(), ()>;
pub(crate) type AppMessageInternal = hexga_engine_window::event::EventMessage<GraphicsEvent, WindowGraphicsData>;

pub type LocalizedEvent = hexga_engine_window::event::LocalizedEvent;
pub(crate) type LocalizedEventInternal = hexga_engine_window::event::LocalizedEvent<WindowGraphicsData>;

pub type EventMessage = hexga_engine_window::event::EventMessage<(),WindowGraphicsData>;
pub(crate) type EventMessageInternal = hexga_engine_window::event::EventMessage<GraphicsEvent,WindowGraphicsData>;

pub type WindowID = hexga_engine_window::window::WindowID<WindowGraphicsData>;

pub trait AppLoop
{
    fn handle_message(&mut self, message: AppMessage, ctx: &mut AppCtx) -> bool
    {
        self.dispatch_message(message, ctx)
    }

    fn dispatch_message(&mut self, message: AppMessage, ctx: &mut AppCtx) -> bool
    {
        match message
        {
            EventMessage::LocalizedEvent(localized_event) =>
            {
                if let Event::Window(WindowEvent::Draw) = localized_event.event
                {
                    self.draw_window(localized_event.window, ctx);
                }
                else
                {
                    return self.handle_localized_event(localized_event, ctx);
                }
            },
            EventMessage::Device(device_message) => match device_message
            {
                DeviceMessage::Resume  => self.resume(ctx),
                DeviceMessage::Update  => self.update(ctx),
                DeviceMessage::Exit    => self.exit(ctx),
                _ => {},
            },
            EventMessage::User(_) => {},
        }
        true
    }

    fn handle_localized_event(&mut self, event: LocalizedEvent, ctx: &mut AppCtx) -> bool
    {
        self.handle_event(event.event, ctx)
    }

    fn handle_event(&mut self, event : Event, ctx: &mut AppCtx) -> bool
    {
        let _ = event;
        let _ = ctx;
        false
    }

    fn update(&mut self, ctx: &mut AppCtx) { let _ = ctx; }

    fn draw_window(&mut self, id: WindowID, ctx: &mut AppCtx) { let _ = id; self.draw(ctx); }
    fn draw(&mut self, ctx: &mut AppCtx) { let _ = ctx; }

    fn resume(&mut self, ctx: &mut AppCtx) { let _ = ctx; }
    fn pause(&mut self, ctx: &mut AppCtx) { let _ = ctx; }

    fn exit(&mut self, ctx: &mut AppCtx) { let _ = ctx; }
}

impl<T> WindowLoop<GraphicsEvent, WindowGraphicsData> for AppRunner<T> where T : AppLoop
{
    fn handle_message(&mut self, message: EventMessage, ctx: &mut WindowCtx<WindowGraphicsData>) -> bool
    {
        let event = match &message
        {
            event::EventMessage::LocalizedEvent(localized_event) => Some(v.clone()),
            event::EventMessage::Device(device_message) => { let _ = self.dispatch_message(event::EventMessage::Device(device_message.clone()), ctx); },
            event::EventMessage::User(g) => {},
        };

        match &message
        {
            event::EventMessage::LocalizedEvent(localized_event) => { let _ = self.dispatch_message(event::EventMessage::LocalizedEvent(localized_event.clone()), ctx); },
            event::EventMessage::Device(device_message) => { let _ = self.dispatch_message(event::EventMessage::Device(device_message.clone()), ctx); },
            event::EventMessage::User(g) => {},
        };
        self.data.dispatch_message(message, &mut self.ctx)
    }

    fn user_event(&mut self, graphic: GraphicsEvent, ctx : &mut WindowCtx<WindowGraphicsData>)
    {
        self.ctx.graphics.handle_event(graphic, ctx);
    }

    fn resume(&mut self, ctx: &mut WindowCtx<WindowGraphicsData>)
    {
        ctx.resume();

        if let Some(proxy) = &self.ctx.proxy
        {
            self.ctx.graphics.resume(ctx, proxy.clone());
        }
    }

    fn draw_window(&mut self, id : WindowID, ctx: &mut WindowCtx<WindowGraphicsData>)
    {
        let _ = ctx;
        if let Some(window) = ctx.window(id)
        {
            //AAAAAAAAAAA
            self.ctx.draw_surface(*window.data());
        }
        //self.ctx.draw_window(id);
    }

    fn handle_localized_event(&mut self, event: LocalizedEvent, ctx: &mut WindowCtx<WindowGraphicsData>) -> bool {
        let LocalizedEvent { window: window_id, event, device : _, .. } =  event;
        let Some(window) = ctx.window(window_id) else {
            return false;
        };

        println!("{:?}", event);
        //dbg!(&event);

        match event
        {
            Event::Window(w) => match w
            {
                WindowEvent::Resize(size) => { self.ctx.graphics.resize_window(window, size); },
                WindowEvent::Visible(_visible) => {}, // Maybe can do something with this
                _ => {},
            }
            _ => {}
        }
        true
    }

    fn pause(&mut self, ctx: &mut WindowCtx<WindowGraphicsData>)
    {
        self.ctx.graphics.pause();
        ctx.pause();
    }
}

#[derive(Default, Debug)]
pub struct AppRunParam
{
    window_param : WindowRunParam<WindowGraphicsData>,
    graphics_param : GraphicsParam,
}
impl AppRunParam
{
    pub fn new() -> Self { ___() }

    pub fn window_param(&self) -> &WindowRunParam<WindowGraphicsData> { &self.window_param }
    pub fn window_param_mut(&mut self) -> &mut WindowRunParam<WindowGraphicsData> { &mut self.window_param }
    pub fn with_window_param(self, window_param : WindowRunParam<WindowGraphicsData>) -> Self { Self { window_param, ..self } }

    pub fn graphics_param(&self) -> &GraphicsParam { &self.graphics_param }
    pub fn graphics_param_mut(&mut self) -> &mut GraphicsParam { &mut self.graphics_param }
    pub fn with_graphics_param(self, graphics_param : GraphicsParam) -> Self { Self { graphics_param, ..self } }
}

impl IWindowRunParam<WindowGraphicsData> for AppRunParam
{
    fn wait_for_event(&self) -> bool { self.window_param.wait_for_event() }
    fn with_wait_for_event(self, wait_for_event : bool) -> Self { Self { window_param: self.window_param.with_wait_for_event(wait_for_event), ..self } }

    fn default_window(&self) -> Option<&WindowParam<WindowGraphicsData>> { self.window_param.default_window() }
    fn with_default_window(self, default_window : Option<WindowParam<WindowGraphicsData>>) -> Self { Self { window_param: self.window_param.with_default_window(default_window), ..self } }
}

impl IGraphicsParam for AppRunParam
{

}

pub trait AppRun : AppLoop + Sized + 'static
{
    fn run(self) -> Result<(), ()> { self.run_with_param(___()) }

    fn run_with_param(self, param : AppRunParam) -> Result<(), ()>
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Allows the setting of the log level through RUST_LOG env var.
            // It also allows wgpu logs to be seen.
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Sets up panics to go to the console.error in browser environments
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Error).expect("Can't initialize the logger");
        }

        let runner = AppRunner { ctx: ___(), data: self };
        let r = <AppRunner<Self> as WindowRun<GraphicsEvent, WindowGraphicsData>>::run_with_param_and_init_from_event_loop
        (runner,
            param.window_param,
            |s, event_loop|
            {
                s.ctx.proxy = Some(event_loop.create_proxy());
            }
        );

        if r.is_err()
        {
            Err(())
        }else
        {
            Ok(())
        }
    }
}
impl<T> AppRun for T where T : AppLoop + Sized + 'static { }
