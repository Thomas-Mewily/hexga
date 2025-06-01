use crate::*;

/// Remap the value [RangeDefault] to the default range of target type,
/// in a generic friendly way.
///
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
///
/// The [CastRangeIntoComposite] trait is the most generic way to use it.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(1f32), 255u8);
/// assert_eq!(u8::cast_range_from(0f32), 0u8);
/// assert_eq!(u8::cast_range_from(0.5f32), 127u8);
/// ```
///
/// Also support casting to another type with the same size :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(0i8), 0u8);
/// assert_eq!(u8::cast_range_from(127i8), 254u8);
///
/// assert_eq!(i8::cast_range_from(0u8), 0i8);
/// assert_eq!(i8::cast_range_from(255u8), 127i8);
///
/// assert_eq!(u8::cast_range_from(128u8), 128u8);
///
/// assert_eq!(<i8 as CastRangeInto<u8>>::cast_range_into(2i8), 4u8);
/// ```
///
/// Also work with composite like [std::array], [Vector], [ColorRGBA]...
///
/// ```
/// use hexga_math::prelude::*;
///
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, 32639u16, 65535u16]);
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX]);
///
/// assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
///             ColorRGBAOf::<u16>::RED
///            );
/// ```
pub trait CastRangeIntoComposite<T>
{
    type Output;
    /// Might lose some precision.
    fn cast_range_into_composite(self) -> Self::Output;
}

/// Remap the value [RangeDefault] to the default range of target type,
/// in a generic friendly way.
///
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
///
/// The [CastRangeIntoComposite] trait is the most generic way to use it.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(1f32), 255u8);
/// assert_eq!(u8::cast_range_from(0f32), 0u8);
/// assert_eq!(u8::cast_range_from(0.5f32), 127u8);
/// ```
///
/// Also support casting to another type with the same size :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(0i8), 0u8);
/// assert_eq!(u8::cast_range_from(127i8), 254u8);
///
/// assert_eq!(i8::cast_range_from(0u8), 0i8);
/// assert_eq!(i8::cast_range_from(255u8), 127i8);
///
/// assert_eq!(u8::cast_range_from(128u8), 128u8);
///
/// assert_eq!(<i8 as CastRangeInto<u8>>::cast_range_into(2i8), 4u8);
/// ```
///
/// Also work with composite like [std::array], [Vector], [ColorRGBA]...
///
/// ```
/// use hexga_math::prelude::*;
///
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, 32639u16, 65535u16]);
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX]);
///
/// assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
///             ColorRGBAOf::<u16>::RED
///            );
/// ```
pub trait CastRangeInto<T> : CastRangeIntoComposite<T,Output = T> + Sized { fn cast_range_into(self) -> Self::Output { self.cast_range_into_composite() } }
impl<T,T2> CastRangeInto<T> for T2 where T2 : CastRangeIntoComposite<T,Output = T> {}

/// Remap the value [RangeDefault] to the default range of target type,
/// in a generic friendly way.
///
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
///
/// The [CastRangeIntoComposite] trait is the most generic way to use it.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(1f32), 255u8);
/// assert_eq!(u8::cast_range_from(0f32), 0u8);
/// assert_eq!(u8::cast_range_from(0.5f32), 127u8);
/// ```
///
/// Also support casting to another type with the same size :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(0i8), 0u8);
/// assert_eq!(u8::cast_range_from(127i8), 254u8);
///
/// assert_eq!(i8::cast_range_from(0u8), 0i8);
/// assert_eq!(i8::cast_range_from(255u8), 127i8);
///
/// assert_eq!(u8::cast_range_from(128u8), 128u8);
///
/// assert_eq!(<i8 as CastRangeInto<u8>>::cast_range_into(2i8), 4u8);
/// ```
///
/// Also work with composite like [std::array], [Vector], [ColorRGBA]...
///
/// ```
/// use hexga_math::prelude::*;
///
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, 32639u16, 65535u16]);
/// assert_eq!(<[u8;3] as CastRangeIntoComposite<u16>>::cast_range_into_composite([0u8, 127u8, 255u8]), [0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX]);
///
/// assert_eq!(<ColorRGBAOf::<u8> as CastRangeIntoComposite<u16>>::cast_range_into_composite(ColorRGBAOf::<u8>::RED),
///             ColorRGBAOf::<u16>::RED
///            );
/// ```
pub trait CastRangeFrom<T> { fn cast_range_from(value : T) -> Self; }
impl<Src,Dest> CastRangeFrom<Dest> for Src where Dest : CastRangeInto<Src> { fn cast_range_from(value : Dest) -> Self { value.cast_range_into_composite() } }

