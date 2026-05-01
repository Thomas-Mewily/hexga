use super::*;

/// Remap the value [`RangeDefault`] to the [`RangeDefault`] of the target type,
/// in a generic friendly way, and similar to the [`From`] trait.
///
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
///
/// One should always prefer implementing [`CastRangeFrom`] over [`CastRangeInto`] because implementing [`CastRangeFrom`] automatically provides one with an implementation of [`CastRangeInto`] thanks to the blanket implementation in the hexga_math library.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(1f32), 255u8);
/// assert_eq!(u8::cast_range_from(0f32), 0u8);
///
/// let casted_range : u16 = u8::MAX.cast_range_into();
/// assert_eq!(casted_range, u16::MAX);
/// ```
///
/// Also work with composite like [`std::array`], [`Vector`]...
///
/// ```ignore
/// use hexga_math::prelude::*;
///
/// let x = [0u8, 127u8, 255u8];
/// let y : [u16; 3] = x.cast_range_into(),
/// assert_eq!(y, [0u16, 32639u16, 65535u16]);
///
///
/// let a = vector3(0u8, 127u8, 255u8);
/// let b : Vector3::<u16> = a.cast_into(),
/// assert_eq!(b, vector3(0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX));
/// ```
pub trait CastRangeFrom<T>
{
    fn cast_range_from(value: T) -> Self;
}
impl<C1, C2> CastRangeFrom<C2> for C1
where
    C1: Map,
    C2: Map<WithType<C1::Item> = Self>,
    C1::Item: CastRangeFrom<C2::Item>,
{
    fn cast_range_from(value: C2) -> Self { value.map(|v| C1::Item::cast_range_from(v)) }
}

/// Remap the value [`RangeDefault`] to the [`RangeDefault`] of the target type,
/// in a generic friendly way, and similar to the [`From`] trait.
///
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
///
/// One should always prefer implementing [`CastRangeFrom`] over [`CastRangeInto`] because implementing [`CastRangeFrom`] automatically provides one with an implementation of [`CastRangeInto`] thanks to the blanket implementation in the hexga_math library.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(u8::cast_range_from(1f32), 255u8);
/// assert_eq!(u8::cast_range_from(0f32), 0u8);
///
/// let casted_range : u16 = u8::MAX.cast_range_into();
/// assert_eq!(casted_range, u16::MAX);
/// ```
///
/// Also work with composite like [`std::array`], [`Vector`]...
///
/// ```ignore
/// use hexga_math::prelude::*;
///
/// let x = [0u8, 127u8, 255u8];
/// let y : [u16; 3] = x.cast_range_into(),
/// assert_eq!(y, [0u16, 32639u16, 65535u16]);
///
///
/// let a = vector3(0u8, 127u8, 255u8);
/// let b : Vector3::<u16> = a.cast_into(),
/// assert_eq!(b, vector3(0u16, u16::MAX / 2 - u8::RANGE_MAX as u16 / 2 - 1, u16::MAX));
/// ```
pub trait CastRangeInto<T>: Sized
{
    fn cast_range_into(self) -> T;
}
impl<S, T> CastRangeInto<T> for S
where
    T: CastRangeFrom<S>,
{
    fn cast_range_into(self) -> T { T::cast_range_from(self) }
}

