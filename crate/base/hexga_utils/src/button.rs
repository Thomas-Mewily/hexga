use super::*;

pub mod prelude
{
    pub use super::{
        ButtonEvolution, ButtonEvolutionExtension, ButtonRepeat, ButtonRepeatExtension,
        ButtonState, ButtonStateEvolution, ButtonStateExtension, ButtonToggle,
        ButtonToggleExtension,
    };
}

pub type ButtonEvolution = PreviousValue<ButtonState>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonToggle
{
    /// False
    #[default]
    Hold,
    /// True
    Toggle,
}
pub trait ButtonToggleExtension: Has<ButtonToggle>
{
    fn is_hold(&self) -> bool { self.retrieve().is_hold() }
    fn is_toggle(&self) -> bool { self.retrieve().is_toggle() }
}
impl<T> ButtonToggleExtension for T where T: Has<ButtonToggle> {}
impl ButtonToggle
{
    pub const fn is_hold(self) -> bool { matches!(self, Self::Hold) }
    pub const fn is_toggle(self) -> bool { matches!(self, Self::Toggle) }
}
impl From<bool> for ButtonToggle
{
    fn from(value: bool) -> Self { if value { Self::Toggle } else { Self::Hold } }
}
impl From<ButtonToggle> for bool
{
    fn from(value: ButtonToggle) -> Self { value.is_toggle() }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonState
{
    /// True
    #[default]
    Down,
    /// False
    Up,
}
pub trait ButtonStateExtension: Has<ButtonState>
{
    fn is_up(&self) -> bool { self.retrieve().is_up() }
    fn is_down(&self) -> bool { self.retrieve().is_down() }
}
impl<T> ButtonStateExtension for T where T: Has<ButtonState> {}
impl ButtonState
{
    pub const fn is_up(self) -> bool { matches!(self, ButtonState::Up) }
    pub const fn is_down(self) -> bool { matches!(self, ButtonState::Down) }
}
impl From<bool> for ButtonState
{
    fn from(value: bool) -> Self { if value { Self::Down } else { Self::Up } }
}
impl From<ButtonState> for bool
{
    fn from(value: ButtonState) -> Self { value.is_down() }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ButtonRepeat
{
    #[default]
    NotRepeated,
    Repeated,
}
pub trait ButtonRepeatExtension: Has<ButtonRepeat>
{
    fn is_repeated(&self) -> bool { self.retrieve().is_repeated() }
    fn is_not_repeated(&self) -> bool { self.retrieve().is_not_repeated() }
}
impl<T> ButtonRepeatExtension for T where T: Has<ButtonRepeat> {}
impl ButtonRepeat
{
    pub const fn is_repeated(self) -> bool { matches!(self, ButtonRepeat::Repeated) }
    pub const fn is_not_repeated(self) -> bool { matches!(self, ButtonRepeat::NotRepeated) }
}
impl From<bool> for ButtonRepeat
{
    fn from(value: bool) -> Self
    {
        if value
        {
            Self::Repeated
        }
        else
        {
            Self::NotRepeated
        }
    }
}
impl From<ButtonRepeat> for bool
{
    fn from(value: ButtonRepeat) -> Self { value.is_repeated() }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ButtonStateEvolution
{
    Down,
    Up,
    /// Can be use for toggle
    Pressed,
    /// Can be use for toggle
    Released,
}
impl From<ButtonState> for ButtonStateEvolution
{
    fn from(value: ButtonState) -> Self
    {
        match value
        {
            ButtonState::Down => Self::Down,
            ButtonState::Up => Self::Up,
        }
    }
}
impl Evolution<ButtonState> for ButtonStateEvolution
{
    fn value(&self) -> ButtonState { matches!(self, Self::Down | Self::Pressed).into() }
    fn old_value(&self) -> ButtonState { matches!(self, Self::Down | Self::Released).into() }
}

pub trait ButtonEvolutionExtension: Evolution<ButtonState>
{
    fn is_down(&self) -> bool;
    fn was_down(&self) -> bool;

    fn is_up(&self) -> bool { !self.is_down() }
    fn was_up(&self) -> bool { !self.was_down() }

    /// Pull up, `false` to `true`, `0` to `1`,
    fn is_pressed(&self) -> bool { self.is_down() && (!self.was_down()) }
    /// Pull down, `true` to `false`, `1` to `0`
    fn is_released(&self) -> bool { self.was_down() && (!self.is_down()) }

    fn is_toggled(&self) -> bool { self.is_down() != self.was_down() }
    fn is_constant(&self) -> bool { self.is_down() == self.was_down() }

    fn is_hold(&self) -> bool { self.is_constant() && self.is_down() }

    fn evolution(&self) -> ButtonStateEvolution
    {
        match (self.is_down(), self.was_down())
        {
            (true, true) => ButtonStateEvolution::Down,
            (true, false) => ButtonStateEvolution::Pressed,
            (false, true) => ButtonStateEvolution::Released,
            (false, false) => ButtonStateEvolution::Up,
        }
    }
}

impl<T> ButtonEvolutionExtension for T
where
    T: Evolution<ButtonState>,
{
    fn is_down(&self) -> bool { self.value().is_down() }
    fn was_down(&self) -> bool { self.old_value().is_down() }
}
