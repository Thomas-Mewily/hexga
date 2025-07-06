use crate::*;
pub use hexga_engine_events::modules::*;

#[derive(Clone, PartialEq)]
pub enum EventMessage<UserEvent,WindowData>
{
    LocalizedEvent(LocalizedEvent<WindowData>),
    Device(DeviceMessage),
    User(UserEvent),
}

impl<T,W> EventMessage<T,W>
{
    pub fn as_localized_event(&self) -> Option<&LocalizedEvent<W>> { if let EventMessage::LocalizedEvent(event) = self { Some(event) } else { None } }
    pub fn as_localized_event_mut(&mut self) -> Option<&mut LocalizedEvent<W>> { if let EventMessage::LocalizedEvent(event) = self { Some(event) } else { None } }

    pub fn as_device_message(&self) -> Option<&DeviceMessage> { if let EventMessage::Device(message) = self { Some(message) } else { None } }
    pub fn as_device_message_mut(&mut self) -> Option<&mut DeviceMessage> { if let EventMessage::Device(message) = self { Some(message) } else { None } }

    pub fn as_user(&self) -> Option<&T> { if let EventMessage::User(user) = self { Some(user) } else { None } }
    pub fn as_user_mut(&mut self) -> Option<&mut T> { if let EventMessage::User(user) = self { Some(user) } else { None } }

    pub fn clone_with_user_message<T2>(&self, user: T2) -> EventMessage<T2,W>
    {
        match self
        {
            EventMessage::LocalizedEvent(event) => EventMessage::LocalizedEvent(event.clone()),
            EventMessage::Device(message) => EventMessage::Device(*message),
            EventMessage::User(_) => EventMessage::User(user),
        }
    }
    pub fn with_replaced_user_message<T2>(self, user: T2) -> EventMessage<T2,W>
    {
        match self {
            EventMessage::LocalizedEvent(event) => EventMessage::LocalizedEvent(event),
            EventMessage::Device(message) => EventMessage::Device(message),
            EventMessage::User(_) => EventMessage::User(user),
        }
    }
    pub fn with_window_data_type<W2>(self) -> EventMessage<T,W2>
    {
        match self
        {
            EventMessage::LocalizedEvent(event) => EventMessage::LocalizedEvent(event.with_data_type()),
            EventMessage::Device(message) => EventMessage::Device(message),
            EventMessage::User(user) => EventMessage::User(user),
        }
    }
}


impl<T,W> From<LocalizedEvent<W>> for EventMessage<T,W> { fn from(value: LocalizedEvent<W>) -> Self { Self::LocalizedEvent(value) } }
impl<T,W> From<DeviceMessage>  for EventMessage<T,W> { fn from(value: DeviceMessage)  -> Self { Self::Device(value) } }

impl<T,W> IDeviceMessage for EventMessage<T,W>
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