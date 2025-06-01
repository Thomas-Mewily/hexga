use crate::*;
use std::{hint::unreachable_unchecked, iter::{FusedIterator,Map}};

pub trait RangeStepExtension
{
    type Output : Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    /// Should emit an empty iterator if step is zero (not well defined)
    fn step(self, step: Self::Item) -> Self::Output;
    fn step_by_one(self) -> Self::Output where Self : Sized, Self::Item : One { self.step(Self::Item::ONE) }
}


// Todo : Remove once the Step trait will be stabilized
pub trait RangeStepIter : NumberPrimitive
{
    // Using Range, last value is excluded
    fn iter(max_excluded : Self) -> RangeStep<Self> { RangeStep { idx: Self::ZERO, end: max_excluded, step: Self::ONE } }
}
impl<T> RangeStepIter for T where T: NumberPrimitive {}

pub trait RangeDefaultStepExtension : RangeDefault where Range<Self> : RangeStepExtension
{
    /// Step using the [RangeDefault] : `Self::RANGE_MIN..Self::MAX`
    fn step(self, step: <Range::<Self> as RangeStepExtension>::Item) -> <Range::<Self> as RangeStepExtension>::Output;
}
impl<T> RangeDefaultStepExtension for T where T: RangeDefault, Range<T> : RangeStepExtension
{
    fn step(self, step: <Range::<Self> as RangeStepExtension>::Item) -> <Range::<Self> as RangeStepExtension>::Output { (Self::RANGE_MIN..Self::RANGE_MAX).step(step) }
}
pub trait RangeDefaultStepInclusiveExtension : RangeDefault where RangeInclusive<Self> : RangeStepExtension
{
    /// Step using the [RangeDefault] : `Self::RANGE_MIN..=Self::MAX`
    fn step_inclusive(self, step: <RangeInclusive::<Self> as RangeStepExtension>::Item) -> <RangeInclusive::<Self> as RangeStepExtension>::Output;
}
impl<T> RangeDefaultStepInclusiveExtension for T where T: RangeDefault, RangeInclusive<T> : RangeStepExtension
{
    fn step_inclusive(self, step: <RangeInclusive::<Self> as RangeStepExtension>::Item) -> <RangeInclusive::<Self> as RangeStepExtension>::Output { (Self::RANGE_MIN..=Self::RANGE_MAX).step(step) }
}

// Not [Copy] because Range<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct RangeStep<T> where T: NumberPrimitive
{
    pub idx  : T,
    pub end  : T,
    pub step : T,
}


impl<T> Iterator for RangeStep<T> where T: NumberPrimitive
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx <= self.end
        {
            let val = self.idx;
            self.idx += self.step;

            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                NumberType::Float => if self.idx == val { None } else { Some(val) },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        } else
        {
            None
        }
    }
}
impl<T> DoubleEndedIterator for RangeStep<T> where T: NumberPrimitive
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end >= self.idx
        {
            let val = self.end;
            self.end -= self.step;

            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float => if self.end == val { None } else { Some(val) },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        } else
        {
            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.end + self.step > self.idx
                    {
                        self.step = T::ZERO;
                        self.end = self.idx;
                        Some(self.idx)
                    } else { None }
                },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        }
    }
}
impl<T> FusedIterator for RangeStep<T> where T: NumberPrimitive  {}


impl<T> RangeStepExtension for Range<T> where T: NumberPrimitive
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        RangeStep { idx: self.start, end: self.end - step, step }
    }
}
impl<T> RangeStepExtension for RangeTo<T> where T: NumberPrimitive + RangeDefault
{
    type Output = RangeStep<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::RANGE_MIN..self.end).step(step)
    }
}


// Not [Copy] because RangeInclusive<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
pub struct RangeStepInclusive<T> where T: NumberPrimitive
{
    pub idx  : T,
    pub end  : T,
    pub step : T,
}

impl<T> Iterator for RangeStepInclusive<T> where T: NumberPrimitive
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx <= self.end
        {
            let val = self.idx;
            self.idx += self.step;

            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float => if self.idx == val { None } else { Some(val) },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        } else
        {
            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.idx - self.step < self.end
                    {
                        self.step = T::ZERO;
                        self.idx = self.end;
                        Some(self.end)
                    } else { None }
                },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        }
    }
}
impl<T> DoubleEndedIterator for RangeStepInclusive<T> where T: NumberPrimitive
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end >= self.idx
        {
            let val = self.end;
            self.end -= self.step;

            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float => if self.end == val { return None } else { Some(val) },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        } else
        {
            match T::PRIMITIVE_NUMBER_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.end + self.step > self.idx
                    {
                        self.step = T::ZERO;
                        self.end = self.idx;
                        Some(self.idx)
                    } else { None }
                },
                NumberType::Bool => unsafe { unreachable_unchecked() },
            }
        }
    }
}
impl<T> FusedIterator for RangeStepInclusive<T> where T: NumberPrimitive  {}


impl<T> RangeStepExtension for RangeFrom<T> where T: NumberPrimitive + RangeDefault
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (self.start..=T::RANGE_MAX).step(step)
    }
}
impl<T> RangeStepExtension for RangeInclusive<T> where T: NumberPrimitive
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        let (start, end) = self.into_inner();
        RangeStepInclusive { idx: start, end, step }
    }
}
impl<T> RangeStepExtension for RangeToInclusive<T> where T: NumberPrimitive + RangeDefault
{
    type Output = RangeStepInclusive<T>;
    type Item = T;
    fn step(self, step: T) -> Self::Output
    {
        (T::RANGE_MIN..=self.end).step(step)
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