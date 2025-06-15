use crate::*;

/// Represents raw hardware events that are not associated with any particular window.
///
/// Note that these events are delivered regardless of input focus.
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DeviceMessage
{
    Added, Removed,
    Resume,
    Update,
    MemoryWarning,
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
            DeviceMessage::MemoryWarning => write!(f, "MemoryWarning"),
        }
    }
}