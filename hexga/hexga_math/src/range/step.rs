use crate::*;
use std::{ops::{Range, RangeInclusive}};
use std::iter::{FusedIterator,Map};

pub trait RangeStepExtension
{
    type Output : Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    /// Should emit an empty iterator if step is zero (not well defined)
    fn step(self, step: Self::Item) -> Self::Output;
}

// Not [Copy] because Range<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug, Hash)]
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
        if self.start <= self.end 
        {
            let val = self.start;
            self.start += self.step;
            if self.start == val { None } else { Some(val) }
        } else 
        {
            None
        }
    }
}
impl<T> DoubleEndedIterator for RangeStep<T> where T : Number 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end >= self.start 
        {
            let val = self.end;
            self.end -= self.step;
            if self.end == val { None } else { Some(val)}
        } else 
        {
            // Force the iterator to stop at the first value
            if self.end + self.step > self.start
            {
                self.step = T::ZERO;
                self.end = self.start;
                Some(self.start)
            } else { None }
        }
    }
}
impl<T> FusedIterator for RangeStep<T> where T : Number  {}


impl<T> RangeStepExtension for Range<T> where T : Number
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        RangeStep { start: self.start, end: self.end - step, step }
    }
}
impl<T> RangeStepExtension for RangeTo<T> where T : Number + DefaultRange
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::MIN_RANGE..self.end).step(step)
    }
}


// Not [Copy] because RangeInclusive<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct RangeStepInclusive<T> where T : Number
{
    pub start  : T,
    pub end    : T,
    pub step   : T,
}

impl<T> Iterator for RangeStepInclusive<T> where T : Number 
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end 
        {
            let val = self.start;
            self.start += self.step;
            if self.start == val { return None } else { Some(val) }
        } else 
        {
            // Force the iterator to stop at the first value
            if self.start - self.step < self.end
            {
                self.step = T::ZERO;
                self.start = self.end;
                Some(self.end)
            } else { None }
        }
    }
}
impl<T> DoubleEndedIterator for RangeStepInclusive<T> where T : Number 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end >= self.start 
        {
            let val = self.end;
            self.end -= self.step;
            if self.end == val { return None } else { Some(val) }
        } else 
        {
            // Force the iterator to stop at the first value
            if self.end + self.step > self.start
            {
                self.step = T::ZERO;
                self.end = self.start;
                Some(self.start)
            } else { None }
        }
    }
}
impl<T> FusedIterator for RangeStepInclusive<T> where T : Number  {}


impl<T> RangeStepExtension for RangeFrom<T> where T : Number + DefaultRange
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (self.start..=T::MAX_RANGE).step(step)
    }
}
impl<T> RangeStepExtension for RangeInclusive<T> where T : Number
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        let (start, end) = self.into_inner();
        RangeStepInclusive { start, end, step }
    }
}
impl<T> RangeStepExtension for RangeToInclusive<T> where T : Number + DefaultRange
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::MIN_RANGE..=self.end).step(step)
    }
}


#[cfg(test)]
mod range_test
{
    use crate::*;

    #[test]
    fn range()
    {
        assert_eq!((-2..5).step(1).to_vec(), vec![-2,-1,0,1,2,3,4]);
    }

    #[test]
    fn range_rev()
    {
        assert_eq!((-2..5).step(1).rev().to_vec(), vec![4,3,2,1,0,-1,-2]);
    }

    #[test]
    fn range_inclusive()
    {
        assert_eq!((-2..=5).step(1).to_vec(), vec![-2,-1,0,1,2,3,4,5]);
    }

    #[test]
    fn range_inclusive_rev()
    {
        assert_eq!((-2..=5).step(1).rev().to_vec(), vec![5,4,3,2,1,0,-1,-2]);
    }


    
    #[test]
    fn range_float()
    {
        assert_eq!((0.5..2.5).step(1.).to_vec(), vec![0.5,1.5]);
    }

    #[test]
    fn range_rev_float()
    {
        assert_eq!((0.5..2.5).step(1.).rev().to_vec(), vec![1.5,0.5]);
    }

    #[test]
    fn range_inclusive_float()
    {
        assert_eq!((0.5..=2.5).step(1.).to_vec(), vec![0.5,1.5,2.5]);
    }

    #[test]
    fn range_inclusive_rev_float()
    {
        assert_eq!((0.5..=2.5).step(1.).rev().to_vec(), vec![2.5,1.5,0.5]);
    }


    #[test]
    fn range_float_2()
    {
        let values = (0.0..1.0).step(0.3).to_vec();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], 0.0);
        assert!(values.last().copied().unwrap() <= 0.95);
    }

    #[test]
    fn range_rev_float_2()
    {
        let values = (0.0..1.0).step(0.3).rev().to_vec();
        assert_eq!(values.len(), 4);
        assert!((values[0] - 0.7).abs() <= 0.00001);
        assert_eq!(values.last().copied().unwrap(), 0.);
    }

    #[test]
    fn range_inclusive_float_2()
    {
        let values = (0.0..=1.0).step(0.3).to_vec();
        assert_eq!(values.len(), 5);
        assert_eq!(values[0], 0.0);
        assert_eq!(values.last().copied().unwrap(), 1.0);
    }

    #[test]
    fn range_inclusive_rev_float_2()
    {
        let values = (0.0..=1.0).step(0.3).rev().to_vec();
        assert_eq!(values.len(), 5);
        assert_eq!(values[0], 1.0);
        assert_eq!(values.last().copied().unwrap(), 0.);
    }
}