impl_composite_output_with_methods!(CastRangeIntoComposite<CastToRangeOut>, cast_range_into_composite);



// Double recursive macro :)
macro_rules! impl_cast_range_to_integer
{
    ($itself: ty, $other: ty) =>
    {
        impl CastRangeIntoComposite<$other> for $itself
        {
            type Output = $other;
            #[inline(always)]
            fn cast_range_into_composite(self) -> Self::Output
            {
                // The match can be inlined by the compiler since it is matching on compile time constant
                match (Self::PRIMITIVE_NUMBER_TYPE, <$other>::PRIMITIVE_NUMBER_TYPE)
                {
                    (NumberType::IntegerSigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<Self>() == std::mem::size_of::<$other>() { return self as $other; }
                        if std::mem::size_of::<Self>() >= std::mem::size_of::<$other>()
                        {
                            // down cast
                            return (self * (Self::RANGE / (<$other>::RANGE as $itself))) as $other
                        }else
                        {
                            // up cast
                            ((self as $other) * (<$other>::RANGE / (Self::RANGE as $other)))
                        }
                    },
                    (NumberType::IntegerSigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<Self>() > std::mem::size_of::<$other>()
                        {
                            // down cast
                            (self * (Self::RANGE / (<$other>::RANGE as $itself))) as $other
                        }else
                        {
                            // up cast or same size
                            ((self as $other) * (<$other>::RANGE / (Self::RANGE as $other)))
                        }
                    },
                    (NumberType::IntegerSigned, NumberType::Float) => ((self as $other - Self::RANGE_MIN as $other) / (Self::RANGE as $other)) ,
                    (NumberType::IntegerSigned, NumberType::Bool) => if (self > Self::ZERO) { <$other>::RANGE_MAX } else { <$other>::RANGE_MIN },
                    (NumberType::IntegerUnsigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<Self>() == std::mem::size_of::<$other>()
                        {
                            // same size, but different range
                            return (self / (Self::RANGE / (<$other>::RANGE as $itself))) as $other
                        }
                        if std::mem::size_of::<Self>() >= std::mem::size_of::<$other>()
                        {
                            // down cast
                            (self * (Self::RANGE / (<$other>::RANGE as $itself))) as $other
                        }else
                        {
                            // up cast
                            ((self as $other) * (<$other>::RANGE / (Self::RANGE as $other)))
                        }
                    },
                    (NumberType::IntegerUnsigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<Self>() == std::mem::size_of::<$other>() { return self as $other; }
                        if std::mem::size_of::<Self>() >= std::mem::size_of::<$other>()
                        {
                            // down cast
                            (self * (Self::RANGE / (<$other>::RANGE as $itself))) as $other
                        }else
                        {
                            // up cast
                            ((self as $other) * (<$other>::RANGE / (Self::RANGE as $other)))
                        }
                    },
                    (NumberType::IntegerUnsigned, NumberType::Float) => ((self as $other - Self::RANGE_MIN as $other) / (Self::RANGE as $other)),
                    (NumberType::IntegerUnsigned, NumberType::Bool) => if (self > Self::ZERO) { <$other>::RANGE_MAX } else { <$other>::RANGE_MIN },
                    (NumberType::Float, NumberType::IntegerSigned) => (self * (<$other>::RANGE as Self) + (<$other>::RANGE_MIN as Self)) as $other,
                    (NumberType::Float, NumberType::IntegerUnsigned) => (self * (<$other>::RANGE as Self) + (<$other>::RANGE_MIN as Self)) as $other,
                    (NumberType::Float, NumberType::Float) => (self * (<$other>::RANGE as Self) + (<$other>::RANGE_MIN as Self)) as $other,
                    (NumberType::Float, NumberType::Bool) => if (self > Self::ZERO) { <$other>::RANGE_MAX } else { <$other>::RANGE_MIN },
                    (NumberType::Bool, NumberType::IntegerSigned) => if self == Self::MIN { <$other>::RANGE_MIN } else { <$other>::RANGE_MAX },
                    (NumberType::Bool, NumberType::IntegerUnsigned) => if self == Self::MIN { <$other>::RANGE_MIN } else { <$other>::RANGE_MAX },
                    (NumberType::Bool, NumberType::Float) => if self == Self::MIN { <$other>::RANGE_MIN } else { <$other>::RANGE_MAX },
                    (NumberType::Bool, NumberType::Bool) => if (self > Self::ZERO) { <$other>::RANGE_MAX } else { <$other>::RANGE_MIN },
                }
            }
        }
    };

    ($other: ty) =>
    {
        map_on_number!(impl_cast_range_to_integer,$other);
    };
}
map_on_number!(impl_cast_range_to_integer);

