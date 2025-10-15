use super::*;



pub trait RangeSampleExtension<I=usize>
{
    type Output : Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    fn sample(self, nb_sample: I) -> Self::Output;
}

pub trait RangeDefaultSampleExtension<I=usize> : RangeDefault where Range<Self> : RangeSampleExtension<I>
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..Self::MAX`
    fn sample(nb_sample: I) -> <Range::<Self> as RangeSampleExtension<I>>::Output;
}
impl<I,T> RangeDefaultSampleExtension<I> for T where T: RangeDefault, Range<T> : RangeSampleExtension<I>
{
    fn sample(nb_sample: I) -> <Range::<Self> as RangeSampleExtension<I>>::Output { (Self::RANGE_MIN..Self::RANGE_MAX).sample(nb_sample) }
}
pub trait RangeDefaultSampleInclusiveExtension<I=usize> : RangeDefault where RangeInclusive<Self> : RangeSampleExtension<I>
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..=Self::MAX`
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive::<Self> as RangeSampleExtension<I>>::Output;
}
impl<I,T> RangeDefaultSampleInclusiveExtension<I> for T where T: RangeDefault, RangeInclusive<T> : RangeSampleExtension<I>
{
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive::<Self> as RangeSampleExtension<I>>::Output { (Self::RANGE_MIN..=Self::RANGE_MAX).sample(nb_sample) }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Debug)]
pub struct RangeSample<I,T> where I : Number + CastInto<T>, T : Number
{
    pub idx   : I,
    pub end   : I,
    pub offset: T,
    pub step  : T,
}

impl<I,T> Iterator for RangeSample<I,T> where I : Number + CastInto<T>, T : Number
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.end {
            None
        } else {
            let val = self.offset + self.idx.cast_into() * self.step;
            self.idx.increment();
            Some(val)
        }
    }
}

impl<I,T> DoubleEndedIterator for RangeSample<I,T> where I : Number + CastInto<T>, T : Number
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.end {
            None
        } else {
            self.end.decrement();
            Some(self.offset + self.idx.cast_into() * self.step)
        }
    }
}
impl<I,T> FusedIterator for RangeSample<I,T> where I : Number + CastInto<T>, T : Number  {}


impl<I,T> RangeSampleExtension<I> for Range<T> where I : Number + CastInto<T>, T : Number
{
    type Output = RangeSample<I,T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = (self.start, self.end);
        let step = if nb_sample.is_zero() { T::ZERO } else { (end - start) / nb_sample.cast_into() };
        RangeSample
        {
            idx: I::ZERO,
            end: nb_sample,
            offset: start,
            step,
        }
    }
}

impl<I,T> RangeSampleExtension<I> for RangeInclusive<T> where I : Number + CastInto<T>, T : Number
{
    type Output = RangeSample<I,T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = self.into_inner();
        let step = if nb_sample.is_zero() { T::ZERO } else { (end - start) / (nb_sample - I::ONE).cast_into() };
        RangeSample
        {
            idx: I::ZERO,
            end: nb_sample,
            offset: start,
            step,
        }
    }
}


impl<I,T> RangeSampleExtension<I> for RangeTo<T> where I : Number + CastInto<T>, T : Number + RangeDefault
{
    type Output = RangeSample<I, T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        (T::RANGE_MIN..self.end).sample(nb_sample)
    }
}
impl<I,T> RangeSampleExtension<I> for RangeToInclusive<T> where I : Number + CastInto<T>, T : Number + RangeDefault
{
    type Output = RangeSample<I, T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        (T::RANGE_MIN..=self.end).sample(nb_sample)
    }
}

impl<I,T> RangeSampleExtension<I> for RangeFrom<T> where I : Number + CastInto<T>, T : Number + RangeDefault
{
    type Output = RangeSample<I, T>;
    type Item = T;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        (self.start..=T::RANGE_MAX).sample(nb_sample)
    }
}