use super::*;



/// Remap the value [RangeDefault] to the default range of target type,
/// in a generic friendly way, and similar to the [From] trait.
/// 
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
/// 
/// One should always prefer implementing [CastRangeFrom] over [CastRangeInto] because implementing [CastRangeFrom] automatically provides one with an implementation of [CastRangeInto] thanks to the blanket implementation in the hexga_math library.
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
/// Also work with composite like [std::array], [Vector]...
///
/// ```rust
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
    fn cast_range_from(value : T) -> Self; 
}
impl<C1,C2> CastRangeFrom<C2> for C1 where C1: CompositeGeneric, C2: CompositeGeneric<WithType<C1::Inside> = Self>, C1::Inside : CastRangeFrom<C2::Inside>
{
    fn cast_range_from(value : C2) -> Self 
    {
        value.map(|v| C1::Inside::cast_range_from(v))
    }
}


/// Remap the value [RangeDefault] to the default range of target type,
/// in a generic friendly way, and similar to the [From] trait.
/// 
/// `[Self::RANGE_MIN..Self::RANGE_MAX]` => `[T::RANGE_MIN..T::RANGE_MAX]`
/// 
/// One should always prefer implementing [CastRangeFrom] over [CastRangeInto] because implementing [CastRangeFrom] automatically provides one with an implementation of [CastRangeInto] thanks to the blanket implementation in the hexga_math library.
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
/// Also work with composite like [std::array], [Vector]...
///
/// ```rust
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
pub trait CastRangeInto<T> : Sized 
{ 
    fn cast_range_into(self) -> T; 
}
impl<S,T> CastRangeInto<T> for S where T:CastRangeFrom<S>
{
    fn cast_range_into(self) -> T {
        T::cast_range_from(self)
    }
}


