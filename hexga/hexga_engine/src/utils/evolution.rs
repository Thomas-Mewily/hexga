use super::*;

pub mod prelude
{
    pub use super::{Evolution,EvolutionTime,EvolutionDelta,EvolutionBool};
}




#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct EvolutionDelta<I,T> where I:Copy, T:Copy+Default
{
    cur : I,
    old : I,
    time : T,
}

pub trait Evolution<I,T> where I:Copy, T:Copy+Default
{
    /// The current state right now
    fn value(&self) -> I;
    /// The state in the old frame
    fn old_value(&self) -> I;

    /// `value() - old_value()`
    fn value_delta(&self) -> I::Output where I:Sub { self.value() - self.old_value() }

    fn last_time_changed(&self) -> T;
    /// Delta time since the last change
    fn delta_time_since(&self, current_time: T) -> T where T:Sub<T,Output=T> { current_time - self.last_time_changed() }


    fn set_at(&mut self, cur : I, time : T) where I:PartialEq;

    /// Also reset the time if the value is different
    fn update_at(&mut self, cur : I, time : T) where I:PartialEq
    {
        if self.old_value() != self.value()
        {
            self.set_at(cur, time);
        }
    }
}

pub trait EvolutionTime<I,F> : Evolution<I,TimeOf<F>> where I:Copy, F:Float, TimeOf<F>:TimeNow
{
    fn delta_time(&self) -> TimeOf<F> { TimeOf::<F>::since_launch() - self.last_time_changed() }
    fn set(&mut self, cur : I) where I:PartialEq { self.set_at(cur, TimeOf::<F>::since_launch()); }
    fn update(&mut self, cur : I) where I:PartialEq { self.update_at(cur, TimeOf::<F>::since_launch()); }
}
impl<I,F,S> EvolutionTime<I,F> for S where S:Evolution<I,TimeOf<F>>, I:Copy, F:Float, TimeOf<F>:TimeNow {}


impl<I, T> Evolution<I,T> for EvolutionDelta<I, T> where I:Copy, T:Copy+Default, I: PartialEq
{
    fn value(&self) -> I {
        self.cur
    }

    fn old_value(&self) -> I {
        self.old
    }

    fn last_time_changed(&self) -> T {
        self.time
    }

    fn set_at(&mut self, cur : I, time : T) where I:PartialEq {
        self.old = self.cur;
        self.cur = cur;
        self.time = time;
    }
}

pub trait EvolutionBoolIterator<T> : Iterator + Sized where Self::Item: EvolutionBool<T>, T:Copy+Default
{
    fn pressed(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_pressed()) }
    fn released(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_released()) }

    fn just_pressed(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_just_pressed()) }
    fn just_released(self) -> impl Iterator<Item = Self::Item> { self.filter(|x| x.is_just_released()) }
}
impl<T,S> EvolutionBoolIterator<T> for S where S: Iterator + Sized, S::Item: EvolutionBool<T>, T:Copy+Default{}

pub trait EvolutionBool<T> : Evolution<bool,T> where T:Copy+Default
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

    fn change(&self) -> ButtonStateChange
    {
        match (self.is_pressed(), self.was_pressed())
        {
            (true, true) => ButtonStateChange::Pressed,
            (true, false) => ButtonStateChange::JustPressed,
            (false, true) => ButtonStateChange::JustReleased,
            (false, false) => ButtonStateChange::Released,
        }
    }
}

impl<S,T> EvolutionBool<T> for S where S:Evolution<bool,T>, T:Copy+Default
{
    fn is_pressed(&self) -> bool { self.value() }
    fn was_pressed(&self) -> bool { self.old_value() }
}