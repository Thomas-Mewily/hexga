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
    fn draw(&mut self) {
        self.graphics.draw_all_window();
    }
}

pub trait IAppCtx : Debug
{
    fn draw(&mut self);
}


struct AppRunner<'a, T : ?Sized> where T : AppLoop
{
    ctx  : AppContext,
    data : &'a mut T,
}

pub type AppCtx<'a> = dyn IAppCtx + 'a;
pub type AppMessageInternal = EventMessage<Graphics, SurfaceID>;
pub type AppMessage = EventMessage<(),()>;

pub trait AppLoop
{
    fn handle_message(&mut self, message: AppMessage, ctx: &mut AppCtx) -> bool
    {
        match message
        {
            EventMessage::LocalizedEvent(localized_event) =>
            {
                if let Event::Window(WindowEvent::Draw) = localized_event.event
                {
                    self.draw(ctx);
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
    fn draw(&mut self, ctx: &mut AppCtx) { let _ = ctx; }

    fn resume(&mut self, ctx: &mut AppCtx) { let _ = ctx; }
    fn pause(&mut self, ctx: &mut AppCtx) { let _ = ctx; }

    fn exit(&mut self, ctx: &mut AppCtx) { let _ = ctx; }
}

impl<'a, T : ?Sized> WindowLoop<GraphicsEvent, WindowGraphicsData> for AppRunner<'a, T> where T : AppLoop
{
    fn handle_message(&mut self, message: EventMessage<GraphicsEvent,WindowGraphicsData>, ctx: &mut WindowCtx<WindowGraphicsData>) -> bool {
        let m = message.clone_with_user_message(());
        self.dispatch_message(message, ctx);
        self.data.handle_message(m.with_window_data_type(), &mut self.ctx)
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

    fn draw_window(&mut self, window_id : WindowID<WindowGraphicsData>, ctx: &mut WindowCtx<WindowGraphicsData>)
    {
        let Some(window) = ctx.window(window_id) else {
            return;
        };
        self.ctx.graphics.draw_window(window);
    }

    fn handle_localized_event(&mut self, event: LocalizedEvent<WindowGraphicsData>, ctx: &mut WindowCtx<WindowGraphicsData>) -> bool {
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

pub trait AppRun : AppLoop
{
    fn run(&mut self) -> Result<(), ()> { self.run_with_param(___()) }

    fn run_with_param<'a>(&'a mut self, param : AppRunParam) -> Result<(), ()>
    {
        let mut runner = AppRunner { ctx: ___(), data: self };
        let r = <AppRunner<'a, Self> as WindowRun<GraphicsEvent, WindowGraphicsData>>::run_with_param_and_init_from_event_loop
        (&mut runner,
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
impl<T> AppRun for T where T : AppLoop { }
