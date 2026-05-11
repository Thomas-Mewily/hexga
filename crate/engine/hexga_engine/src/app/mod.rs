use super::*;

mod application;
pub use application::*;

mod param;
pub use param::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

pub mod prelude
{
    pub use super::{AppParam,AppEvent,AppResult,AppError};
    pub use super::traits::*;

    pub(crate) use super::{AppInternalEvent, AppCustomEvent,
        AppInternalProxy,AppInternalEventLoop
    };
    
}

pub mod traits
{
    pub use super::{App,AppRun};
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
