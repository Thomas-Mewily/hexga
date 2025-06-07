use crate::*;


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowEvent
{
    Resize(Vec2),
    Minimized,
    Restored,
    Quit,
    DropFile, //(DropFileEvent),
}

/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropFileEvent
{
    pub path : std::path::PathBuf,
    pub data : Vec<u8>,
}
    */