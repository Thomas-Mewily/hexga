use super::*;

pub mod prelude
{
    pub use super::
    {
        InputDelta,IInputDelta,IInputButton,InputButtonChange,
        InputBool,InputInt,InputFloat,InputVec,
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct InputDelta<I,T> where I:Copy, T:Copy+Default
{
    cur : I,
    old : I,
    time : T,
}
impl<I,T> Deref for InputDelta<I,T> where I:Copy, T:Copy+Default
{
    type Target=I;
    fn deref(&self) -> &Self::Target { &self.cur }
}

impl<I,T> From<I> for InputDelta<I,T> where I:Copy, T:Copy+Default
{
    fn from(value: I) -> Self {
        Self::from_value(value)
    }
}

pub type InputVec<T=Time> = InputDelta<Vec2,T>;
pub type InputBool<T=Time> = InputDelta<bool,T>;
pub type InputInt<T=Time> = InputDelta<int,T>;
pub type InputFloat<T=Time> = InputDelta<float,T>;

impl<I,T> InputDelta<I,T> where I:Copy, T:Copy+Default
{
    pub fn from_time_and_value(time : T, value : I) -> Self { Self::new(value, value, time) }
    pub fn from_value(value : I) -> Self { Self::new(value, value, ___()) }
    pub fn new(cur : I, old : I, time : T) -> Self { Self { cur, old, time }}
}

pub trait IInputDelta<I,T> where I:Copy, T:Copy+Default
{
    fn cur(&self) -> I;
    fn old(&self) -> I;
    fn last_time_changed(&self) -> T;

    fn delta(&self) -> I::Output where I:Sub { self.cur() - self.old() }

    fn set(&mut self, cur : I, time : T) where I:PartialEq;

    /// Set the value if it is different
    fn update(&mut self, cur : I, time : T) where I:PartialEq
    {
        if self.old() != self.cur()
        {
            self.set(cur, time);
        }
    }
}

impl<I, T> IInputDelta<I,T> for InputDelta<I, T> where I:Copy, T:Copy+Default, I: PartialEq
{
    fn cur(&self) -> I {
        self.cur
    }

    fn old(&self) -> I {
        self.old
    }

    fn last_time_changed(&self) -> T {
        self.time
    }

    fn set(&mut self, cur : I, time : T) where I:PartialEq {
        self.old = self.cur;
        self.cur = cur;
        self.time = time;
    }
}

pub trait IInputButton
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

    fn change(&self) -> InputButtonChange
    {
        match (self.is_pressed(), self.was_pressed())
        {
            (true, true) => InputButtonChange::Pressed,
            (true, false) => InputButtonChange::JustPressed,
            (false, true) => InputButtonChange::JustReleased,
            (false, false) => InputButtonChange::Released,
        }
    }
}


impl<T> IInputButton for InputDelta<bool, T> where T:Copy+Default
{
    fn is_pressed(&self) -> bool { self.cur }
    fn was_pressed(&self) -> bool { self.old }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum InputButtonChange
{
    Pressed,
    Released,
    /// Can be use for toggle
    JustPressed,
    /// Can be use for toggle
    JustReleased,
}
impl IInputButton for InputButtonChange
{
    fn is_pressed(&self) -> bool {
        matches!(self, Self::Pressed | Self::JustPressed)
    }

    fn was_pressed(&self) -> bool
    {
        matches!(self, Self::Released | Self::JustReleased)
    }
}