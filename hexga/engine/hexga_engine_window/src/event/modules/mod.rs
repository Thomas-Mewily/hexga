use crate::*;
pub use hexga_engine_events::modules::*;

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq)]
pub struct LocalizedEvent
{
    pub window : Option<WindowID>,
    pub event  : Event,
    pub device : DeviceID,
}

impl LocalizedEvent
{
    pub fn new(window: Option<WindowID>, event: Event, device: DeviceID) -> Self {
        Self { window, event, device }
    }

    pub fn new_os(event: Event) -> Self {
        Self { window: None, event, device: DeviceID::OS }
    }

    pub fn with_window(self, window: Option<WindowID>) -> Self {
        Self { window: window, ..self }
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
impl From<Event> for LocalizedEvent
{
    fn from(value: Event) -> Self {
        Self::new_os(value)
    }
}

impl From<WindowEvent> for LocalizedEvent { fn from(value: WindowEvent) -> Self { Self::new_os(value.into()) } }
impl From<MouseMoveEvent  > for LocalizedEvent { fn from(value: MouseMoveEvent) -> Self { Self::new_os(value.into()) } }
impl From<MouseButtonEvent> for LocalizedEvent { fn from(value: MouseButtonEvent) -> Self { Self::new_os(value.into()) } }
impl From<MouseEvent > for LocalizedEvent { fn from(value: MouseEvent) -> Self { Self::new_os(value.into()) } }
impl From<KeyEvent   > for LocalizedEvent { fn from(value: KeyEvent) -> Self { Self::new_os(value.into()) } }
impl From<TouchEvent > for LocalizedEvent { fn from(value: TouchEvent) -> Self { Self::new_os(value.into()) } }
impl From<DeviceEvent> for LocalizedEvent { fn from(value: DeviceEvent) -> Self { Self::new_os(value.into()) } }

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceID(usize);

impl DeviceID
{
    pub const OS : Self = DeviceID(0);
}