map_on_number!(
    ($type_name: tt) =>
    {
        impl CastRangeIntoComposite<$type_name> for bool
        {
            type Output = $type_name;
            fn cast_range_into_composite(self) -> Self::Output
            {
                if self { <$type_name>::RANGE_MAX } else { <$type_name>::RANGE_MIN }
            }
        }
        impl CastRangeIntoComposite<bool> for $type_name
        {
            type Output = bool;
            fn cast_range_into_composite(self) -> Self::Output
            {
                self > Self::ZERO
            }
        }
    }
);
impl CastRangeIntoComposite<bool> for bool
{
    type Output = bool;
    fn cast_range_into_composite(self) -> Self::Output {
        self
    }
}




/// fX
pub trait CastRangeIntoFloat             : CastRangeInto<f32> + CastRangeInto<f64> {}
impl<T> CastRangeIntoFloat for T where T: CastRangeInto<f32> + CastRangeInto<f64> {}

/// fX
pub trait CastRangeFromFloat             : CastRangeFrom<f32> + CastRangeFrom<f64> {}
impl<T> CastRangeFromFloat for T where T: CastRangeFrom<f32> + CastRangeFrom<f64> {}

/// fX
pub trait CastRangeFloat             : CastRangeIntoFloat + CastRangeFromFloat {}
impl<T> CastRangeFloat for T where T: CastRangeIntoFloat + CastRangeFromFloat {}

/// uX
pub trait CastRangeIntoIntegerUnsigned :
    CastRangeInto<u8 > +
    CastRangeInto<u16> +
    CastRangeInto<u32> +
    CastRangeInto<u64> +
    CastRangeInto<usize>
{}
impl<T> CastRangeIntoIntegerUnsigned for T where T:
    CastRangeInto<u8 > +
    CastRangeInto<u16> +
    CastRangeInto<u32> +
    CastRangeInto<u64> +
    CastRangeInto<usize>
{}

/// uX
pub trait CastRangeFromIntegerUnsigned :
    CastRangeFrom<u8 > +
    CastRangeFrom<u16> +
    CastRangeFrom<u32> +
    CastRangeFrom<u64> +
    CastRangeFrom<usize>
{}
impl<T> CastRangeFromIntegerUnsigned for T where T:
    CastRangeFrom<u8 > +
    CastRangeFrom<u16> +
    CastRangeFrom<u32> +
    CastRangeFrom<u64> +
    CastRangeFrom<usize>
{}

/// uX
pub trait CastRangeIntegerUnsigned             : CastRangeFromIntegerUnsigned + CastRangeFromIntegerUnsigned {}
impl<T> CastRangeIntegerUnsigned for T where T: CastRangeFromIntegerUnsigned + CastRangeFromIntegerUnsigned {}


/// iX
pub trait CastRangeIntoIntegerSigned :
    CastRangeInto<i8 > +
    CastRangeInto<i16> +
    CastRangeInto<i32> +
    CastRangeInto<i64> +
    CastRangeInto<isize>
{}
impl<T> CastRangeIntoIntegerSigned for T where T:
    CastRangeInto<i8 > +
    CastRangeInto<i16> +
    CastRangeInto<i32> +
    CastRangeInto<i64> +
    CastRangeInto<isize>
{}

/// iX
pub trait CastRangeFromIntegerSigned :
    CastRangeFrom<i8 > +
    CastRangeFrom<i16> +
    CastRangeFrom<i32> +
    CastRangeFrom<i64> +
    CastRangeFrom<isize>
{}
impl<T> CastRangeFromIntegerSigned for T where T:
    CastRangeFrom<i8 > +
    CastRangeFrom<i16> +
    CastRangeFrom<i32> +
    CastRangeFrom<i64> +
    CastRangeFrom<isize>
{}

/// iX
pub trait CastRangeIntegerSigned             : CastRangeFromIntegerSigned + CastRangeFromIntegerSigned {}
impl<T> CastRangeIntegerSigned for T where T: CastRangeFromIntegerSigned + CastRangeFromIntegerSigned {}


