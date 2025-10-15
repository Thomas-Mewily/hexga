use super::*;

// Most function are suffixed with `_elementwise` like min, max, clamp, because there are already defiend in [std::cmp::Ord] and I don't want to fully qualify them...


/// Max function in a component way
pub trait Max
{
    /// Max function in a component way
    ///
    /// Can cause a panic for grid if the size mismatch
    fn max_elementwise(self, other: Self) -> Self;
}
/// Max function in a component way
///
/// Can cause a panic for grid if the size mismatch
pub fn max<S>(a: S, b:S) -> S where S:Max  { a.max_elementwise(b) }


map_on_integer!(
    ($type_name: tt) =>
    {
        impl Max for $type_name
        {
            fn max_elementwise(self, other: Self) -> Self { Ord::max(self, other) }
        }
    }
);
map_on_float!(
    ($type_name: tt) =>
    {
        impl Max for $type_name
        {
            fn max_elementwise(self, other: Self) -> Self { Self::max(self, other) }
        }
    }
);
impl Max for bool
{
    fn max_elementwise(self, other: Self) -> Self { self | other }
}

impl<S> Max for S where S:MapWithIntern, S::Item : Max
{
    fn max_elementwise(self, other: Self) -> Self { self.map_with_intern(other, Max::max_elementwise) }
}



/// Min function in a component way
pub trait Min
{
    /// Min function in a component way
    ///
    /// Can cause a panic for grid if the size mismatch
    fn min_elementwise(self, other: Self) -> Self;
}
/// Min function in a component way
///
/// Can cause a panic for grid if the size mismatch
pub fn min<S>(a: S, b:S) -> S where S:Min  { a.min_elementwise(b) }

map_on_integer!(
    ($type_name: tt) =>
    {
        impl Min for $type_name
        {
            fn min_elementwise(self, other: Self) -> Self { Ord::min(self, other) }
        }
    }
);
map_on_float!(
    ($type_name: tt) =>
    {
        impl Min for $type_name
        {
            fn min_elementwise(self, other: Self) -> Self { Self::min(self, other) }
        }
    }
);
impl Min for bool
{
    fn min_elementwise(self, other: Self) -> Self { self & other }
}

impl<S> Min for S where S:MapWithIntern, S::Item : Min
{
    fn min_elementwise(self, other: Self) -> Self { self.map_with_intern(other, Min::min_elementwise) }
}



/// Clamp function in a component way
pub trait Clamp
{
    /// Clamp function in a component way
    ///
    /// Clamps self between min_val and max_val (inclusive)
    ///
    /// Can cause a panic for grid if the size mismatch
    fn clamp_elementwise(self, min_val: Self, max_val: Self) -> Self;
}
    /// Clamp function in a component way
    ///
    /// Clamps self between min_val and max_val (inclusive)
    ///
    /// Can cause a panic for grid if the size mismatch
pub fn clamp<S>(value: S, min_val: S, max_val: S) -> S where S:Clamp  { value.clamp_elementwise(min_val, max_val) }

impl<T> Clamp for T
where
    T: Min + Max,
{
    fn clamp_elementwise(self, min_val: Self, max_val: Self) -> Self {
        self.max_elementwise(min_val).min_elementwise(max_val)
    }
}




/// Mix function in a component way
pub trait Mix<C=float> : Sized where C: Float
{
    /// Mix function in a component way
    ///
    /// The coef is clamped between `0.0..1.`
    ///
    /// 0. => self,
    /// 1. => dest.
    ///
    /// Can cause a panic for grid if the size mismatch
    fn mix(self, dest: Self, coef: C) -> Self { self.mix_unchecked(dest, coef.clamp_partial(zero(), one())) }

    /// Mix function in a component way
    ///
    /// The coef is **not** clamped between `0.0..1.`
    ///
    /// 0. => self,
    /// 1. => dest.
    ///
    /// Can cause a panic for grid if the size mismatch
    fn mix_unchecked(self, dest: Self, coef: C) -> Self;
}
/// Mix function in a component way
///
/// The coef is clamped between `0.0..1.`
///
/// 0. => self,
/// 1. => dest.
///
/// Can cause a panic for grid if the size mismatch
pub fn mix<S,F>(src: S, dest: S, coef: F) -> S where S:Mix<F>, F:Float
{
    src.mix(dest, coef)
}


