use crate::*;


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum MouseEvent
{
    Enter (bool),
    Move  (MouseMoveEvent),
    Wheel (Vec2),
    Button(MouseButtonEvent),
}

impl Debug for MouseEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Enter(v) => f.debug_tuple("MouseEnter").field(v).finish(),
            Self::Move(v) => write!(f, "{:?}", v),
            Self::Wheel(v) => f.debug_tuple("MouseWheel").field(v).finish(),
            Self::Button(v) => write!(f, "{:?}", v),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseMoveEvent
{
    pub position : Vec2,
    pub delta    : Vec2
}

impl Debug for MouseMoveEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MouseMove").field("position", &self.position).field("delta", &self.delta).finish()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseButtonEvent
{
    pub position : Vec2,
    pub button   : MouseButton,
    pub action   : EventAction,
}

impl Debug for MouseButtonEvent
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.debug_struct("MouseButton").field("position", &self.position).field("button", &self.button).field("action", &self.action).finish()
    }
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