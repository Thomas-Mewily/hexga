use super::*;


pub mod prelude
{
    pub use super::{PreviousValue,Evolution};
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct PreviousValue<T> where T:Copy
{
    pub value      : T,
    pub old_value  : T,
    //pub last_change: T,
}

impl<V> PreviousValue<V>  where V:Copy
{
    pub fn new(value: V, old_value: V) -> Self { Self { value, old_value }}
}

impl<T> Evolution<T> for PreviousValue<T> where T:Copy
{
    fn value(&self) -> T { self.value }
    fn old_value(&self) -> T { self.old_value }
}


pub trait Evolution<I> where I:Copy
{
    /// The current state right now
    fn value(&self) -> I;
    /// The state in the old frame
    fn old_value(&self) -> I;

    /// `value() - old_value()`
    fn delta(&self) -> I::Output where I:Sub { self.value() - self.old_value() }

    fn evolution(&self) -> PreviousValue<I> { PreviousValue::new(self.value(), self.old_value()) }

    // fn last_time_change(&self) -> T;
    // Delta time since the last change
    // fn dt_since(&self, current_time: T) -> T where T:Sub<T,Output=T> { current_time - self.last_time_change() }
}





/*
impl<I> Evolution<I> where I:Copy
{
    /// Also reset the time if the value is different
    pub fn update_at(&mut self, value : I, time : T) where I:PartialEq
    {
        if value != self.value()
        {
            self.last_change = time;
        }
        self.old_value = value;
        self.value = value;
    }
}
impl<I,T> Evolution<I,T> where I:Copy, T:Copy+Default+TimeNow
{
    /// Also reset the time if the value is different
    pub fn update(&mut self, value : I) where I:PartialEq
    {
        self.update_at(value, T::since_launch())
    }
}

pub trait EvolutionTime<I,F> : IEvolution<I,TimeOf<F>> where I:Copy, F:Float, TimeOf<F>:TimeNow
{
    fn dt(&self) -> DeltaTimeOf<F> { DeltaTimeOf::<F>::since_launch() - self.last_time_change() }
}
impl<I,F,S> EvolutionTime<I,F> for S where S:IEvolution<I,TimeOf<F>>, I:Copy, F:Float, TimeOf<F>:TimeNow {}


impl<I, T> IEvolution<I,T> for Evolution<I, T> where I:Copy, T:Copy+Default, I: PartialEq
{
    fn value(&self) -> I { self.value }
    fn old_value(&self) -> I { self.old_value }
    fn last_time_change(&self) -> T { self.last_change }
}
*/
