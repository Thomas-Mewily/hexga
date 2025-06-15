use crate::*;
pub use hexga_engine_events::modules::*;

mod device;
pub use device::*;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq)]
pub enum AppMessage
{
    LocalizedEvent(LocalizedEvent),
    Device(DeviceMessage),
}


impl From<LocalizedEvent> for AppMessage { fn from(value: LocalizedEvent) -> Self { Self::LocalizedEvent(value) } }
impl From<DeviceMessage>  for AppMessage { fn from(value: DeviceMessage)  -> Self { Self::Device(value) } }
