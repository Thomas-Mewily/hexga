use super::*;

pub trait IterSample<I = usize>
{
    type Output: Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    fn sample(self, nb_sample: I) -> Self::Output;
}

pub trait IterSampleDefault<I = usize>: RangeDefault
where
    Range<Self>: IterSample<I>,
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..Self::MAX`
    fn sample(nb_sample: I) -> <Range<Self> as IterSample<I>>::Output;
}
impl<I, T> IterSampleDefault<I> for T
where
    T: RangeDefault,
    Range<T>: IterSample<I>,
{
    fn sample(nb_sample: I) -> <Range<Self> as IterSample<I>>::Output { (Self::RANGE_MIN..Self::RANGE_MAX).sample(nb_sample) }
}
pub trait IterSampleDefaultInclusive<I = usize>: RangeDefault
where
    RangeInclusive<Self>: IterSample<I>,
{
    /// Sample using the [`RangeDefault`] : `Self::RANGE_MIN..=Self::MAX`
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive<Self> as IterSample<I>>::Output;
}
impl<I, T> IterSampleDefaultInclusive<I> for T
where
    T: RangeDefault,
    RangeInclusive<T>: IterSample<I>,
{
    fn sample_inclusive(nb_sample: I) -> <RangeInclusive<Self> as IterSample<I>>::Output { (Self::RANGE_MIN..=Self::RANGE_MAX).sample(nb_sample) }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    pub idx: I,
    pub nb: I,

    pub begin: U,
    pub end  : U,
    pub step : U,
}

impl<I, U> Iterator for RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.idx >= self.nb
        {
            None
        }
        else
        {
            let val = self.begin.inner_value() + self.idx.cast_into() * self.step.inner_value();
            self.idx.increment();
            Some(U::from_inner_value(val.clamp_partial(self.begin.inner_value(), self.end.inner_value())))
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
        if self.idx >= self.nb
        {
            None
        }
        else
        {
            self.nb.decrement();
            let val = self.begin.inner_value() + self.nb.cast_into() * self.step.inner_value();
            Some(U::from_inner_value(val.clamp_partial(self.begin.inner_value(), self.end.inner_value())))
        }
    }
}
impl<I, U> FusedIterator for RangeSample<I, U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
}

impl<I, U> IterSample<I> for Range<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = (self.start, self.end);
        let step = if nb_sample <= one()
        {
            U::Precision::ZERO
        }
        else
        {
            (end.inner_value() - start.inner_value()) / nb_sample.cast_into()
        };
        RangeSample {
            idx: I::ZERO,
            nb: nb_sample,
            begin: start,
            step: U::from_inner_value(step),
            end,
        }
    }
}

impl<I, U> IterSample<I> for RangeInclusive<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit,
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output
    {
        let (start, end) = self.into_inner();
        let step = if nb_sample <= one()
        {
            U::Precision::ZERO
        }
        else
        {
            (end.inner_value() - start.inner_value()) / (nb_sample - I::ONE).cast_into()
        };
        RangeSample {
            idx: I::ZERO,
            nb: nb_sample,
            begin: start,
            step: U::from_inner_value(step),
            end,
        }
    }
}

impl<I, U> IterSample<I> for RangeTo<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (U::RANGE_MIN..self.end).sample(nb_sample) }
}
impl<I, U> IterSample<I> for RangeToInclusive<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (U::RANGE_MIN..=self.end).sample(nb_sample) }
}

impl<I, U> IterSample<I> for RangeFrom<U>
where
    I: Number + CastInto<U::Precision>,
    U: Unit + RangeDefault
{
    type Output = RangeSample<I, U>;
    type Item = U;

    fn sample(self, nb_sample: I) -> Self::Output { (self.start..=U::RANGE_MAX).sample(nb_sample) }
}
