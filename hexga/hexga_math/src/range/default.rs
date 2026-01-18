use super::*;


/// For floating the range is : `[0., 1.]`. For integers the range is : `[0, MAX]`, even for signed
pub trait RangeDefault : MinValue + MaxValue //+ CastTo + FromFloat + UnitArithmeticCanBePositive
{ 
    const RANGE_MIN  : Self;
    /// Useful for stuff like Color::GRAY
    /// 
    /// `RANGE / 2 + RANGE`
    const RANGE_HALF : Self;
    const RANGE_MAX  : Self;

    /// `RANGE_MAX - RANGE_MIN`
    const RANGE : Self;

    // `Self::RANGE_MIN..Self::RANGE_MAX`
    fn range() -> Range<Self> { Self::RANGE_MIN..Self::RANGE_MAX }
    // `Self::RANGE_MIN..=Self::RANGE_MAX`
    fn range_inclusive() -> RangeInclusive<Self> { Self::RANGE_MIN..=Self::RANGE_MAX }
}
macro_rules! impl_have_min_max {
    ($primitive_name: ty) => 
    { impl RangeDefault for $primitive_name 
        { 
            const RANGE_MIN  : Self = Self::ZERO;
            const RANGE_HALF : Self = Self::MAX / 2;
            const RANGE_MAX  : Self = Self::MAX;
            const RANGE      : Self = Self::RANGE_MAX - Self::RANGE_MIN;
        }
    };
}
map_on_integer!(impl_have_min_max);

macro_rules! impl_have_min_max_for_float {
    ($primitive_name: ty) => 
    { impl RangeDefault for $primitive_name 
        { 
            const RANGE_MIN  : Self = Self::ZERO;
            const RANGE_HALF : Self = Self::ONE / 2.;
            const RANGE_MAX  : Self = Self::ONE;
            const RANGE      : Self = Self::RANGE_MAX - Self::RANGE_MIN;
        }
    };
}
map_on_float!(impl_have_min_max_for_float);

impl RangeDefault for bool
{
    const RANGE_MIN  : Self = false;
    const RANGE_HALF : Self = false;
    const RANGE_MAX  : Self = true;
    const RANGE : Self = true;
}