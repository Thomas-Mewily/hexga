use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonToggle
{
    /// False
    #[default]
    Hold,
    /// True
    Toggle,
}
impl ButtonToggle
{
    pub const fn is_hold(self) -> bool { matches!(self, Self::Hold) }
    pub const fn is_toggle(self) -> bool { matches!(self, Self::Toggle) }
}
impl From<bool> for ButtonToggle
{
    fn from(value: bool) -> Self {
        if value { Self::Toggle } else { Self::Hold }
    }
}
impl From<ButtonToggle> for bool
{
    fn from(value: ButtonToggle) -> Self {
        value.is_toggle()
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonState
{
    /// True
    #[default]
    Down,
    /// False
    Up,
}
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
impl IEvolution<ButtonState> for ButtonStateEvolution
{
    fn value(&self) -> ButtonState { matches!(self, Self::Down | Self::Pressed).into() }
    fn old_value(&self) -> ButtonState { matches!(self, Self::Down | Self::Released).into() }
}





#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ButtonRepeat
{
    #[default]
    NotRepeated,
    Repeated,
}
pub trait IsRepetable
{
    fn is_repeated(&self) -> bool;
    fn is_not_repeated(&self) -> bool { !self.is_repeated() }
}
impl IsRepetable for ButtonRepeat
{
    fn is_repeated(&self) -> bool { matches!(self, ButtonRepeat::Repeated) }
    fn is_not_repeated(&self) -> bool { matches!(self, ButtonRepeat::NotRepeated) }
}