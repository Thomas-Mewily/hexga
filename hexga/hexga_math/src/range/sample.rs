use crate::*;
use std::{ops::{Range, RangeInclusive}};
use std::iter::{FusedIterator,Map};

pub trait RangeSampleExtension<I=usize>
{
    type Output : Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    fn sample(self, nb_sample: I) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct RangeSample<I,T> where I : Integer, T : Number, I : CastInto<T>
{
    current   : I,
    nb_sample : I,
    begin     : T,
    step      : T,
}

impl<I,T> Iterator for RangeSample<I,T> where I : Integer, T : Number, I : CastInto<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nb_sample {
            None
        } else {
            let val = self.begin + T::cast_from(self.current) * self.step;
            self.current.increase();
            Some(val)
        }
    }
}

impl<I,T> DoubleEndedIterator for RangeSample<I,T> where I : Integer, T : Number, I : CastInto<T> 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current >= self.nb_sample {
            None
        } else {
            self.nb_sample.decrease();
            Some(self.begin + T::cast_from(self.current) * self.step)
        }
    }
}
impl<I,T> FusedIterator for RangeSample<I,T> where I : Integer, T : Number, I : CastInto<T>  {}


impl<I,T> RangeSampleExtension<I> for Range<T> where I : Integer, T : Number, I : CastInto<T>
{
    type Output = RangeSample<I,T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output 
    {
        let (start, end) = (self.start, self.end);
        let step = if nb_sample.is_zero() { T::ZERO } else { (end - start) / T::cast_from(nb_sample) };
        RangeSample 
        {
            current: I::ZERO,
            nb_sample,
            begin: start,
            step,
        }
    }
}

impl<I,T> RangeSampleExtension<I> for RangeInclusive<T> where I : Integer, T : Number, I : CastInto<T>
{
    type Output = RangeSample<I,T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output 
    {
        let (start, end) = self.into_inner();
        let step = if nb_sample.is_zero() { T::ZERO } else { (end - start) / T::cast_from(nb_sample - I::ONE) };
        RangeSample 
        {
            current: I::ZERO,
            nb_sample,
            begin: start,
            step,
        }
    }
}