use crate::*;
pub use hexga_engine_events::modules::*;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq)]
pub struct LocalizedEvent
{
    pub window : WindowID,
    pub event  : Event,
    pub device : DeviceID,
}
impl std::ops::Deref for LocalizedEvent
{
    type Target = Event;
    fn deref(&self) -> &Self::Target { &self.event }
}
impl std::ops::DerefMut for LocalizedEvent
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.event }
}


impl LocalizedEvent
{
    pub fn new(window: WindowID, event: Event, device: DeviceID) -> Self {
        Self { window, event, device }
    }
    pub fn with_window(self, window: WindowID) -> Self {
        Self { window, ..self }
    }
    pub fn with_device(self, device: DeviceID) -> Self {
        Self { device, ..self }
    }
}

impl Into<Event> for LocalizedEvent
{
    fn into(self) -> Event {
        self.event
    }
}