/// iX uX
pub trait CastRangeIntoInteger             : CastRangeIntoIntegerSigned + CastRangeIntoIntegerUnsigned {}
impl<T> CastRangeIntoInteger for T where T: CastRangeIntoIntegerSigned + CastRangeIntoIntegerUnsigned {}

/// iX uX
pub trait CastRangeFromInteger             : CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned {}
impl<T> CastRangeFromInteger for T where T: CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned {}

/// iX uX
pub trait CastRangeInteger             : CastRangeIntoInteger + CastRangeFromInteger {}
impl<T> CastRangeInteger for T where T: CastRangeIntoInteger + CastRangeFromInteger {}


/// bool
pub trait CastRangeIntoBool             : CastRangeInto<bool> {}
impl<T> CastRangeIntoBool for T where T: CastRangeInto<bool> {}

/// bool
pub trait CastRangeFromBool             : CastRangeFrom<bool> {}
impl<T> CastRangeFromBool for T where T: CastRangeFrom<bool> {}

/// bool
pub trait CastRangeBool             : CastRangeIntoBool + CastRangeFromBool {}
impl<T> CastRangeBool for T where T: CastRangeIntoBool + CastRangeFromBool {}


/// iX uX fX
pub trait CastRangeIntoNumber             : CastRangeIntoInteger + CastRangeIntoFloat {}
impl<T> CastRangeIntoNumber for T where T: CastRangeIntoInteger + CastRangeIntoFloat {}

/// iX uX fX
pub trait CastRangeFromNumber             : CastRangeFromInteger + CastRangeFromFloat {}
impl<T> CastRangeFromNumber for T where T: CastRangeFromInteger + CastRangeFromFloat {}

/// iX uX fX
pub trait CastRangeNumber             : CastRangeInteger + CastRangeFloat {}
impl<T> CastRangeNumber for T where T: CastRangeInteger + CastRangeFloat {}


/// iX uX fX bool
pub trait CastRangeIntoPrimitive             : CastRangeIntoNumber + CastRangeIntoBool {}
impl<T> CastRangeIntoPrimitive for T where T: CastRangeIntoNumber + CastRangeIntoBool {}

/// iX uX fX bool
pub trait CastRangeFromPrimitive             : CastRangeFromNumber + CastRangeFromBool {}
impl<T> CastRangeFromPrimitive for T where T: CastRangeFromNumber + CastRangeFromBool {}

/// iX uX fX bool
pub trait CastRangePrimitive             : CastRangeIntoPrimitive + CastRangeFromPrimitive {}
impl<T> CastRangePrimitive for T where T: CastRangeIntoPrimitive + CastRangeFromPrimitive {}


#[cfg(test)]
mod cast_range_test
{
    use super::*;

    #[test]
    fn identity()
    {
        macro_rules! check_identity {
            ($type_name : ident) => {
                assert_eq!(<$type_name>::cast_range_from(<$type_name>::RANGE_MIN), <$type_name>::RANGE_MIN);
                assert_eq!(<$type_name>::cast_range_from(<$type_name>::RANGE_MAX), <$type_name>::RANGE_MAX);
            };
        }

        map_on_number!(check_identity);
        assert_eq!(bool::cast_range_from(false), false);
        assert_eq!(bool::cast_range_from(true), true);
    }

    #[test]
    fn some_range_test()
    {
        assert_eq!(u16::cast_range_from(0u8), 0u16);
        assert_eq!(u16::cast_range_from(u8::RANGE_MAX), u16::RANGE_MAX);
        assert_eq!(u16::cast_range_from(u8::RANGE_MAX / 2), u16::RANGE_MAX / 2 - (u8::RANGE_MAX as u16) / 2 - 1);
    }

    #[test]
    fn bool_conv()
    {
        macro_rules! check_bool {
            ($type_name : ident) => {
                assert_eq!(<$type_name>::cast_range_from(false), <$type_name>::RANGE_MIN);
                assert_eq!(<$type_name>::cast_range_from(true), <$type_name>::RANGE_MAX);
            };
        }

        map_on_number!(check_bool);
    }

    #[test]
    fn float_conv()
    {
        macro_rules! check_float {
            ($type_name : ident) => {
                assert_eq!(<$type_name>::cast_range_from(f32::RANGE_MAX), <$type_name>::RANGE_MAX);
                assert_eq!(<$type_name>::cast_range_from(f32::RANGE_MIN), <$type_name>::RANGE_MIN);
            };
        }

        map_on_number!(check_float);
    }
}