map_on_number_and_bool!(
    ($type_name: tt) =>
    {
        impl<F> Mix<F> for $type_name where F: Float
        {
            fn mix_unchecked(self, dest: Self, coef: F) -> Self
            {
                let src  = F::cast_from(self);
                let dest = F::cast_from(dest);
                let r = src * (F::ONE - coef) + dest * coef;
                r.cast_into()
            }
        }
    }
);
impl<S,F> Mix<F> for S where S:MapWithIntern, S::Item : Mix<F>, F:Float
{
    fn mix_unchecked(self, dest: Self, coef: F) -> Self
    {
        self.map_with_intern(dest, |a,b| a.mix_unchecked(b, coef))
    }
}


/// Abs function in a component way
pub trait Abs
{
    type Output;
    /// Absolute value in a component way.
    ///
    /// ## Overflow behavior
    /// The absolute value of `iX::MIN` cannot be represented as an `iX`, and attempting to calculate it will cause an overflow.
    /// This means that code in debug mode will trigger a panic on this case and optimized code will return `iX::MIN` without a panic.
    ///
    /// Can cause a panic for grid if the size mismatch
    fn abs(self) -> Self::Output;
}
map_on_integer_unsigned!(
    ($primitive_name: ty) =>
    {
        impl Abs for $primitive_name
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { self }
        }
        impl Abs for Saturating<$primitive_name>
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { self }
        }
        impl Abs for Wrapping<$primitive_name>
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { self }
        }
    }
);
map_on_integer_signed!(
    ($primitive_name: ty) =>
    {
        impl Abs for $primitive_name
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { self.abs() }
        }
        impl Abs for Saturating<$primitive_name>
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { Self::abs(self) }
        }
        impl Abs for Wrapping<$primitive_name>
        {
            type Output = Self;
            // TODO: replace it by Wrapping<$primitive_name>::abs once stabilized
            #[inline(always)]
            fn abs(self) -> Self { if self.is_max_value() { Self(<$primitive_name>::MIN) } else { Self(<$primitive_name>::abs(self.0)) } }
        }
    }
);
map_on_float!(
    ($primitive_name: ty) =>
    {
        impl Abs for $primitive_name
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { self.abs() }
        }
        impl Abs for Saturating<$primitive_name>
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { Self::abs(self) }
        }
        impl Abs for Wrapping<$primitive_name>
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self { Self::abs(self) }
        }
    }
);
impl Abs for bool
{
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self { self }
}
impl Abs for Saturating<bool>
{
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self { self }
}
impl Abs for Wrapping<bool>
{
    type Output = Self;
    #[inline(always)]
    fn abs(self) -> Self { self }
}
impl<S> Abs for S where S:Map, S::Item : Abs
{
    type Output = S::WithType<<S::Item as Abs>::Output>;
    fn abs(self) -> Self::Output {
        self.map(Abs::abs)
    }
}
/// Absolute value in a component way.
///
/// ## Overflow behavior
/// The absolute value of `iX::MIN` cannot be represented as an `iX`, and attempting to calculate it will cause an overflow.
/// This means that code in debug mode will trigger a panic on this case and optimized code will return `iX::MIN` without a panic.
///
/// Can cause a panic for grid if the size mismatch
pub fn abs<S>(value: S) -> S::Output where S:Abs
{
    value.abs()
}




