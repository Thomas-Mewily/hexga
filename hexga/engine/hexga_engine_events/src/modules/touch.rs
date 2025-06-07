use crate::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchEvent
{
    pub phase    : TouchPhase,
    pub id       : TouchID,
    pub position : Vec2,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TouchID { pub index : u64 }
impl TouchID { pub const fn new(index : u64) -> Self { Self { index }}}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase
{
    Begin,
    Move,
    End,
    Cancel,
}

impl TouchPhase
{
    pub fn is_start (&self) -> bool { matches!(self, Self::Begin ) }
    pub fn is_move  (&self) -> bool { matches!(self, Self::Move  ) }
    pub fn is_end   (&self) -> bool { matches!(self, Self::End   ) }
    pub fn is_cancel(&self) -> bool { matches!(self, Self::Cancel) }
}
