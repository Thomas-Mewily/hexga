use crate::*;

// Similar to Trait From / Into, but those trait suppose no loss when converting, so it is impossible to cast a f32 to a i64 for exemple

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
pub trait CastToComposite<T>
{
    type Output;
    /// Might lose some precision.
    /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
    fn cast_to(self) -> Self::Output;
}

pub trait CastTo<T> : CastToComposite<T,Output = T> {}
impl<T,T2> CastTo<T> for T2 where T2 : CastToComposite<T,Output = T> {}

impl_composite_output_with_methods!(CastToComposite <CastToOut>, cast_to);

/// Wrap the coef inside for a new type.
/// Used to differenciate the type Coef and float because they are the same
pub struct CoefWrapper(pub Coef);
impl<T> CastToComposite<CoefWrapper> for T where T : CastTo<float> + DefaultRange + UnitArithmetic
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
        impl CastToComposite<$cast_into> for $itself
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
        impl CastToComposite<bool> for $itself
        {
            type Output=bool;
            fn cast_to(self) -> Self::Output { self == (0 as $itself) }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastToComposite<bool> for bool { type Output = bool; fn cast_to(self) -> Self::Output { self } }


pub trait CastFloat : CastTo<f32> + CastTo<f64> {}
impl<T> CastFloat for T where T : CastTo<f32> + CastTo<f64> {}

pub trait CastIntegerUnsigned : 
    CastTo<u8 > + 
    CastTo<u16> +
    CastTo<u32> +
    CastTo<u64> +
    CastTo<usize>
{}
impl<T> CastIntegerUnsigned for T where T :
    CastTo<u8 > + 
    CastTo<u16> +
    CastTo<u32> +
    CastTo<u64> +
    CastTo<usize>
{}

pub trait CastIntegerSigned : 
    CastTo<i8 > + 
    CastTo<i16> +
    CastTo<i32> +
    CastTo<i64> +
    CastTo<isize>
{}
impl<T> CastIntegerSigned for T where T :
    CastTo<i8 > + 
    CastTo<i16> +
    CastTo<i32> +
    CastTo<i64> +
    CastTo<isize>
{}

pub trait CastInteger : CastIntegerSigned + CastIntegerUnsigned {}
impl<T> CastInteger for T where T : CastIntegerSigned + CastIntegerUnsigned {}

pub trait CastBool : CastTo<bool> {}
impl<T> CastBool for T where T : CastTo<bool> {}

pub trait CastNumber : CastInteger + CastFloat {}
impl<T> CastNumber for T where T : CastInteger + CastFloat {}

pub trait CastPrimitive : CastNumber + CastBool + Copy {}
impl<T> CastPrimitive for T where T : CastNumber + CastBool + Copy {}