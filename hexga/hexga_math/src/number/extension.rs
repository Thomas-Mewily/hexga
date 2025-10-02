use super::*;


/// Define the `2` representation for the number
pub trait Two : Sized
{
    // Can't do a `const TWO  : Self; = Self::ONE + Self::ONE` because can't tell rust that the add operator should be const for custom type

    /// There will be a constant TWO later when const trait will be stable
    #[allow(non_snake_case)]
    fn two() -> Self;

    #[inline(always)] fn is_two(&self) -> bool where Self : PartialEq<Self> { self == &Self::two() }
    #[inline(always)] fn is_non_two(&self) -> bool where Self : PartialEq<Self> { !self.is_two() }
}
impl<T> Two for T where T: NumericIdentity
{
    #[inline(always)] fn two() -> Self { Self::ONE + Self::ONE }
}


/// Define the `3` representation for the number
pub trait Three : Sized
{
    // Can't do a `const THREE  : Self; = Self::ONE + Self::ONE + Self::ONE` because can't tell rust that the add operator should be const for custom type

    /// There will be a constant THREE later when const trait will be stable
    #[allow(non_snake_case)]
    fn three() -> Self;

    #[inline(always)] fn is_three(&self) -> bool where Self : PartialEq<Self> { self == &Self::three() }
    #[inline(always)] fn is_non_three(&self) -> bool where Self : PartialEq<Self> { !self.is_three() }
}
impl<T> Three for T where T: One + Add<T,Output = T>
{
    #[inline(always)] fn three() -> Self { Self::ONE + Self::ONE + Self::ONE }
}


pub trait OddOrEven : Sized
{
    fn is_even(&self) -> bool;
    #[inline(always)] fn is_odd (&self) -> bool { !self.is_even() }
}
impl<T> OddOrEven for T where T: Integer
{
    fn is_even(&self) -> bool { (*self % Self::two()).is_zero() }
}


pub trait PositiveOrNegative : Zero + PartialOrd<Self> + Sized
{
    /// Is >= 0
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!((-1).is_positive_or_zero(), false);
    /// debug_assert_eq!(   0.is_positive_or_zero(), true);
    /// debug_assert_eq!(   1.is_positive_or_zero(), true);
    /// ```
    fn is_positive_or_zero(&self) -> bool { self >= &Self::ZERO }

    /// Is <= 0
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!((-1).is_negative_or_zero(), true);
    /// debug_assert_eq!(   0.is_negative_or_zero(), true);
    /// debug_assert_eq!(   1.is_negative_or_zero(), false);
    /// ```
    fn is_negative_or_zero(&self) -> bool { self <= &Self::ZERO }

    /// Is > 0
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!((-1).is_strictly_positive(), false);
    /// debug_assert_eq!(   0.is_strictly_positive(), false);
    /// debug_assert_eq!(   1.is_strictly_positive(), true);
    /// ```
    fn is_strictly_positive(&self) -> bool { self > &Self::ZERO }

    /// Is < 0
    /// ```
    /// use hexga_math::prelude::*;
    /// debug_assert_eq!((-1).is_strictly_negative(), true);
    /// debug_assert_eq!(  0.is_strictly_negative(), false);
    /// debug_assert_eq!(  1.is_strictly_negative(), false);
    /// ```
    fn is_strictly_negative(&self) -> bool { self < &Self::ZERO }
}
impl<T : Zero + PartialOrd<T> + Sized> PositiveOrNegative for T {}

pub trait TakeHalf : Sized
{
    /// Half the current value
    fn take_half(self) -> Self;
}
impl<T> TakeHalf for T where T: NumericIdentity
{
    fn take_half(self) -> Self { self / Self::two() }
}

pub trait PartialOrdExtension : PartialOrd
{
    /// Using [PartialOrd] operator, not component wise
    #[inline(always)] fn max_partial(self, other: Self) -> Self where Self: Sized { if self >= other { self } else { other } }
    /// Using [PartialOrd] operator, not component wise
    #[inline(always)] fn min_partial(self, other: Self) -> Self where Self: Sized { if self <= other { self } else { other } }

    /// Using [PartialOrd] operator, not component wise
    #[inline(always)] fn clamp_partial(self, min: Self, max: Self) -> Self where Self: Sized
    {
        // copied from rust std
        assert!(min <= max);
             if self < min { min }
        else if self > max { max }
        else               { self }
    }
}
impl<T : PartialOrd> PartialOrdExtension for T {}