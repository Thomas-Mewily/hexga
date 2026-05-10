use hexga_event_loop::event_loop::*;

use super::*;

pub(crate) struct EngineCtx
{

}

impl PlatformEventHandler for EngineCtx
{
    fn update(&mut self, dt: Duration, event_loop: &mut EventLoop<()>) { let _ = dt; }

    fn draw(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn resumed(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn paused(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn exit(&mut self, event_loop: &mut EventLoop<()>) { let _ = event_loop; }

    fn event(&mut self, ev: Event<()>, event_loop: &mut EventLoop<()>) -> Option<Event<()>> { Some(ev) }
}