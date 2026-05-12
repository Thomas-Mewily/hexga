use super::*;

pub mod prelude
{
    pub(crate) use super::Monitor;
}

#[derive(Debug, Clone)]
pub struct Monitor
{
    handle: WinitMonitorHandle,
}
impl From<WinitMonitorHandle> for Monitor
{
    fn from(handle: WinitMonitorHandle) -> Self { Self { handle } }
}
impl From<Monitor> for WinitMonitorHandle
{
    fn from(value: Monitor) -> Self { value.handle }
}

impl Monitor
{
    pub fn name(&self) -> Option<String> { self.handle.name() }
    pub fn refresh_rate_millihertz(&self) -> Option<u32> { self.handle.refresh_rate_millihertz() }
    pub fn scale_factor(&self) -> float { self.handle.scale_factor() as float }
    pub fn scale(&self) -> Vec2 { self.scale_factor().splat2() }

    /// Expose external lib impl details
    #[doc(hidden)]
    pub fn winit(&self) -> &WinitMonitorHandle { &self.handle }
}
impl GetPosition<int, 2> for Monitor
{
    fn pos(&self) -> Vector<int, 2> { self.handle.position().convert() }
}
impl GetSize<int, 2> for Monitor
{
    fn size(&self) -> Vector<int, 2> { self.handle.size().convert() }
}

pub(crate) type WinitMonitorHandle = winit::monitor::MonitorHandle;
