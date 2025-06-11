use crate::*;


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MouseEvent
{
    Enter (bool),
    Move  (MouseMoveEvent),
    Wheel (Vec2),
    Button(MouseButtonEvent),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseMoveEvent
{
    pub position : Vec2,
    pub delta    : Vec2
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseButtonEvent
{
    pub position : Vec2,
    pub button   : MouseButton,
    pub press    : bool,
}

#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum MouseButton
{
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Unknow(u16),
}
impl MouseButton
{
    pub fn is_left   (&self) -> bool { matches!(self, Self::Left   ) }
    pub fn is_right  (&self) -> bool { matches!(self, Self::Right  ) }
    pub fn is_middle (&self) -> bool { matches!(self, Self::Middle ) }
    pub fn is_back   (&self) -> bool { matches!(self, Self::Back   ) }
    pub fn is_forward(&self) -> bool { matches!(self, Self::Forward) }
    pub fn is_unknow (&self) -> bool { matches!(self, Self::Unknow(_)) }
}