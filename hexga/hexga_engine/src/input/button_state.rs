use hexga::core::default;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum ButtonState
{
    #[default]
    Released,
    Pressed,
}
impl ButtonState
{
    pub fn is_pressed(self) -> bool { matches!(self, ButtonState::Pressed) }
    pub fn is_released(self) -> bool { matches!(self, ButtonState::Released) }
}
impl From<bool> for ButtonState
{
    fn from(value: bool) -> Self { if value { Self::Pressed } else { Self::Released } }
}
impl From<ButtonState> for bool
{
    fn from(value: ButtonState) -> Self { value.is_pressed() }
}



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ButtonStateEvolution
{
    Pressed,
    Released,
    /// Can be use for toggle
    JustPressed,
    /// Can be use for toggle
    JustReleased,
}
impl<T> IEvolution<ButtonState,T> for ButtonStateEvolution where T:Copy+Default
{
    fn value(&self) -> ButtonState {
        matches!(self, Self::Pressed | Self::JustPressed).into()
    }

    fn old_value(&self) -> ButtonState {
        matches!(self, Self::Pressed | Self::JustReleased).into()
    }

    fn last_time_changed(&self) -> T {
        ___()
    }

    fn set_at(&mut self, cur : ButtonState, _time : T) where bool:PartialEq 
    {
        let old = <ButtonStateEvolution as evolution::IEvolution<button_state::ButtonState, T>>::old_value(self);
        *self = match (old, cur)
        {
            (ButtonState::Released, ButtonState::Released) => ButtonStateEvolution::Released,
            (ButtonState::Released, ButtonState::Pressed) => ButtonStateEvolution::JustPressed,
            (ButtonState::Pressed, ButtonState::Released) => ButtonStateEvolution::JustReleased,
            (ButtonState::Pressed, ButtonState::Pressed) => ButtonStateEvolution::Pressed,
        };
    }
}


pub trait EvolutionButtonStateIterator<T> : Iterator + Sized where Self::Item: EvolutionButtonState<T>, T:Copy+Default
{
    fn pressed(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_pressed()) }
    fn released(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_released()) }

    fn just_pressed(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_just_pressed()) }
    fn just_released(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_just_released()) }
}
impl<T,S> EvolutionButtonStateIterator<T> for S where S: Iterator + Sized, S::Item: EvolutionButtonState<T>, T:Copy+Default{}
pub trait EvolutionButtonState<T> : IEvolution<ButtonState,T> where T:Copy+Default
{
    fn is_pressed(&self) -> bool;
    fn was_pressed(&self) -> bool;

    /// `false` to `true`, `0` to `1`
    fn is_pull_up(&self) -> bool { self.is_pressed() && (!self.was_pressed()) }
    /// `true` to `false`, `1` to `0`
    fn is_pull_down(&self) -> bool { self.was_pressed() && (!self.is_pressed()) }

    fn is_pull_changed(&self) -> bool { self.is_pressed() != self.was_pressed() }
    fn is_pull_constant(&self) -> bool { self.is_pressed() == self.was_pressed() }

    fn is_toggle(&self) -> bool { self.is_pull_changed() }

    fn is_released(&self) -> bool { !self.is_pressed() }
    fn was_released(&self) -> bool { !self.was_pressed() }

    fn is_just_pressed(&self) -> bool { self.is_pull_up() }
    fn is_just_released(&self) -> bool { self.is_pull_down() }

    fn is_hold(&self) -> bool { self.is_pull_constant() && self.is_pressed() }

    fn evolution(&self) -> ButtonStateEvolution
    {
        match (self.is_pressed(), self.was_pressed())
        {
            (true, true) => ButtonStateEvolution::Pressed,
            (true, false) => ButtonStateEvolution::JustPressed,
            (false, true) => ButtonStateEvolution::JustReleased,
            (false, false) => ButtonStateEvolution::Released,
        }
    }
}

impl<S,T> EvolutionButtonState<T> for S where S:IEvolution<ButtonState,T>, T:Copy+Default
{
    fn is_pressed(&self) -> bool { self.value().is_pressed() }
    fn was_pressed(&self) -> bool { self.old_value().is_pressed() }
}