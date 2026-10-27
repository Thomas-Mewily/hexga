use hexga_event_loop::app::{AppProxy, AppFlow};
//use hexga_graphics::gpu::GpuContext;

use super::*;

pub(crate) type AppInternalEventLoop<'a> = EventLoop<'a, AppCustomEvent>;
pub(crate) type AppInternalProxy = AppProxy<AppCustomEvent>;
pub(crate) type AppInternalMessage = AppMessage<AppCustomEvent>;

pub(crate) enum AppCustomEvent
{
    SurfaceReady(GpuSurface<'static>),
    GpuReady(GpuContext),
    GpuError(GpuError),
}

/*
pub type AppEvent = PlatformEvent;
pub type AppFlow = PlatformFlow;


#[derive(Clone, PartialEq, Debug)]
pub enum AppMessage<Event=AppEvent>
{
    Event(Event),
    Flow(AppFlow),
}


*/