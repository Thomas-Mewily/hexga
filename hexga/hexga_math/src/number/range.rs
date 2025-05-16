use crate::*;

pub trait FloatRange<F> : Sized
{
    fn step(self, step: F) -> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator;
    fn samples(self, n: usize) -> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator;
}

/* 
pub trait FloatRangeMinMax<F> : Sized where F : Float
{

}
*/

// Not [Copy] because Range<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug)]
pub struct FloatIt<F> where F : Float
{
    begin : F,
    left  : F,
    right : F,
    step  : F,
}

impl<F> Iterator for FloatIt<F> where F : Float
{
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left > self.right 
        {
            None
        } else {
            let val = self.left;
            self.left += self.step;
            if self.left == val { return None }
            Some(val)
        }
    }
}

impl<F> DoubleEndedIterator for FloatIt<F> where F : Float
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.right < self.left 
        {
            None
        } else {
            let val = self.right;
            self.right -= self.step;
            if self.right == val { return None }
            Some(val)
        }
    }
}
impl<F> std::iter::FusedIterator for FloatIt<F> where F : Float {}

impl<F> FloatRange<F> for std::ops::Range<F> where F : Float 
{
    fn step(self, step: F) -> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator
    {
        assert!(step.is_strictly_positive(), "step must be positive");
        FloatIt 
        {
            begin: self.start,
            left: self.start,
            right: self.end - step,
            step,
        }
    }

    fn samples(self, n: usize)-> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator 
    {
        assert!(n >= 1, "Need at least one sample");
        let step = (self.end - self.start) / F::cast_from(n);
        Self::step(self, step)
    }
}

impl<F> FloatRange<F> for std::ops::RangeInclusive<F> where F : Float {
    fn step(self, step: F) -> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator
    {
        assert!(step.is_strictly_positive(), "step must be positive");
        let (start, end) = self.clone().into_inner();
        FloatIt 
        {
            begin: start,
            left: start,
            right: end,  // inclusive end, last value = end
            step,
        }
    }

    fn samples(self, n: usize) -> impl Iterator<Item = F> + DoubleEndedIterator + std::iter::FusedIterator
    {
        assert!(n >= 2, "Need at least two samples for inclusive range");
        let (start, end) = self.clone().into_inner();
        let step = (end - start) / F::cast_from(n - 1);
        Self::step(self, step)
    }
}
