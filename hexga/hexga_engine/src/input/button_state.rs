use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ButtonStateChange
{
    Pressed,
    Released,
    /// Can be use for toggle
    JustPressed,
    /// Can be use for toggle
    JustReleased,
}
impl<T> Evolution<bool,T> for ButtonStateChange where T:Copy+Default
{
    fn value(&self) -> bool {
        matches!(self, Self::Pressed | Self::JustPressed)
    }

    fn old_value(&self) -> bool {
        matches!(self, Self::Pressed | Self::JustReleased)
    }

    fn last_time_changed(&self) -> T {
        ___()
    }

    fn set_at(&mut self, cur : bool, _time : T) where bool:PartialEq 
    {
        let value = Evolution::<bool,T>::value(self);
        *self = match (value, cur)
        {
            (true, true) => Self::Pressed,
            (true, false) => Self::JustReleased,
            (false, true) => Self::JustPressed,
            (false, false) => Self::JustReleased,
        };
    }
}