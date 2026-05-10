use hexga_event_loop::event_loop::*;

use super::*;

pub type AppEvent = PlatformEvent<()>;

pub trait App<Event,Ctx=()>
{
    fn event(&mut self, ev: Event, ctx: &mut Ctx) -> Option<Event>;
    fn update(&mut self, dt: Duration, ctx: &mut Ctx); 
    fn draw(&mut self, coef: float, ctx: &mut Ctx);
}

/*
pub(crate) struct AppCtx<A>
    where A:
{
    app : A,
}

impl PlatformEventHandler for AppCtx
{
    fn update(&mut self, dt: Duration, event_loop: &mut EventLoop<()>) 
    { 
        let _ = dt; 
    }

    fn draw(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn resumed(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn paused(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn exit(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn event(&mut self, ev: PlatformEvent<()>, event_loop: &mut EventLoop<()>) -> Option<PlatformEvent<()>> { Some(ev) }
}
*/
