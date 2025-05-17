use crate::*;
use std::{ops::{Range, RangeInclusive}};
use std::iter::{FusedIterator,Map};

pub trait RangeStepExtension<T>
{
    type Output : Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    /// Should emit an empty iterator if step is zero (not well defined)
    fn step(self, step: T) -> Self::Output;
}

// Not [Copy] because Range<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug)]
pub struct RangeStep<T> where T : Number
{
    pub start  : T,
    pub end    : T,
    pub step   : T,
}


impl<T> Iterator for RangeStep<T> where T : Number 
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let val = self.start;
            self.start += self.step;
            if self.start == val { return None }
            Some(val)
        }
    }
}
impl<T> DoubleEndedIterator for RangeStep<T> where T : Number 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end < self.start {
            None
        } else {
            let val = self.end;
            self.end -= self.step;
            if self.end == val { return None }
            Some(val)
        }
    }
}
impl<T> FusedIterator for RangeStep<T> where T : Number  {}


impl<T> RangeStepExtension<T> for Range<T> where T : Number
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        RangeStep { start: self.start, end: self.end - step, step }
    }
}
impl<T> RangeStepExtension<T> for RangeInclusive<T> where T : Number
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        let (start, end) = self.into_inner();
        RangeStep { start, end, step }
    }
}

impl<T> RangeStepExtension<T> for RangeTo<T> where T : Number
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::ZERO..self.end).step(step)
    }
}

impl<T> RangeStepExtension<T> for RangeToInclusive<T> where T : Number
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::ZERO..=self.end).step(step)
    }
}