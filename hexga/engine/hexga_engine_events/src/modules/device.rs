use crate::*;

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DeviceEvent
{
    Added, Removed,
    Resume,
    Draw,
}
impl Debug for DeviceEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            DeviceEvent::Added => write!(f, "Added"),
            DeviceEvent::Removed => write!(f, "Removed"),
            DeviceEvent::Resume => write!(f, "Resume"),
            DeviceEvent::Draw => write!(f, "Draw"),
        }
    }
}