use hexga_event_loop::app::{EventLoopProxy, PlatformFlow};
//use hexga_graphics::gpu::GpuContext;

use super::*;

pub(crate) type AppInternalEventLoop<'a> = EventLoop<'a, AppCustomEvent>;
pub(crate) type AppInternalProxy = EventLoopProxy<AppCustomEvent>;

pub(crate) enum AppCustomEvent
{
    SurfaceReady(GpuSurface<'static>),
    GpuReady(GpuContext),
    GpuError(GpuError),
}

/*
pub type AppEvent = PlatformEvent;
pub type AppFlow = PlatformFlow;

pub(crate) type AppInternalEvent = AppMessage<AppCustomEvent>;

#[derive(Clone, PartialEq, Debug)]
pub enum AppMessage<Event=AppEvent>
{
    Event(Event),
    Flow(AppFlow),
}


*/