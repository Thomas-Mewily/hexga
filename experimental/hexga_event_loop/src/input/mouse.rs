use super::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct MouseWheel
{
    pub delta : Vec2,
    pub phase : TouchPhase
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseClick
{
    pub button : MouseButton,
    pub state : ButtonState,
}

/// Describes a button of a mouse controller.
///
/// ## Platform-specific
///
/// **macOS:** `Back` and `Forward` might not work with all hardware.
/// **Orbital:** `Back` and `Forward` are unsupported due to orbital not supporting them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TouchPhase {
    Begin,
    Move,
    End,
    Cancel,
}

pub trait TouchPhaseExtension
{
    fn is_begin(&self) -> bool;
    fn is_move(&self) -> bool;
    fn is_end(&self) -> bool;
    fn is_cancel(&self) -> bool;
}

impl TouchPhaseExtension for MouseWheel
{
    fn is_begin(&self) -> bool {
        self.phase.is_begin()
    }

    fn is_move(&self) -> bool {
        self.phase.is_move()
    }

    fn is_end(&self) -> bool {
        self.phase.is_end()
    }

    fn is_cancel(&self) -> bool {
        self.phase.is_cancel()
    }
}

impl TouchPhaseExtension for TouchPhase
{
    fn is_begin(&self) -> bool {
        matches!(self, Self::Begin)
    }

    fn is_move(&self) -> bool {
        matches!(self, Self::Move)
    }

    fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }

    fn is_cancel(&self) -> bool {
        matches!(self, Self::Cancel)
    }
}