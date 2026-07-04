use super::*;

pub trait RangeSampleExtension<I = usize>
{
    type Output: Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    fn sample(self, nb_sample: I) -> Self::Output;
}

pub trait RangeDefaultSampleExtension<I = usize>: RangeDefault
where
    Range<Self>: RangeSampleExtension<I>,
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..Self::MAX`
    fn sample(nb_sample: I) -> <Range<Self> as RangeSampleExtension<I>>::Output;
}
impl<I, T> RangeDefaultSampleExtension<I> for T
where
    T: RangeDefault,
    Range<T>: RangeSampleExtension<I>,
{
    fn sample(nb_sample: I) -> <Range<Self> as RangeSampleExtension<I>>::Output { (Self::RANGE_MIN..Self::RANGE_MAX).sample(nb_sample) }
}
pub trait RangeDefaultSampleInclusiveExtension<I = usize>: RangeDefault
where
    RangeInclusive<Self>: RangeSampleExtension<I>,
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..=Self::MAX`
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive<Self> as RangeSampleExtension<I>>::Output;
}
impl<I, T> RangeDefaultSampleInclusiveExtension<I> for T
where
    T: RangeDefault,
    RangeInclusive<T>: RangeSampleExtension<I>,
{
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive<Self> as RangeSampleExtension<I>>::Output { (Self::RANGE_MIN..=Self::RANGE_MAX).sample(nb_sample) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    pub idx: I,
    pub end: I,
    pub offset: U,
    pub step: U,
}

impl<I, U> Iterator for RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.idx >= self.end
        {
            None
        }
        else
        {
            let val = self.offset.inner_value() + self.idx.cast_into() * self.step.inner_value();
            self.idx.increment();
            Some(U::from_inner_value(val))
        }
    }
}

impl<I, U> DoubleEndedIterator for RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        if self.idx >= self.end
        {
            None
        }
        else
        {
            self.end.decrement();
            Some(U::from_inner_value(self.offset.inner_value() + self.idx.cast_into() * self.step.inner_value()))
        }
    }
}
impl<I, U> FusedIterator for RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
}

impl<I, U> RangeSampleExtension<I> for Range<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = (self.start, self.end);
        let step = if nb_sample.is_zero()
        {
            U::Precision::ZERO
        }
        else
        {
            (end - start).inner_value() / nb_sample.cast_into()
        };
        RangeSample {
            idx: I::ZERO,
            end: nb_sample,
            offset: start,
            step: U::from_inner_value(step),
        }
    }
}

impl<I, U> RangeSampleExtension<I> for RangeInclusive<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = self.into_inner();
        let step = if nb_sample.is_zero()
        {
            U::Precision::ZERO
        }
        else
        {
            (end - start).inner_value() / (nb_sample - I::ONE).cast_into()
        };
        RangeSample {
            idx: I::ZERO,
            end: nb_sample,
            offset: start,
            step: U::from_inner_value(step),
        }
    }
}

impl<I, U> RangeSampleExtension<I> for RangeTo<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (U::RANGE_MIN..self.end).sample(nb_sample) }
}
impl<I, U> RangeSampleExtension<I> for RangeToInclusive<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (U::RANGE_MIN..=self.end).sample(nb_sample) }
}

impl<I, U> RangeSampleExtension<I> for RangeFrom<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (self.start..=U::RANGE_MAX).sample(nb_sample) }
}
