use crate::*;



use super::*;

pub mod prelude
{
    pub use super::{CastInto,CastFrom};
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
/// assert_eq!(i32::cast_from(255u8), 255);
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// 
/// assert_eq!(255u8.cast_into(), 255i32);
/// assert_eq!(12.3f32.cast_into(), 12i32);
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
impl<C1,C2> CastFrom<C2> for C1 where C1: CompositeGeneric, C2: CompositeGeneric<WithType<C1::Inside> = Self>, C1::Inside : CastFrom<C2::Inside>
{
    fn cast_from(value : C2) -> Self 
    {
        value.map(|v| C1::Inside::cast_from(v))
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
/// assert_eq!(i32::cast_from(255u8), 255);
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// 
/// assert_eq!(255u8.cast_into(), 255i32);
/// assert_eq!(12.3f32.cast_into(), 12i32);
/// ```
///
/// Also work with composite like [std::array], [Vector]...
/// 
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


macro_rules! impl_cast_to_bool
{
    ($itself: ty) =>
    {
        impl CastFrom<bool> for $itself
        {
            fn cast_from(value: bool) -> $itself  { if value { 1 as $itself } else { 0 as $itself } }
        }

        impl CastFrom<$itself> for bool
        {
            fn cast_from(value: $itself) -> bool { value != (0 as $itself) }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastFrom<bool> for bool { fn cast_from(value : bool) -> Self { value } }




/// fX
pub trait CastIntoFloat            : CastInto<f32> + CastInto<f64> {}
impl<T> CastIntoFloat for T where T: CastInto<f32> + CastInto<f64> {}

/// fX
pub trait CastFromFloat            : CastFrom<f32> + CastFrom<f64> {}
impl<T> CastFromFloat for T where T: CastFrom<f32> + CastFrom<f64> {}

/// fX
pub trait CastFloat            : CastIntoFloat + CastFromFloat {}
impl<T> CastFloat for T where T: CastIntoFloat + CastFromFloat {}


/// uX
pub trait CastIntoIntegerUnsigned :
    CastInto<u8 > +
    CastInto<u16> +
    CastInto<u32> +
    CastInto<u64> +
    CastInto<usize>
{}
impl<T> CastIntoIntegerUnsigned for T where T:
    CastInto<u8 > +
    CastInto<u16> +
    CastInto<u32> +
    CastInto<u64> +
    CastInto<usize>
{}

/// uX
pub trait CastFromIntegerUnsigned :
    CastFrom<u8 > +
    CastFrom<u16> +
    CastFrom<u32> +
    CastFrom<u64> +
    CastFrom<usize>
{}
impl<T> CastFromIntegerUnsigned for T where T:
    CastFrom<u8 > +
    CastFrom<u16> +
    CastFrom<u32> +
    CastFrom<u64> +
    CastFrom<usize>
{}


/// uX
pub trait CastIntegerUnsigned            : CastFromIntegerUnsigned + CastFromIntegerUnsigned {}
impl<T> CastIntegerUnsigned for T where T: CastFromIntegerUnsigned + CastFromIntegerUnsigned {}


/// iX
pub trait CastIntoIntegerSigned :
    CastInto<i8 > +
    CastInto<i16> +
    CastInto<i32> +
    CastInto<i64> +
    CastInto<isize>
{}
impl<T> CastIntoIntegerSigned for T where T:
    CastInto<i8 > +
    CastInto<i16> +
    CastInto<i32> +
    CastInto<i64> +
    CastInto<isize>
{}

/// iX
pub trait CastFromIntegerSigned :
    CastFrom<i8 > +
    CastFrom<i16> +
    CastFrom<i32> +
    CastFrom<i64> +
    CastFrom<isize>
{}
impl<T> CastFromIntegerSigned for T where T:
    CastFrom<i8 > +
    CastFrom<i16> +
    CastFrom<i32> +
    CastFrom<i64> +
    CastFrom<isize>
{}

/// iX
pub trait CastIntegerSigned            : CastFromIntegerSigned + CastFromIntegerSigned {}
impl<T> CastIntegerSigned for T where T: CastFromIntegerSigned + CastFromIntegerSigned {}

/// iX uX
pub trait CastIntoInteger            : CastIntoIntegerSigned + CastIntoIntegerUnsigned {}
impl<T> CastIntoInteger for T where T: CastIntoIntegerSigned + CastIntoIntegerUnsigned {}

/// iX uX
pub trait CastFromInteger            : CastFromIntegerSigned + CastFromIntegerUnsigned {}
impl<T> CastFromInteger for T where T: CastFromIntegerSigned + CastFromIntegerUnsigned {}

/// iX uX
pub trait CastInteger            : CastIntoInteger + CastFromInteger {}
impl<T> CastInteger for T where T: CastIntoInteger + CastFromInteger {}


/// bool
pub trait CastIntoBool            : CastInto<bool> {}
impl<T> CastIntoBool for T where T: CastInto<bool> {}

/// bool
pub trait CastFromBool             : CastFrom<bool> {}
impl<T> CastFromBool for T where T: CastFrom<bool> {}

/// bool
pub trait CastBool            : CastIntoBool + CastFromBool {}
impl<T> CastBool for T where T: CastIntoBool + CastFromBool {}


/// iX uX fX
pub trait CastIntoNumber            : CastIntoInteger + CastIntoFloat {}
impl<T> CastIntoNumber for T where T: CastIntoInteger + CastIntoFloat {}

/// iX uX fX
pub trait CastFromNumber            : CastFromInteger + CastFromFloat {}
impl<T> CastFromNumber for T where T: CastFromInteger + CastFromFloat {}

/// iX uX fX
pub trait CastNumber            : CastInteger + CastFloat {}
impl<T> CastNumber for T where T: CastInteger + CastFloat {}


/// iX uX fX bool
pub trait CastIntoPrimitive            : CastIntoNumber + CastIntoBool {}
impl<T> CastIntoPrimitive for T where T: CastIntoNumber + CastIntoBool {}

/// iX uX fX bool
pub trait CastFromPrimitive            : CastFromNumber + CastFromBool {}
impl<T> CastFromPrimitive for T where T: CastFromNumber + CastFromBool {}

/// iX uX fX bool
pub trait CastPrimitive            : CastIntoPrimitive + CastFromPrimitive {}
impl<T> CastPrimitive for T where T: CastIntoPrimitive + CastFromPrimitive {}