// Double recursive macro :)
macro_rules! impl_cast_range_to_integer
{
    ($src: ty, $dest: ty) =>
    {
        impl CastRangeFrom<$src> for $dest
        {
            #[inline(always)]
            fn cast_range_from(value: $src) -> $dest
            {
                // The match can be inlined by the compiler since it is matching on compile time constant
                match (<$src>::PRIMITIVE_NUMBER_TYPE, <$dest>::PRIMITIVE_NUMBER_TYPE)
                {
                    (NumberType::IntegerSigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>() { return value as $dest; }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            return (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    },
                    (NumberType::IntegerSigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<$src>() > std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }else
                        {
                            // up cast or same size
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    },
                    (NumberType::IntegerSigned, NumberType::Float) => ((value as $dest - <$src>::RANGE_MIN as $dest) / (<$src>::RANGE as $dest)) ,
                    (NumberType::IntegerSigned, NumberType::Bool) => if (value > <$src>::ZERO) { <$dest>::RANGE_MAX } else { <$dest>::RANGE_MIN },
                    (NumberType::IntegerUnsigned, NumberType::IntegerSigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>()
                        {
                            // same size, but different range
                            return (value / (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    },
                    (NumberType::IntegerUnsigned, NumberType::IntegerUnsigned) =>
                    {
                        if std::mem::size_of::<$src>() == std::mem::size_of::<$dest>() { return value as $dest; }
                        if std::mem::size_of::<$src>() >= std::mem::size_of::<$dest>()
                        {
                            // down cast
                            (value * (<$src>::RANGE / (<$dest>::RANGE as $src))) as $dest
                        }else
                        {
                            // up cast
                            ((value as $dest) * (<$dest>::RANGE / (<$src>::RANGE as $dest)))
                        }
                    },
                    (NumberType::IntegerUnsigned, NumberType::Float) => ((value as $dest - <$src>::RANGE_MIN as $dest) / (<$src>::RANGE as $dest)),
                    (NumberType::IntegerUnsigned, NumberType::Bool) => if (value > <$src>::ZERO) { <$dest>::RANGE_MAX } else { <$dest>::RANGE_MIN },
                    (NumberType::Float, NumberType::IntegerSigned) => (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest,
                    (NumberType::Float, NumberType::IntegerUnsigned) => (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest,
                    (NumberType::Float, NumberType::Float) => (value * (<$dest>::RANGE as $src) + (<$dest>::RANGE_MIN as $src)) as $dest,
                    (NumberType::Float, NumberType::Bool) => if (value > <$src>::ZERO) { <$dest>::RANGE_MAX } else { <$dest>::RANGE_MIN },
                    (NumberType::Bool, NumberType::IntegerSigned) => if value == <$src>::MIN { <$dest>::RANGE_MIN } else { <$dest>::RANGE_MAX },
                    (NumberType::Bool, NumberType::IntegerUnsigned) => if value == <$src>::MIN { <$dest>::RANGE_MIN } else { <$dest>::RANGE_MAX },
                    (NumberType::Bool, NumberType::Float) => if value == <$src>::MIN { <$dest>::RANGE_MIN } else { <$dest>::RANGE_MAX },
                    (NumberType::Bool, NumberType::Bool) => if (value > <$src>::ZERO) { <$dest>::RANGE_MAX } else { <$dest>::RANGE_MIN },
                }
            }
        }
    };

    ($dest: ty) =>
    {
        map_on_number!(impl_cast_range_to_integer,$dest);
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




/// fX
pub trait CastRangeIntoFloat            : CastRangeInto<f32> + CastRangeInto<f64> {}
impl<T> CastRangeIntoFloat for T where T: CastRangeInto<f32> + CastRangeInto<f64> {}

/// fX
pub trait CastRangeFromFloat            : CastRangeFrom<f32> + CastRangeFrom<f64> {}
impl<T> CastRangeFromFloat for T where T: CastRangeFrom<f32> + CastRangeFrom<f64> {}

/// fX
pub trait CastRangeFloat            : CastRangeIntoFloat + CastRangeFromFloat {}
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
pub trait CastRangeIntegerUnsigned            : CastRangeFromIntegerUnsigned + CastRangeFromIntegerUnsigned {}
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
pub trait CastRangeIntegerSigned            : CastRangeFromIntegerSigned + CastRangeFromIntegerSigned {}
impl<T> CastRangeIntegerSigned for T where T: CastRangeFromIntegerSigned + CastRangeFromIntegerSigned {}


/// iX uX
pub trait CastRangeIntoInteger            : CastRangeIntoIntegerSigned + CastRangeIntoIntegerUnsigned {}
impl<T> CastRangeIntoInteger for T where T: CastRangeIntoIntegerSigned + CastRangeIntoIntegerUnsigned {}

/// iX uX
pub trait CastRangeFromInteger            : CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned {}
impl<T> CastRangeFromInteger for T where T: CastRangeFromIntegerSigned + CastRangeFromIntegerUnsigned {}

/// iX uX
pub trait CastRangeInteger            : CastRangeIntoInteger + CastRangeFromInteger {}
impl<T> CastRangeInteger for T where T: CastRangeIntoInteger + CastRangeFromInteger {}


/// bool
pub trait CastRangeIntoBool            : CastRangeInto<bool> {}
impl<T> CastRangeIntoBool for T where T: CastRangeInto<bool> {}

/// bool
pub trait CastRangeFromBool            : CastRangeFrom<bool> {}
impl<T> CastRangeFromBool for T where T: CastRangeFrom<bool> {}

/// bool
pub trait CastRangeBool            : CastRangeIntoBool + CastRangeFromBool {}
impl<T> CastRangeBool for T where T: CastRangeIntoBool + CastRangeFromBool {}


/// iX uX fX
pub trait CastRangeIntoNumber            : CastRangeIntoInteger + CastRangeIntoFloat {}
impl<T> CastRangeIntoNumber for T where T: CastRangeIntoInteger + CastRangeIntoFloat {}

/// iX uX fX
pub trait CastRangeFromNumber            : CastRangeFromInteger + CastRangeFromFloat {}
impl<T> CastRangeFromNumber for T where T: CastRangeFromInteger + CastRangeFromFloat {}

/// iX uX fX
pub trait CastRangeNumber            : CastRangeInteger + CastRangeFloat {}
impl<T> CastRangeNumber for T where T: CastRangeInteger + CastRangeFloat {}


/// iX uX fX bool
pub trait CastRangeIntoPrimitive            : CastRangeIntoNumber + CastRangeIntoBool {}
impl<T> CastRangeIntoPrimitive for T where T: CastRangeIntoNumber + CastRangeIntoBool {}

/// iX uX fX bool
pub trait CastRangeFromPrimitive            : CastRangeFromNumber + CastRangeFromBool {}
impl<T> CastRangeFromPrimitive for T where T: CastRangeFromNumber + CastRangeFromBool {}

/// iX uX fX bool
pub trait CastRangePrimitive            : CastRangeIntoPrimitive + CastRangeFromPrimitive {}
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