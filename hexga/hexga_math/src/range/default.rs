use crate::*;

/// For floating the range is : `[0., 1.]`. For integers the range is : `[0, MAX]`, even for signed
pub trait DefaultRange : MinValue + MaxValue //+ CastTo + FromFloat + UnitArithmeticCanBePositive
{ 
    const MIN_RANGE  : Self;
    /// Useful for stuff like Color::GRAY
    const HALF_RANGE : Self;
    const MAX_RANGE  : Self;

    /// `MAX_RANGE - MIN_RANGE`
    const RANGE : Self;
}
macro_rules! impl_have_min_max {
    ($primitive_name: ty) => 
    { impl DefaultRange for $primitive_name 
        { 
            const MIN_RANGE  : Self = Self::ZERO;
            const HALF_RANGE : Self = Self::MAX / 2;
            const MAX_RANGE  : Self = Self::MAX;
            const RANGE      : Self = Self::MAX_RANGE - Self::MIN_RANGE;
        }
    };
}
map_on_integer!(impl_have_min_max);

macro_rules! impl_have_min_max_for_float {
    ($primitive_name: ty) => 
    { impl DefaultRange for $primitive_name 
        { 
            const MIN_RANGE  : Self = Self::ZERO;
            const HALF_RANGE : Self = Self::ONE / 2.;
            const MAX_RANGE  : Self = Self::ONE;
            const RANGE      : Self = Self::MAX_RANGE - Self::MIN_RANGE;
        }
    };
}
map_on_float!(impl_have_min_max_for_float);