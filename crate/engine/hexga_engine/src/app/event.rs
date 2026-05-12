use hexga_event_loop::event_loop::EventLoopProxy;
use hexga_graphics::gpu::GpuContext;

use super::*;

pub type AppEvent = PlatformEvent<()>;

pub(crate) type AppInternalEvent = PlatformEvent<AppCustomEvent>;
pub(crate) type AppInternalProxy = EventLoopProxy<AppCustomEvent>;
pub(crate) type AppInternalEventLoop<'a> = EventLoop<'a, AppCustomEvent>;

pub(crate) enum AppCustomEvent
{
    SurfaceReady(GpuSurface<'static>),
    GpuReady(GpuContext),
    GpuError(GpuError),
}
