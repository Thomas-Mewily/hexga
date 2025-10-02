use super::*;



/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) 
/// keyword: `4f32 as u64`, and the [From] trait, but generic friendly.
/// 
/// One should always prefer implementing [CastFrom] over [CastInto] because implementing [CastFrom] automatically provides one with an implementation of [CastInto] thanks to the blanket implementation in the hexga_math library.
///
/// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// 
/// let casted : i32 = 12.3f32.cast_into();
/// assert_eq!(casted, 12i32);
/// ```
///
/// Also work with composite type
/// ```rust
/// use hexga_math::prelude::*;
///
/// let x = [1, 2i32];
/// let y : [f32; 2] = x.cast_into(),
/// assert_eq!(y, [1f32, 2f32]);
///  
/// 
/// let a = point2(1, 2);
/// let b : Vec2 = a.cast_into(),
/// assert_eq!(b, vec2(1., 2.));
/// ```
pub trait CastFrom<T> 
{ 
    fn cast_from(value : T) -> Self; 
}
impl<C1,C2> CastFrom<C2> for C1 where C1: MapGeneric, C2: MapGeneric<WithType<C1::Item> = Self>, C1::Item : CastFrom<C2::Item>
{
    fn cast_from(value : C2) -> Self 
    {
        value.map(|v| C1::Item::cast_from(v))
    }
}


/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) 
/// keyword: `4f32 as u64`, and the [From] trait, but generic friendly.
/// 
/// One should always prefer implementing [CastFrom] over [CastInto] because implementing [CastFrom] automatically provides one with an implementation of [CastInto] thanks to the blanket implementation in the hexga_math library.
///
/// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// 
/// let casted : i32 = 12.3f32.cast_into();
/// assert_eq!(casted, 12i32);
/// ```
///
/// Also work with composite type
/// ```rust
/// use hexga_math::prelude::*;
///
/// let x = [1, 2i32];
/// let y : [f32; 2] = x.cast_into(),
/// assert_eq!(y, [1f32, 2f32]);
///  
/// 
/// let a = point2(1, 2);
/// let b : Vec2 = a.cast_into(),
/// assert_eq!(b, vec2(1., 2.));
/// ```
pub trait CastInto<T> : Sized 
{ 
    fn cast_into(self) -> T; 
}
impl<S,T> CastInto<T> for S where T:CastFrom<S>
{
    fn cast_into(self) -> T {
        T::cast_from(self)
    }
}


// Double recursive macro :)
macro_rules! impl_cast_to
{
    ($src: ty, $dest: ty) =>
    {
        impl CastFrom<$src> for $dest
        {
            fn cast_from(value: $src) -> $dest { value as $dest }
        }
    };

    ($cast_into: ty) =>
    {
        map_on_number!(impl_cast_to,$cast_into);
    };
}
map_on_number!(impl_cast_to);


map_on_integer!(
    ($itself: ty) =>
    {
        impl CastFrom<bool> for $itself
        {
            fn cast_from(value: bool) -> $itself  { if value { 1 } else { 0 } }
        }

        impl CastFrom<$itself> for bool
        {
            fn cast_from(value: $itself) -> bool { value != (0) }
        }
    };
);
map_on_float!(
    ($itself: ty) =>
    {
        impl CastFrom<bool> for $itself
        {
            fn cast_from(value: bool) -> $itself  { if value { 1. } else { 0. } }
        }

        impl CastFrom<$itself> for bool
        {
            fn cast_from(value: $itself) -> bool { value as $itself >= 0.5 }
        }
    };
);
impl CastFrom<bool> for bool { fn cast_from(value : bool) -> Self { value } }



trait_marker!(
    /// fX
    CastIntoFloat: CastInto<f32> + ToF32<Output = f32> + CastInto<f64> + ToF64<Output = f64>
);

trait_marker!(
    /// fX
    CastFromFloat: CastFrom<f32> + CastFrom<f64>
);

trait_marker!(
    /// fX
    CastFloat: CastIntoFloat + CastFromFloat
);

trait_marker!(
/// uX
CastIntoIntegerUnsigned:
    CastInto<u8 > + ToU8<Output = u8> +
    CastInto<u16> + ToU16<Output = u16> +
    CastInto<u32> + ToU32<Output = u32> +
    CastInto<u64> + ToU64<Output = u64> +
    CastInto<usize> + ToUSize<Output = usize> +
);

trait_marker!(
/// uX
CastFromIntegerUnsigned:
    CastFrom<u8 > +
    CastFrom<u16> +
    CastFrom<u32> +
    CastFrom<u64> +
    CastFrom<usize>
);


trait_marker!(
/// uX
CastIntegerUnsigned: CastFromIntegerUnsigned + CastFromIntegerUnsigned
);

trait_marker!(
/// iX
CastIntoIntegerSigned:
    CastInto<i8 > + ToI8<Output = i8> +
    CastInto<i16> + ToI16<Output = i16> +
    CastInto<i32> + ToI32<Output = i32> +
    CastInto<i64> + ToI64<Output = i64> +
    CastInto<isize> + ToISize<Output = isize>
);


trait_marker!(
/// iX
CastFromIntegerSigned:
    CastFrom<i8 > +
    CastFrom<i16> +
    CastFrom<i32> +
    CastFrom<i64> +
    CastFrom<isize>
);


trait_marker!(
/// iX
CastIntegerSigned: CastFromIntegerSigned + CastFromIntegerUnsigned
);

trait_marker!(
/// iX uX
CastIntoInteger: CastIntoIntegerSigned + CastIntoIntegerUnsigned
);

trait_marker!(
/// iX uX
CastFromInteger: CastFromIntegerSigned + CastFromIntegerUnsigned
);

trait_marker!(
/// iX uX
CastInteger: CastIntoInteger + CastFromInteger
);


trait_marker!(
/// bool
CastIntoBool: CastInto<bool> + ToBool<Output = bool>
);

trait_marker!(
/// bool
CastFromBool: CastFrom<bool>
);

trait_marker!(
/// bool
CastBool: CastIntoBool + CastFromBool
);

trait_marker!(
/// iX uX fX
CastIntoNumber: CastIntoInteger + CastIntoFloat
);

trait_marker!(
/// iX uX fX
CastFromNumber: CastFromInteger + CastFromFloat
);

trait_marker!(
/// iX uX fX
CastNumber: CastInteger + CastFloat
);

trait_marker!(
/// iX uX fX bool
CastIntoPrimitive: CastIntoNumber + CastIntoBool
);

trait_marker!(
/// iX uX fX bool
CastFromPrimitive: CastFromNumber + CastFromBool
);

trait_marker!(
/// iX uX fX bool
CastPrimitive: CastIntoPrimitive + CastFromPrimitive
);