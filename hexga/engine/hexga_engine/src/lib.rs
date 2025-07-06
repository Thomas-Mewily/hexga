use hexga_engine_graphics::prelude::*;
use hexga_engine_window::{event::IDeviceMessage, prelude::*, window::{EventLoopProxy, WindowContext, WindowRunParam}};
use hexga_core::prelude::*;//
use std::fmt::Debug;

mod asset;
use asset::*;

pub mod prelude
{
    pub use super::
    {
        AppCtx,
        AppLoop,
        AppRun,
    };

    pub use hexga_engine_window::window::IWindowRunParam; // `game()` `software()` shortcut
}

#[derive(Debug)]
pub struct AppContext
{
    window_context : WindowContext,
    graphics : Asset<Graphics, Option<EventLoopProxy<Graphics>>>,
}

impl Default for AppContext
{
    fn default() -> Self {
        Self { window_context: ___(), graphics: Asset::Loadding(None) }
    }
}


impl IAppCtx for AppContext { }

pub trait IAppCtx
{

}


struct AppRunner<'a, T : ?Sized> where T : AppLoop
{
    ctx  : AppContext,
    data : &'a mut T,
}

pub type AppCtx<'a> = dyn IAppCtx + 'a;
pub type AppMessageInternal = EventMessage<Graphics>;
pub type AppMessage = EventMessage;

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

impl<'a, T : ?Sized>  WindowLoop<Graphics> for AppRunner<'a, T> where T : AppLoop
{
    fn handle_message(&mut self, message: AppMessageInternal, ctx: &mut WindowCtx) -> bool
    {
        let m = message.clone_with_user_message(());
        self.dispatch_message(message, ctx);
        self.data.handle_message(m, &mut self.ctx)
    }

    fn user_event(&mut self, graphic: Graphics, _ : &mut WindowCtx)
    {
        self.ctx.graphics = Asset::Loaded(graphic);
    }

    fn resume(&mut self, ctx: &mut WindowCtx)
    {

        if let Asset::Loadding(proxy) = &mut self.ctx.graphics
        {
            if let Some(proxy) = proxy.take()
            {

            }
        }
    }
}

#[derive(Default, Debug)]
pub struct AppRunParam
{
    window_param : WindowRunParam,
    graphics_param : GraphicsParam,
}
impl AppRunParam
{
    pub fn new() -> Self { ___() }

    pub fn window_param(&self) -> &WindowRunParam { &self.window_param }
    pub fn window_param_mut(&mut self) -> &mut WindowRunParam { &mut self.window_param }
    pub fn with_window_param(self, window_param : WindowRunParam) -> Self { Self { window_param, ..self } }

    pub fn graphics_param(&self) -> &GraphicsParam { &self.graphics_param }
    pub fn graphics_param_mut(&mut self) -> &mut GraphicsParam { &mut self.graphics_param }
    pub fn with_graphics_param(self, graphics_param : GraphicsParam) -> Self { Self { graphics_param, ..self } }
}

impl IWindowRunParam for AppRunParam
{
    fn wait_for_event(&self) -> bool { self.window_param.wait_for_event() }
    fn with_wait_for_event(self, wait_for_event : bool) -> Self { Self { window_param: self.window_param.with_wait_for_event(wait_for_event), ..self } }

    fn default_window(&self) -> Option<&WindowParam> { self.window_param.default_window() }
    fn with_default_window(self, default_window : Option<WindowParam>) -> Self { Self { window_param: self.window_param.with_default_window(default_window), ..self } }
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
        let r = <AppRunner<'a, Self> as WindowRun<Graphics>>::run_with_param_and_init_from_event_loop
        (&mut runner,
            param.window_param,
            |s, event_loop|
            {
                s.ctx.graphics = Asset::Loadding(Some(event_loop.create_proxy()));
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
