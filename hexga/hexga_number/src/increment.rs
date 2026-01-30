use super::*;

/// The `+1` operation
pub trait Increment: One + Add<Self, Output = Self> + AddAssign<Self> + Copy + Sized
{
    /// The increment `x++` operator
    #[inline(always)]
    fn increment(&mut self) { *self += Self::ONE; }
    /// The increment `x++` operator
    #[inline(always)]
    fn increment_checked(&mut self) -> Result<(), ()>
    where
        Self: OverflowBehavior + MaxValue + PartialEq,
    {
        if self.can_increment()
        {
            self.increment();
            Ok(())
        }
        else
        {
            Err(())
        }
    }

    /// Do the current value have a successor.
    ///
    /// True if not Self::MAX, except for wrapping type, they always have a successor because they wrap
    #[inline(always)]
    fn can_increment(&self) -> bool
    where
        Self: OverflowBehavior + MaxValue + PartialEq,
    {
        Self::OVERFLOW_BEHAVIOR.is_wrapping() || self.is_not_max()
    }

    /// Return the successor `self + 1`
    #[inline(always)]
    fn successor(self) -> Self { self + Self::ONE }
    /// Return the successor `self + 1`
    #[inline(always)]
    fn successor_checked(&mut self) -> Result<Self, ()>
    where
        Self: OverflowBehavior + MaxValue + PartialEq,
    {
        if self.have_successor()
        {
            Ok(self.successor())
        }
        else
        {
            Err(())
        }
    }

    /// Do the current value have a successor.
    ///
    /// True if not Self::MAX, except for wrapping type, they always have a successor because they wrap
    #[inline(always)]
    fn have_successor(self) -> bool
    where
        Self: OverflowBehavior + MaxValue + PartialEq,
    {
        self.can_increment()
    }
}
impl<T> Increment for T where T: One + Add<T, Output = T> + AddAssign<Self> + Copy {}

/// The `-1` operation
pub trait Decrement: One + Sub<Self, Output = Self> + SubAssign<Self> + Copy + Sized
{
    /// The decrement `x--` operator
    #[inline(always)]
    fn decrement(&mut self) { *self -= Self::ONE; }
    /// The increment `x--` operator
    #[inline(always)]
    fn decrement_checked(&mut self) -> Result<(), ()>
    where
        Self: OverflowBehavior + MinValue + PartialEq,
    {
        if self.can_decrement()
        {
            self.decrement();
            Ok(())
        }
        else
        {
            Err(())
        }
    }

    /// Do the current value have a predecessor.
    ///
    /// True if not Self::MIN, except for wrapping type, they always have a predecessor because they wrap
    #[inline(always)]
    fn can_decrement(&self) -> bool
    where
        Self: OverflowBehavior + MinValue + PartialEq,
    {
        Self::OVERFLOW_BEHAVIOR.is_wrapping() || self.is_not_min()
    }

    /// Return the predecessor `self - 1`
    #[inline(always)]
    fn predecessor(self) -> Self { self - Self::ONE }
    /// Return the predecessor `self - 1`
    #[inline(always)]
    fn predecessor_checked(&mut self) -> Result<Self, ()>
    where
        Self: OverflowBehavior + MinValue + PartialEq,
    {
        if self.have_predecessor()
        {
            Ok(self.predecessor())
        }
        else
        {
            Err(())
        }
    }

    /// Do the current value have a predecessor.
    ///
    /// True if not Self::MIN, except for wrapping type, they always have a predecessor because they wrap
    #[inline(always)]
    fn have_predecessor(self) -> bool
    where
        Self: OverflowBehavior + MinValue + PartialEq,
    {
        self.can_decrement()
    }
}
impl<T> Decrement for T where T: One + Sub<Self, Output = Self> + SubAssign<Self> + Copy + Sized {}