// Double recursive macro :)
macro_rules! impl_cast_range_to_integer {
    ($src: ty, $dest: ty) => {
        impl CastRangeFrom<$src> for $dest
        {
            #[inline(always)]
            fn cast_range_from(value: $src) -> $dest
            {
                // The match can be inlined by the compiler since it is matching on compile time constant
                match (<$src>::PRIMITIVE_TYPE, <$dest>::PRIMITIVE_TYPE)
                {
                    (NumberType::IntegerSigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>()
                        {
                            return value as $dest;
                        }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            return (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest;
                        }
                        else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    }
                    (NumberType::IntegerSigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<$src>() > std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }
                        else
                        {
                            // up cast or same size
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    }
                    (NumberType::IntegerSigned, NumberType::Float) =>
                    {
                        ((value as $dest - <$src>::RANGE_MIN as $dest) / (<$src>::RANGE as $dest))
                    }
                    (NumberType::IntegerSigned, NumberType::Bool) =>
                    {
                        if (value > <$src>::ZERO)
                        {
                            <$dest>::RANGE_MAX
                        }
                        else
                        {
                            <$dest>::RANGE_MIN
                        }
                    }
                    (NumberType::IntegerUnsigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>()
                        {
                            // same size, but different range
                            return (value / (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest;
                        }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }
                        else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    }
                    (NumberType::IntegerUnsigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>()
                        {
                            return value as $dest;
                        }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }
                        else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    }
                    (NumberType::IntegerUnsigned, NumberType::Float) =>
                    {
                        ((value as $dest - <$src>::RANGE_MIN as $dest) / (<$src>::RANGE as $dest))
                    }
                    (NumberType::IntegerUnsigned, NumberType::Bool) =>
                    {
                        if (value > <$src>::ZERO)
                        {
                            <$dest>::RANGE_MAX
                        }
                        else
                        {
                            <$dest>::RANGE_MIN
                        }
                    }
                    (NumberType::Float, NumberType::IntegerSigned) =>
                    {
                        (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest
                    }
                    (NumberType::Float, NumberType::IntegerUnsigned) =>
                    {
                        (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest
                    }
                    (NumberType::Float, NumberType::Float) =>
                    {
                        (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest
                    }
                    (NumberType::Float, NumberType::Bool) =>
                    {
                        if (value > <$src>::ZERO)
                        {
                            <$dest>::RANGE_MAX
                        }
                        else
                        {
                            <$dest>::RANGE_MIN
                        }
                    }
                    (NumberType::Bool, NumberType::IntegerSigned) =>
                    {
                        if value == <$src>::MIN
                        {
                            <$dest>::RANGE_MIN
                        }
                        else
                        {
                            <$dest>::RANGE_MAX
                        }
                    }
                    (NumberType::Bool, NumberType::IntegerUnsigned) =>
                    {
                        if value == <$src>::MIN
                        {
                            <$dest>::RANGE_MIN
                        }
                        else
                        {
                            <$dest>::RANGE_MAX
                        }
                    }
                    (NumberType::Bool, NumberType::Float) =>
                    {
                        if value == <$src>::MIN
                        {
                            <$dest>::RANGE_MIN
                        }
                        else
                        {
                            <$dest>::RANGE_MAX
                        }
                    }
                    (NumberType::Bool, NumberType::Bool) =>
                    {
                        if (value > <$src>::ZERO)
                        {
                            <$dest>::RANGE_MAX
                        }
                        else
                        {
                            <$dest>::RANGE_MIN
                        }
                    }
                }
            }
        }
    };

    ($dest: ty) => {
        map_on_number!(impl_cast_range_to_integer, $dest);
    };
}
map_on_number!(impl_cast_range_to_integer);

map_on_number!(
    ($type_name: tt) =>
    {
        impl CastRangeFrom<$type_name> for bool
        {
            fn cast_range_from(value: $type_name) -> bool
            {
                value >= $type_name::RANGE_HALF
            }
        }
        impl CastRangeFrom<bool> for $type_name
        {
            fn cast_range_from(value: bool) -> $type_name
            {
                if value { <$type_name>::RANGE_MAX } else { <$type_name>::RANGE_MIN }
            }
        }
    }
);
impl CastRangeFrom<bool> for bool
{
    fn cast_range_from(value: bool) -> Self { value }
}

trait_marker!(
/// fX
CastRangeIntoFloat: CastRangeInto<f32> + CastRangeInto<f64>
);

trait_marker!(
/// fX
CastRangeFromFloat: CastRangeFrom<f32> + CastRangeFrom<f64>
);

trait_marker!(
/// fX
CastRangeFloat: CastRangeIntoFloat + CastRangeFromFloat
);

trait_marker!(
/// uX
CastRangeIntoIntegerUnsigned:
    CastRangeInto<u8 > +
    CastRangeInto<u16> +
    CastRangeInto<u32> +
    CastRangeInto<u64> +
    CastRangeInto<usize>
);

trait_marker!(
/// uX
CastRangeFromIntegerUnsigned:
    CastRangeFrom<u8 > +
    CastRangeFrom<u16> +
    CastRangeFrom<u32> +
    CastRangeFrom<u64> +
    CastRangeFrom<usize>
);

trait_marker!(
/// uX
CastRangeIntegerUnsigned: CastRangeFromIntegerUnsigned + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX
CastRangeIntoIntegerSigned:
    CastRangeInto<i8 > +
    CastRangeInto<i16> +
    CastRangeInto<i32> +
    CastRangeInto<i64> +
    CastRangeInto<isize>
);

trait_marker!(
/// iX
CastRangeFromIntegerSigned :
    CastRangeFrom<i8 > +
    CastRangeFrom<i16> +
    CastRangeFrom<i32> +
    CastRangeFrom<i64> +
    CastRangeFrom<isize>
);

trait_marker!(
/// iX
CastRangeIntegerSigned: CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX
CastRangeIntoInteger: CastRangeIntoIntegerSigned + CastRangeIntoIntegerUnsigned + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX
CastRangeFromInteger: CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX
CastRangeInteger: CastRangeIntoInteger + CastRangeFromInteger+ CastRangeFromIntegerUnsigned
);

trait_marker!(
/// bool
CastRangeIntoBool: CastRangeInto<bool> + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// bool
CastRangeFromBool: CastRangeFrom<bool> + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// bool
CastRangeBool: CastRangeIntoBool + CastRangeFromBool + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX
CastRangeIntoNumber: CastRangeIntoInteger + CastRangeIntoFloat + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX
CastRangeFromNumber: CastRangeFromInteger + CastRangeFromFloat+ CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX
CastRangeNumber: CastRangeInteger + CastRangeFloat + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX bool
CastRangeIntoPrimitive: CastRangeIntoNumber + CastRangeIntoBool + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX bool
CastRangeFromPrimitive: CastRangeFromNumber + CastRangeFromBool + CastRangeFromIntegerUnsigned
);

trait_marker!(
/// iX uX fX bool
CastRangePrimitive: CastRangeIntoPrimitive + CastRangeFromPrimitive + CastRangeFromIntegerUnsigned
);

#[cfg(test)]
mod cast_range_test
{
    use super::*;

    #[test]
    fn identity()
    {
        macro_rules! check_identity {
            ($type_name : ident) => {
                assert_eq!(
                    <$type_name>::cast_range_from(<$type_name>::RANGE_MIN),
                    <$type_name>::RANGE_MIN
                );
                assert_eq!(
                    <$type_name>::cast_range_from(<$type_name>::RANGE_MAX),
                    <$type_name>::RANGE_MAX
                );
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
        assert_eq!(
            u16::cast_range_from(u8::RANGE_MAX / 2),
            u16::RANGE_MAX / 2 - (u8::RANGE_MAX as u16) / 2 - 1
        );
    }

    #[test]
    fn bool_conv()
    {
        macro_rules! check_bool {
            ($type_name : ident) => {
                assert_eq!(
                    <$type_name>::cast_range_from(false),
                    <$type_name>::RANGE_MIN
                );
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
                assert_eq!(
                    <$type_name>::cast_range_from(f32::RANGE_MAX),
                    <$type_name>::RANGE_MAX
                );
                assert_eq!(
                    <$type_name>::cast_range_from(f32::RANGE_MIN),
                    <$type_name>::RANGE_MIN
                );
            };
        }

        map_on_number!(check_float);
    }
}
