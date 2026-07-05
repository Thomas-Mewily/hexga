use super::*;

pub trait IterStep
{
    type Output: Iterator<Item = Self::Item> + DoubleEndedIterator + FusedIterator;
    type Item;
    /// Should emit an empty iterator if step is zero (not well defined)
    fn step(self, step: Self::Item) -> Self::Output;
    fn step_by_one(self) -> Self::Output
    where
        Self: Sized,
        Self::Item: One,
    {
        self.step(Self::Item::ONE)
    }
}

// Todo : Remove once the Step trait will be stabilized
pub trait IterStepMax: Unit + One
{
    // Using Range, last value is excluded
    fn iter(max_excluded: Self) -> RangeStep<Self>
    {
        RangeStep {
            idx: Self::ZERO,
            end: max_excluded,
            step: Self::ONE,
        }
    }
}
impl<U> IterStepMax for U where U: Unit + One {}

pub trait IterStepDefault: RangeDefault
where
    Range<Self>: IterStep,
{
    /// Step using the [`RangeDefault`] : `Self::RANGE_MIN..Self::MAX`
    fn step(step: <Range<Self> as IterStep>::Item) -> <Range<Self> as IterStep>::Output;
}
impl<T> IterStepDefault for T
where
    T: RangeDefault,
    Range<T>: IterStep,
{
    fn step(step: <Range<Self> as IterStep>::Item) -> <Range<Self> as IterStep>::Output
    {
        (Self::RANGE_MIN..Self::RANGE_MAX).step(step)
    }
}
pub trait IterStepDefaultInclusive: RangeDefault
where
    RangeInclusive<Self>: IterStep,
{
    /// Step using the [`RangeDefault`] : `Self::RANGE_MIN..=Self::MAX`
    fn step_inclusive(step: <RangeInclusive<Self> as IterStep>::Item) -> <RangeInclusive<Self> as IterStep>::Output;
}
impl<T> IterStepDefaultInclusive for T
where
    T: RangeDefault,
    RangeInclusive<T>: IterStep,
{
    fn step_inclusive(step: <RangeInclusive<Self> as IterStep>::Item) -> <RangeInclusive<Self> as IterStep>::Output
    {
        (Self::RANGE_MIN..=Self::RANGE_MAX).step(step)
    }
}

// It's copy now.
// ~~Not [`Copy`] because Range<T> don't impl Copy because iterator are used by reference most of the time~~
// ~~See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy~~
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RangeStep<U>
where
    U: Unit,
{
    pub idx: U,
    pub end: U,
    pub step: U,
}

