use crate::*;


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum WindowEvent
{
    Resize(Point2),
    Move  (Vec2),

    Focus(bool),

    /// The window has been occluded (completely hidden from view).
    ///
    /// This is different to window visibility as it depends on whether the window is closed,
    /// minimised, set invisible, or fully occluded by another window.
    Visible(bool),

    Draw,

    Quit,

    //DropFile(PathBuf), //(DropFileEvent),
    // HoverFile(PathBuf)
    //HoverFileCancel
}

/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DropFileEvent
{
    pub path : std::path::PathBuf,
    pub data : Vec<u8>,
}
    */