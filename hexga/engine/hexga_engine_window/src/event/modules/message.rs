use crate::*;
pub use hexga_engine_events::modules::*;

#[derive(Clone, PartialEq)]
pub enum EventMessage<T=()>
{
    LocalizedEvent(LocalizedEvent),
    Device(DeviceMessage),
    User(T),
}

impl<T> EventMessage<T>
{
    pub fn as_localized_event(&self) -> Option<&LocalizedEvent> { if let EventMessage::LocalizedEvent(event) = self { Some(event) } else { None } }
    pub fn as_localized_event_mut(&mut self) -> Option<&mut LocalizedEvent> { if let EventMessage::LocalizedEvent(event) = self { Some(event) } else { None } }

    pub fn as_device_message(&self) -> Option<&DeviceMessage> { if let EventMessage::Device(message) = self { Some(message) } else { None } }
    pub fn as_device_message_mut(&mut self) -> Option<&mut DeviceMessage> { if let EventMessage::Device(message) = self { Some(message) } else { None } }

    pub fn as_user(&self) -> Option<&T> { if let EventMessage::User(user) = self { Some(user) } else { None } }
    pub fn as_user_mut(&mut self) -> Option<&mut T> { if let EventMessage::User(user) = self { Some(user) } else { None } }
}


impl<T> From<LocalizedEvent> for EventMessage<T> { fn from(value: LocalizedEvent) -> Self { Self::LocalizedEvent(value) } }
impl<T> From<DeviceMessage>  for EventMessage<T> { fn from(value: DeviceMessage)  -> Self { Self::Device(value) } }

impl<T> IDeviceMessage for EventMessage<T>
{
    fn is_added(&self) -> bool { self.as_device_message().map(|m| m.is_added()).unwrap_or_default() }
    fn is_removed(&self) -> bool { self.as_device_message().map(|m| m.is_removed()).unwrap_or_default() }
    fn is_resume(&self) -> bool { self.as_device_message().map(|m| m.is_resume()).unwrap_or_default() }
    fn is_update(&self) -> bool { self.as_device_message().map(|m| m.is_update()).unwrap_or_default() }
    fn is_exit(&self) -> bool { self.as_device_message().map(|m| m.is_exit()).unwrap_or_default() }
    fn is_memory_warning(&self) -> bool { self.as_device_message().map(|m| m.is_memory_warning()).unwrap_or_default() }
}


/// Represents raw hardware events that are not associated with any particular window.
///
/// Note that these message are delivered regardless of input focus.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DeviceMessage
{
    Added, Removed,
    Resume,
    Update,
    Exit,
    MemoryWarning,
}
pub trait IDeviceMessage
{
    fn is_added(&self) -> bool;
    fn is_removed(&self) -> bool;
    fn is_resume(&self) -> bool;
    fn is_update(&self) -> bool;
    fn is_exit(&self) -> bool;
    fn is_memory_warning(&self) -> bool;
}
impl IDeviceMessage for DeviceMessage
{
    fn is_added(&self) -> bool { matches!(self, DeviceMessage::Added) }
    fn is_removed(&self) -> bool { matches!(self, DeviceMessage::Removed) }
    fn is_resume(&self) -> bool { matches!(self, DeviceMessage::Resume) }
    fn is_update(&self) -> bool { matches!(self, DeviceMessage::Update) }
    fn is_exit(&self) -> bool { matches!(self, DeviceMessage::Exit) }
    fn is_memory_warning(&self) -> bool { matches!(self, DeviceMessage::MemoryWarning) }
}
impl std::fmt::Debug for DeviceMessage
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            DeviceMessage::Added => write!(f, "Added"),
            DeviceMessage::Removed => write!(f, "Removed"),
            DeviceMessage::Resume => write!(f, "Resume"),
            DeviceMessage::Update => write!(f, "Update"),
            DeviceMessage::Exit => write!(f, "Exit"),
            DeviceMessage::MemoryWarning => write!(f, "MemoryWarning"),
        }
    }
}