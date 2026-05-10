use hexga_event_loop::event_loop::PlatformEventHandler;

use super::*;

pub(crate) struct AppCtx
{

}

impl PlatformEventHandler for AppCtx
{
    fn update(&mut self, dt: Duration, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) { let _ = dt; }

    fn draw(&mut self, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) { let _ = event_loop; }

    fn resumed(&mut self, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) { let _ = event_loop; }

    fn paused(&mut self, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) { let _ = event_loop; }

    fn exit(&mut self, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) { let _ = event_loop; }

    fn event(&mut self, ev: hexga_event_loop::prelude::Event<()>, event_loop: &mut hexga_event_loop::prelude::EventLoop<()>) -> Option<hexga_event_loop::prelude::Event<()>> { Some(ev) }

    fn run_event_loop_with_param(self, param: hexga_event_loop::event_loop::EventLoopParam) -> hexga_event_loop::event_loop::EventLoopResult 
    {
        hexga_event_loop::event_loop::run(self, param)
    }
}