#[macro_export]
macro_rules! impl_number_basic_trait
{
    () => {

        // NOTE: Most of these function clash with at least 2 traits (ex: max() is defined in the [`std::cmp::Ord`] (non component wise) and also in [`crate::number::Max`] (component wise)
        // Redefining them here and redirecting them to the correct trait avoid the need to fully qualify the method with the trait in order to call it

        /// Max function in a component way
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn max(self, other: Self) -> Self where Self: $crate::number::Max { $crate::number::Max::max_elementwise(self, other) }

        /// Min function in a component way
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn min(self, other: Self) -> Self where Self: $crate::number::Min { $crate::number::Min::min_elementwise(self, other) }

        /// Absolute value in a component way.
        ///
        /// ## Overflow behavior
        /// The absolute value of `iX::MIN` cannot be represented as an `iX`, and attempting to calculate it will cause an overflow.
        /// This means that code in debug mode will trigger a panic on this case and optimized code will return `iX::MIN` without a panic.
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn abs(self) -> <Self as $crate::number::Abs>::Output where Self: $crate::number::Abs { $crate::number::Abs::abs(self) }

        /// Mix function in a component way
        ///
        /// The coef is clamped between `0.0..1.`
        ///
        /// 0. => self,
        /// 1. => dest.
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn mix<C>(self, other: Self, coef: C) -> Self where Self: $crate::number::Mix<C>, C: Float { $crate::number::Mix::mix(self, other, coef) }

        /// Mix function in a component way
        ///
        /// The coef is **not** clamped between `0.0..1.`
        ///
        /// 0. => self,
        /// 1. => dest.
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn mix_unchecked<C>(self, other: Self, coef: C) -> Self where Self: $crate::number::Mix<C>, C: Float { $crate::number::Mix::mix_unchecked(self, other, coef) }

        /// Clamp function in a component way
        ///
        /// Clamps self between min_val and max_val (inclusive)
        ///
        /// Can cause a panic for grid if the size mismatch
        pub fn clamp(self, min_val: Self, max_val: Self) -> Self where Self: $crate::number::Clamp { $crate::number::Clamp::clamp_elementwise(self, min_val, max_val) }
    };
}



#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn max_test()
    {
        assert_eq!(max(1, 2), 2);
        assert_eq!(max(2, 1), 2);
        assert_eq!(max([1,10], [4,5]), [4,10]);
        assert_eq!(max([10,1], [5,4]), [10,4]);
    }


    #[test]
    fn min_test()
    {
        assert_eq!(min(1, 2), 1);
        assert_eq!(min(2, 1), 1);
        assert_eq!(min([1,10], [4,5]), [1,5]);
        assert_eq!(min([10,1], [5,4]), [5,1]);
    }

    #[test]
    fn mix_test()
    {
        assert_eq!(mix(0., 1., 0.), 0.);
        assert_eq!(mix(0., 1., 0.5), 0.5);
        assert_eq!(mix(0., 1., 1.), 1.);

        assert_eq!(mix(0., 100., 0.), 0.);
        assert_eq!(mix(0., 100., 1.), 100.);

        assert_eq!(mix(1., 0., 0.), 1.);
        assert_eq!(mix(1., 0., 0.5), 0.5);
        assert_eq!(mix(1., 0., 1.), 0.);

        assert_eq!(mix([1.,2.], [4.,8.], 0.5), [2.5,5.]);
    }


    #[test]
    fn abs_test()
    {
        assert_eq!(abs(-2), 2);
        assert_eq!(abs(u8::MAX), u8::MAX);
        assert_eq!(abs(i8::MAX), i8::MAX);
        assert_eq!(abs(i8::MIN+1), i8::MAX);
        assert_eq!(abs(i8::MIN+1), i8::MAX);
    }

    #[test]
    fn clamp_test()
    {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
        assert_eq!(clamp(0, 0, 10), 0);
        assert_eq!(clamp(10, 0, 10), 10);

        assert_eq!(clamp(2.5, 1.0, 3.0), 2.5);
        assert_eq!(clamp(0.5, 1.0, 3.0), 1.0);
        assert_eq!(clamp(3.5, 1.0, 3.0), 3.0);

        assert_eq!(clamp([1, 5, 10], [2, 3, 8], [4, 7, 12]), [2, 5, 10]);
        assert_eq!(clamp([0, 2, 15], [1, 1, 10], [3, 5, 20]), [1, 2, 15]);

        assert_eq!(clamp(true, false, true), true);
        assert_eq!(clamp(true, false, false), false);
        assert_eq!(clamp(false, true, true), true);
        assert_eq!(clamp(false, false, false), false);
    }
}