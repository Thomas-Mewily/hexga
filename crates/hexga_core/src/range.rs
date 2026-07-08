use std::ops::{Bound, RangeBounds};
use super::*;

/// Range compatibilty type that support `Copy`. Will be replaced by the next `std::range::Range` type when stable.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Range<Idx>
{
    pub start: Idx,
    pub end: Idx,
}
impl<Idx: Debug> Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        self.start.fmt(fmt)?;
        write!(fmt, "..")?;
        self.end.fmt(fmt)?;
        Ok(())
    }
}
impl<Idx> From<std::ops::Range<Idx>> for Range<Idx>
{
    fn from(value: std::ops::Range<Idx>) -> Self {
        Self { start: value.start, end: value.end }
    }
}
impl<Idx> From<Range<Idx>> for std::ops::Range<Idx>
{
    fn from(value: Range<Idx>) -> Self {
        Self { start: value.start, end: value.end }
    }
}

impl<Idx: PartialOrd<Idx>> Range<Idx> {

    pub fn as_std(self) -> std::ops::Range<Idx> { self.into() }

    /// Returns `true` if `item` is contained in the range.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(!(3..5).contains(&2));
    /// assert!( (3..5).contains(&3));
    /// assert!( (3..5).contains(&4));
    /// assert!(!(3..5).contains(&5));
    ///
    /// assert!(!(3..3).contains(&3));
    /// assert!(!(3..2).contains(&3));
    ///
    /// assert!( (0.0..1.0).contains(&0.5));
    /// assert!(!(0.0..1.0).contains(&f32::NAN));
    /// assert!(!(0.0..f32::NAN).contains(&0.5));
    /// assert!(!(f32::NAN..1.0).contains(&0.5));
    /// ```
    #[inline]
    pub fn contains<U>(&self, item: &U) -> bool
    where
        Idx: PartialOrd<U>,
        U: ?Sized + PartialOrd<Idx>,
    {
        <Self as RangeBounds<Idx>>::contains(self, item)
    }

    /// Returns `true` if the range contains no items.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(!(3..5).is_empty());
    /// assert!( (3..3).is_empty());
    /// assert!( (3..2).is_empty());
    /// ```
    ///
    /// The range is empty if either side is incomparable:
    ///
    /// ```
    /// assert!(!(3.0..5.0).is_empty());
    /// assert!( (3.0..f32::NAN).is_empty());
    /// assert!( (f32::NAN..5.0).is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool
    where
        Idx: PartialOrd<Idx>,
    {
        !(self.start < self.end)
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }
    fn end_bound(&self) -> Bound<&T> {
        Bound::Excluded(&self.end)
    }
}
