use super::*;


pub trait IAppEvent : 'static + Debug + Send {}
impl<T> IAppEvent for T where T: 'static + Debug + Send {}

pub trait App: 'static
{
    type CustomEvent : IAppEvent;
    fn handle_event(&mut self, ev: AppEvent<Self::CustomEvent>, ctx: &mut Ctx) { self.dispatch_event(ev, ctx); }
    fn dispatch_event(&mut self, ev: AppEvent<Self::CustomEvent>, ctx: &mut Ctx)
    {
        match ev
        {
            AppEvent::Flow(f) => self.handle_flow(f, ctx),
            AppEvent::Input(i) => self.handle_input(i, ctx),
            AppEvent::Custom(c) => self.handle_custom(c, ctx),
        }
    }

    fn handle_flow(&mut self, flow: FlowEvent, ctx: &mut Ctx) { self.dispatch_flow(flow, ctx); }
    fn dispatch_flow(&mut self, flow: FlowEvent, ctx: &mut Ctx)
    {
        match flow
        {
            FlowEvent::Resumed => self.resumed(ctx),
            FlowEvent::Paused => self.paused(ctx),
            FlowEvent::Update => self.update(ctx),
            FlowEvent::Draw => self.draw(ctx),
            FlowEvent::Exit => self.exit(ctx),
        }
    }


    fn handle_custom(&mut self, custom: Self::CustomEvent, ctx: &mut Ctx) { let _ = (custom, ctx); }

    fn update(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }


    fn paused(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn resumed(&mut self, ctx: &mut Ctx) { let _ = ctx; }


    fn handle_input(&mut self, input: InputEvent, ctx: &mut Ctx) { let _ = (ctx, input); }
    fn exit(&mut self, ctx: &mut Ctx) { let _ = ctx; }

}


