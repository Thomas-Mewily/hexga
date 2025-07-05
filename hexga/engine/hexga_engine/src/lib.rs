use hexga_engine_window::{prelude::*, window::{WindowContext, WindowRunParam}};
use hexga_core::prelude::*;//

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

#[derive(Default, Debug)]
pub struct AppContext
{
    window_context : WindowContext,
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

pub trait AppLoop
{
    fn handle_message(&mut self, message: AppMessage, ctx: &mut AppCtx) -> bool
    {
        match message
        {
            AppMessage::LocalizedEvent(localized_event) =>
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
            AppMessage::Device(device_message) => match device_message
            {
                DeviceMessage::Resume  => self.resume(ctx),
                DeviceMessage::Update  => self.update(ctx),
                DeviceMessage::Exit    => self.exit(ctx),
                _ => {},
            },
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

impl<'a, T : ?Sized>  WindowLoop for AppRunner<'a, T> where T : AppLoop
{
    fn handle_message(&mut self, message: AppMessage, ctx: &mut WindowCtx) -> bool
    {
        self.data.handle_message(message, &mut self.ctx)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AppRunParam
{
    window_param : WindowRunParam,
}
impl IWindowRunParam for AppRunParam
{
    fn wait_for_event(&self) -> bool { self.window_param.wait_for_event() }
    fn with_wait_for_event(self, wait_for_event : bool) -> Self { Self { window_param: self.window_param.with_wait_for_event(wait_for_event) } }

    fn default_window(&self) -> Option<&WindowParam> { self.window_param.default_window() }
    fn with_default_window(self, default_window : Option<WindowParam>) -> Self { Self { window_param: self.window_param.with_default_window(default_window) } }
}

pub trait AppRun : AppLoop
{
    fn run(&mut self) -> Result<(), ()> { self.run_with_param(___()) }

    fn run_with_param<'a>(&'a mut self, param : AppRunParam) -> Result<(), ()>
    {
        let mut runner = AppRunner { ctx: ___(), data: self };
        let r = <AppRunner<'a, Self> as WindowRun>::run_with_param(&mut runner, param.window_param);

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
