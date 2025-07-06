use crate::*;
pub use hexga_engine_events::modules::*;

#[non_exhaustive]
pub struct LocalizedEvent<WindowData=()>
{
    pub window : WindowID<WindowData>,
    pub event  : Event,
    pub device : DeviceID,
}

impl<W> Clone for LocalizedEvent<W>
{
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            event: self.event.clone(),
            device: self.device,
        }
    }
}
impl<W> Copy for LocalizedEvent<W> {}
impl<W> PartialEq for LocalizedEvent<W>
{
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window && self.event == other.event && self.device == other.device
    }
}

impl<W> std::ops::Deref for LocalizedEvent<W>
{
    type Target = Event;
    fn deref(&self) -> &Self::Target { &self.event }
}
impl<W> std::ops::DerefMut for LocalizedEvent<W>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.event }
}


impl<W> LocalizedEvent<W>
{
    pub fn new(window: WindowID<W>, event: Event, device: DeviceID) -> Self {
        Self { window, event, device }
    }
    pub fn with_window(self, window: WindowID<W>) -> Self {
        Self { window, ..self }
    }
    pub fn with_device(self, device: DeviceID) -> Self {
        Self { device, ..self }
    }

    pub fn with_data_type<W2>(self) -> LocalizedEvent<W2> {
        LocalizedEvent {
            window: WindowID::from_other_id(self.window),
            event: self.event,
            device: self.device,
        }
    }
}

impl<W> Into<Event> for LocalizedEvent<W>
{
    fn into(self) -> Event {
        self.event
    }
}