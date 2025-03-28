use std::{rc::Rc, sync::Arc};

use crate::*;

// Similar to Trait From / Into, but those trait suppose no loss when converting, so it is impossible to cast a f32 to a i64 for exemple

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
pub trait CastTo<T>
{
    type Output;
    /// Might lose some precision.
    /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
    fn cast_to(self) -> Self::Output;
}

pub trait CastToPrimitive<T> : CastTo<T,Output = T> {}
impl<T,T2> CastToPrimitive<T> for T2 where T2 : CastTo<T,Output = T> {}

impl_composite_output_with_methods!(CastTo <CastToOut>, cast_to);

/// Wrap the coef inside for a new type.
/// Used to differenciate the type Coef and float because they are the same
pub struct CoefWrapper(pub Coef);
impl<T> CastTo<CoefWrapper> for T where T : CastToPrimitive<float> + DefaultRange + UnitArithmetic
{
    type Output=Coef;

    fn cast_to(self) -> Self::Output {
        (self - Self::MIN_RANGE).to_float() / Self::RANGE.to_float() + Self::MIN_RANGE.to_float()
    }
}



// Double recursive macro :)
macro_rules! impl_cast_to 
{ 
    ($itself: ty, $cast_into: ty) => 
    { 
        impl CastTo<$cast_into> for $itself
        {
            type Output = $cast_into;
            fn cast_to(self) -> Self::Output { self as _ }
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
        impl CastTo<bool> for $itself
        {
            type Output=bool;
            fn cast_to(self) -> Self::Output { self == (0 as $itself) }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastTo<bool> for bool { type Output = bool; fn cast_to(self) -> Self::Output { self } }


pub trait CastFloat : CastToPrimitive<f32> + CastToPrimitive<f64> {}
impl<T> CastFloat for T where T : CastToPrimitive<f32> + CastToPrimitive<f64> {}

pub trait CastIntegerUnsigned : 
    CastToPrimitive<u8 > + 
    CastToPrimitive<u16> +
    CastToPrimitive<u32> +
    CastToPrimitive<u64> +
    CastToPrimitive<usize>
{}
impl<T> CastIntegerUnsigned for T where T :
    CastToPrimitive<u8 > + 
    CastToPrimitive<u16> +
    CastToPrimitive<u32> +
    CastToPrimitive<u64> +
    CastToPrimitive<usize>
{}

pub trait CastIntegerSigned : 
    CastToPrimitive<i8 > + 
    CastToPrimitive<i16> +
    CastToPrimitive<i32> +
    CastToPrimitive<i64> +
    CastToPrimitive<isize>
{}
impl<T> CastIntegerSigned for T where T :
    CastToPrimitive<i8 > + 
    CastToPrimitive<i16> +
    CastToPrimitive<i32> +
    CastToPrimitive<i64> +
    CastToPrimitive<isize>
{}

pub trait CastInteger : CastIntegerSigned + CastIntegerUnsigned {}
impl<T> CastInteger for T where T : CastIntegerSigned + CastIntegerUnsigned {}

pub trait CastBool : CastToPrimitive<bool> {}
impl<T> CastBool for T where T : CastToPrimitive<bool> {}

pub trait CastNumber : CastInteger + CastFloat {}
impl<T> CastNumber for T where T : CastInteger + CastFloat {}

pub trait CastPrimitive : CastNumber + CastBool + Copy {}
impl<T> CastPrimitive for T where T : CastNumber + CastBool + Copy {}