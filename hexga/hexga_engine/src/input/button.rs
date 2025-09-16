use super::*;


#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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
pub trait ButtonToggleExtension: Getter<ButtonToggle>
{
    fn is_hold(&self) -> bool { self.get().is_hold() }
    fn is_toggle(&self) -> bool { self.get().is_toggle() }
}
impl<T> ButtonToggleExtension for T where T: Getter<ButtonToggle> {}
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


#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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
pub trait ButtonStateExtension: Getter<ButtonState>
{
    fn is_up(&self) -> bool { self.get().is_up() }
    fn is_down(&self) -> bool { self.get().is_down() }
}
impl<T> ButtonStateExtension for T where T: Getter<ButtonState> {}
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

#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ButtonRepeat
{
    #[default]
    NotRepeated,
    Repeated,
}
pub trait ButtonRepeatExtension: Getter<ButtonRepeat>
{
    fn is_repeated(&self) -> bool { self.get().is_repeated() }
    fn is_not_repeated(&self) -> bool { self.get().is_not_repeated() }
}
impl<T> ButtonRepeatExtension for T where T: Getter<ButtonRepeat> {}
impl ButtonRepeat
{
    pub const fn is_repeated(self) -> bool { matches!(self, ButtonRepeat::Repeated) }
    pub const fn is_not_repeated(self) -> bool { matches!(self, ButtonRepeat::NotRepeated) }
}
impl From<bool> for ButtonRepeat
{
    fn from(value: bool) -> Self { if value { Self::Repeated } else { Self::NotRepeated } }
}
impl From<ButtonRepeat> for bool
{
    fn from(value: ButtonRepeat) -> Self { value.is_repeated() }
}