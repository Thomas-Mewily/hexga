use super::*;

#[derive(Default)]
pub struct AppCtx
{
    pub(crate) window : Window,
    pub(crate) graphics : Option<AppGraphics>,
}

impl AppContext for AppCtx
{
    fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn graphics(&mut self) -> &mut Option<AppGraphics> 
    {
        &mut self.graphics
    }
}

pub mod experimental
{
    pub(crate) use super::*;

    pub trait AppContext
    {
        fn window(&mut self) -> &mut Window;

        fn graphics(&mut self) -> &mut Option<AppGraphics>;
    }
}
use experimental::*;


pub trait App<Event=AppEvent,Ctx=AppCtx>
{
    fn message(&mut self, msg: AppMessage, ctx: &mut Ctx) 
    { 
        match msg
        {
            AppMessage::Event(event) => self.event(event, ctx),
            AppMessage::Flow(flow) => self.flow(flow, ctx),
        }
    }
    fn event(&mut self, ev: AppEvent, ctx: &mut Ctx) { let _ = ev; }
    fn flow(&mut self, flow: AppFlow, ctx: &mut Ctx) 
    {
        match flow
        {
            AppFlow::Resumed => self.resumed(ctx),
            AppFlow::Suspended => self.suspended(ctx),
            AppFlow::Update(dt) => self.update(dt, ctx),
            AppFlow::Draw => self.draw(ctx),
        }
    }


    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut Ctx) {}
    fn suspended(&mut self, ctx: &mut Ctx) {}
}