impl<U> Iterator for RangeStep<U>
where
    U: Unit,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.idx.inner_value() <= self.end.inner_value()
        {
            let val = self.idx;
            self.idx += self.step;

            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                NumberType::Float =>
                {
                    if self.idx == val
                    {
                        None
                    }
                    else
                    {
                        Some(val)
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
        else
        {
            None
        }
    }
}
impl<U> DoubleEndedIterator for RangeStep<U>
where
    U: Unit,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        if self.end.inner_value() >= self.idx.inner_value()
        {
            let val = self.end;
            self.end -= self.step;

            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    if self.end == val
                    {
                        None
                    }
                    else
                    {
                        Some(val)
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
        else
        {
            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.end.inner_value() + self.step.inner_value() > self.idx.inner_value()
                    {
                        self.step = U::ZERO;
                        self.end = self.idx;
                        Some(self.idx)
                    }
                    else
                    {
                        None
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
    }
}
impl<U> FusedIterator for RangeStep<U> where U: Unit {}

impl<U> IterStep for Range<U>
where
    U: Unit,
{
    type Output = RangeStep<U>;
    type Item = U;
    fn step(self, step: U) -> Self::Output
    {
        RangeStep {
            idx: self.start,
            end: self.end - step,
            step,
        }
    }
}
impl<U> IterStep for RangeTo<U>
where
    U: Unit + RangeDefault,
{
    type Output = RangeStep<U>;
    type Item = U;
    fn step(self, step: U) -> Self::Output { (U::RANGE_MIN..self.end).step(step) }
}

// Not [`Copy`] because RangeInclusive<T> don't impl Copy because iterator are used by reference most of the time
// See https://stackoverflow.com/questions/43416914/why-doesnt-opsranget-implement-copy-even-if-t-is-copy
#[derive(Clone, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RangeStepInclusive<U>
where
    U: Unit,
{
    pub idx: U,
    pub end: U,
    pub step: U,
}

impl<U> Iterator for RangeStepInclusive<U>
where
    U: Unit,
{
    type Item = U;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.idx.inner_value() <= self.end.inner_value()
        {
            let val = self.idx;
            self.idx += self.step;

            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    if self.idx == val
                    {
                        None
                    }
                    else
                    {
                        Some(val)
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
        else
        {
            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.idx.inner_value() - self.step.inner_value() < self.end.inner_value()
                    {
                        self.step = U::ZERO;
                        self.idx = self.end;
                        Some(self.end)
                    }
                    else
                    {
                        None
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
    }
}
impl<U> DoubleEndedIterator for RangeStepInclusive<U>
where
    U: Unit,
{
    fn next_back(&mut self) -> Option<Self::Item>
    {
        if self.end.inner_value() >= self.idx.inner_value()
        {
            let val = self.end;
            self.end -= self.step;

            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => Some(val),
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    if self.end == val
                    {
                        return None;
                    }
                    else
                    {
                        Some(val)
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
        else
        {
            match U::Precision::PRIMITIVE_TYPE
            {
                NumberType::IntegerUnsigned | NumberType::IntegerSigned => None,
                // Reached the limit of floating-point precision
                NumberType::Float =>
                {
                    // Force the iterator to stop at the first value
                    if self.end.inner_value() + self.step.inner_value() > self.idx.inner_value()
                    {
                        self.step = U::ZERO;
                        self.end = self.idx;
                        Some(self.idx)
                    }
                    else
                    {
                        None
                    }
                }
                NumberType::Bool => unreachable!(),
            }
        }
    }
}
impl<U> FusedIterator for RangeStepInclusive<U> where U: Unit {}

impl<U> IterStep for RangeFrom<U>
where
    U: Unit + RangeDefault,
{
    type Output = RangeStepInclusive<U>;
    type Item = U;
    fn step(self, step: U) -> Self::Output { (self.start..=U::RANGE_MAX).step(step) }
}
impl<U> IterStep for RangeInclusive<U>
where
    U: Unit,
{
    type Output = RangeStepInclusive<U>;
    type Item = U;
    fn step(self, step: U) -> Self::Output
    {
        let (start, end) = self.into_inner();
        RangeStepInclusive { idx: start, end, step }
    }
}
impl<U> IterStep for RangeToInclusive<U>
where
    U: Unit + RangeDefault,
{
    type Output = RangeStepInclusive<U>;
    type Item = U;
    fn step(self, step: U) -> Self::Output { (U::RANGE_MIN..=self.end).step(step) }
}

#[cfg(test)]
mod range_test
{
    use super::*;

    trait HelperToVec : Iterator + Sized
    {
        fn to_vec(self) -> Vec<Self::Item> { self.collect() } 
    }
    impl<I> HelperToVec for I where I: Iterator + Sized {}

    #[test]
    fn range()
    {
        assert_eq!((-2..5).step(1).to_vec(), vec![-2, -1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn range_rev()
    {
        assert_eq!((-2..5).step(1).rev().to_vec(), vec![4, 3, 2, 1, 0, -1, -2]);
    }

    #[test]
    fn range_inclusive()
    {
        assert_eq!((-2..=5).step(1).to_vec(), vec![-2, -1, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn range_inclusive_rev()
    {
        assert_eq!((-2..=5).step(1).rev().to_vec(), vec![5, 4, 3, 2, 1, 0, -1, -2]);
    }

    #[test]
    fn range_float()
    {
        assert_eq!((0.5..2.5).step(1.).to_vec(), vec![0.5, 1.5]);
    }

    #[test]
    fn range_rev_float()
    {
        assert_eq!((0.5..2.5).step(1.).rev().to_vec(), vec![1.5, 0.5]);
    }

    #[test]
    fn range_inclusive_float()
    {
        assert_eq!((0.5..=2.5).step(1.).to_vec(), vec![0.5, 1.5, 2.5]);
    }

    #[test]
    fn range_inclusive_rev_float()
    {
        assert_eq!((0.5..=2.5).step(1.).rev().to_vec(), vec![2.5, 1.5, 0.5]);
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
        let values = (0.0..1.0f32).step(0.3).rev().to_vec();
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
