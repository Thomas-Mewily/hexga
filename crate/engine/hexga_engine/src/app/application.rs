use super::*;


pub trait App<Event=AppEvent,Ctx=AppCtx>
{
    fn event(&mut self, ev: Event, ctx: &mut Ctx) -> Option<Event> { let _ = ev; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut Ctx) {}
    fn suspended(&mut self, ctx: &mut Ctx) {}
}
