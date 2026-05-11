use hexga_event_loop::event_loop::EventLoopProxy;

use super::*;


pub type AppEvent = PlatformEvent<()>;

pub(crate) type AppInternalEvent = PlatformEvent<AppCustomEvent>;
pub(crate) type AppInternalProxy = EventLoopProxy<AppCustomEvent>;
pub(crate) type AppInternalEventLoop<'a> = EventLoop<'a, AppCustomEvent>;

pub(crate) enum AppCustomEvent
{
    GpuReady,
    GpuError(GpuError),
}

