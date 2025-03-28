use crate::*;

impl_composite_constant_for_internal_type!(Zero, ZERO);

impl_composite_constant_for_internal_type!(One, ONE);

impl_composite_constant_for_internal_type!(Half, HALF);

impl_composite_constant_for_internal_type!(NaNValue, NAN);

impl_composite_constant_for_internal_type!(MinValue, MIN);
impl_composite_constant_for_internal_type!(MaxValue, MAX);

/// Define the `2` representation for the number
pub trait Two : Sized
{ 
    //const TWO  : Self;

    // Can't do a `const TWO  : Self; = Self::ONE + Self::ONE` because the add operator is non const
    #[allow(non_snake_case)]
    fn two() -> Self;
    
    #[inline] fn is_two(&self) -> bool where Self : PartialEq<Self> { self == &Self::two() }
    #[inline] fn is_non_two(&self) -> bool where Self : PartialEq<Self> { !self.is_two() }
}
impl<T> Two for T where T : One + Add<T,Output = T>
{
    // There will be a constant TWO later when const trait will be stable
    #[inline] fn two() -> Self { Self::ONE + Self::ONE }
}

pub trait OddOrEven : Sized
{
    fn is_even(&self) -> bool;
    #[inline] fn is_odd (&self) -> bool { !self.is_even() }
}
impl<T> OddOrEven for T where T : Integer
{
    fn is_even(&self) -> bool { (*self % Self::two()).is_zero() }
}

pub trait PositiveOrNegative : Zero + PartialOrd<Self> + Sized
{ 
    /// Is >= 0
    /// ```
    /// use hexga_math::*;
    /// debug_assert_eq!((-1).is_positive_or_zero(), false);
    /// debug_assert_eq!(   0.is_positive_or_zero(), true);
    /// debug_assert_eq!(   1.is_positive_or_zero(), true);
    /// ```
    fn is_positive_or_zero(&self) -> bool { self >= &Self::ZERO }

    /// Is <= 0
    /// ```
    /// use hexga_math::*;
    /// debug_assert_eq!((-1).is_negative_or_zero(), true);
    /// debug_assert_eq!(   0.is_negative_or_zero(), true);
    /// debug_assert_eq!(   1.is_negative_or_zero(), false);
    /// ```
    fn is_negative_or_zero(&self) -> bool { self <= &Self::ZERO }

    /// Is > 0
    /// ```
    /// use hexga_math::*;
    /// debug_assert_eq!((-1).is_strictly_positive(), false);
    /// debug_assert_eq!(   0.is_strictly_positive(), false);
    /// debug_assert_eq!(   1.is_strictly_positive(), true);
    /// ```
    fn is_strictly_positive(&self) -> bool { self > &Self::ZERO }

    /// Is < 0
    /// ```
    /// use hexga_math::*;
    /// debug_assert_eq!((-1).is_strictly_negative(), true);
    /// debug_assert_eq!(  0.is_strictly_negative(), false);
    /// debug_assert_eq!(  1.is_strictly_negative(), false);
    /// ```
    fn is_strictly_negative(&self) -> bool { self < &Self::ZERO }
}
impl<T : Zero + PartialOrd<T> + Sized> PositiveOrNegative for T {}


pub trait PartialOrdExtension : PartialOrd
{
    #[inline] fn max_partial(self, other: Self) -> Self where Self: Sized { if self >= other { self } else { other } }
    #[inline] fn min_partial(self, other: Self) -> Self where Self: Sized { if self <= other { self } else { other } }

    #[inline] fn clamp_partial(self, min: Self, max: Self) -> Self where Self: Sized
    {
        // copied from rust std
        assert!(min <= max);
             if self < min { min } 
        else if self > max { max } 
        else               { self }
    }
}
impl<T : PartialOrd> PartialOrdExtension for T{}

pub trait Absolute 
{
    fn abs(self) -> Self;
}

macro_rules! impl_abs_is_itself {
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name { #[inline] fn abs(self) -> Self { self }} };
}
map_on_integer_unsigned!(impl_abs_is_itself);

macro_rules! impl_abs {
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name {#[inline] fn abs(self) -> Self { Self::abs(self) }}};
}
map_on_integer_signed!(impl_abs);
map_on_float!(impl_abs);
impl_composite_with_methods!(Absolute,(abs,Self));


pub trait TakeHalf : Sized
{
    /// Half the current value
    fn take_half(self) -> Self;
}
impl<T> TakeHalf for T where T :  Number
{
    fn take_half(self) -> Self { self / Self::two() }
}