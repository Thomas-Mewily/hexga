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
pub type AppMessage = EventMessage<AppGraphics>;

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
            EventMessage::User(_u) => {},
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

impl<'a, T : ?Sized>  WindowLoop<AppGraphics> for AppRunner<'a, T> where T : AppLoop
{
    fn handle_message(&mut self, message: AppMessage, ctx: &mut WindowCtx) -> bool
    {
        if message.is_resume()
        {

        }
        self.data.handle_message(message, &mut self.ctx)
    }

    fn user_event(&mut self, graphic: AppGraphics, ctx: &mut WindowCtx) {
        self.ctx.graphics = Asset::Loaded(graphic);
    }
}

type AppGraphics = Graphics;
type WindowAppRunParam = WindowRunParam<AppGraphics>;

#[derive(Default, Debug)]
pub struct AppRunParam
{
    window_param : WindowAppRunParam,
    graphics_param : GraphicsParam,
}
impl AppRunParam
{
    pub fn new() -> Self { ___() }

    pub fn window_param(&self) -> &WindowAppRunParam { &self.window_param }
    pub fn window_param_mut(&mut self) -> &mut WindowAppRunParam { &mut self.window_param }
    pub fn with_window_param(self, window_param : WindowAppRunParam) -> Self { Self { window_param, ..self } }

    pub fn graphics_param(&self) -> &GraphicsParam { &self.graphics_param }
    pub fn graphics_param_mut(&mut self) -> &mut GraphicsParam { &mut self.graphics_param }
    pub fn with_graphics_param(self, graphics_param : GraphicsParam) -> Self { Self { graphics_param, ..self } }
}

impl IWindowRunParam<AppGraphics> for AppRunParam
{
    fn wait_for_event(&self) -> bool { self.window_param.wait_for_event() }
    fn with_wait_for_event(self, wait_for_event : bool) -> Self { Self { window_param: self.window_param.with_wait_for_event(wait_for_event), ..self } }

    fn default_window(&self) -> Option<&WindowParam> { self.window_param.default_window() }
    fn with_default_window(self, default_window : Option<WindowParam>) -> Self { Self { window_param: self.window_param.with_default_window(default_window), ..self } }

    fn event_loop_param(&self) -> &AppGraphics { self.window_param.event_loop_param() }
    fn with_event_loop_param(self, event_loop_param : AppGraphics) -> Self { Self { window_param: self.window_param.with_event_loop_param(event_loop_param), ..self } }
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
        let r = <AppRunner<'a, Self> as WindowRun<AppGraphics>>::run_with_param(&mut runner, param.window_param);

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
