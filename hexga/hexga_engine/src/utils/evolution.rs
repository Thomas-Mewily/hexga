use super::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Evolution<I,T=Time> where I:Copy, T:Copy+Default
{
    pub value      : I,
    pub old_value  : I,
    pub last_change: T,
}



pub trait IEvolution<I,T=Time> where I:Copy, T:Copy+Default
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

pub trait EvolutionTime<I,F> : IEvolution<I,TimeOf<F>> where I:Copy, F:Float, TimeOf<F>:TimeNow
{
    fn dt(&self) -> DeltaTimeOf<F> { DeltaTimeOf::<F>::since_launch() - self.last_time_changed() }
    fn set(&mut self, cur : I) where I:PartialEq { self.set_at(cur, TimeOf::<F>::since_launch()); }
    fn update(&mut self, cur : I) where I:PartialEq { self.update_at(cur, TimeOf::<F>::since_launch()); }
}
impl<I,F,S> EvolutionTime<I,F> for S where S:IEvolution<I,TimeOf<F>>, I:Copy, F:Float, TimeOf<F>:TimeNow {}


impl<I, T> IEvolution<I,T> for Evolution<I, T> where I:Copy, T:Copy+Default, I: PartialEq
{
    fn value(&self) -> I { self.value }
    fn old_value(&self) -> I { self.old_value }
    fn last_time_changed(&self) -> T { self.last_change }
    
    fn set_at(&mut self, cur : I, time : T) where I:PartialEq {
        self.old_value = self.value;
        self.value = cur;
        self.last_change = time;
    }
}
