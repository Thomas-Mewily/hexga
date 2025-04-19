use crate::*;

// Similar to Trait From / Into, but those trait suppose no loss when converting, so it is impossible to cast a f32 to a i64 for exemple

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
/// 
/// The Output type can be a little bit different, but still related to the generic type :
/// 
/// ```rust
/// use hexga_math::prelude::*;
/// use std::any::{TypeId};
/// 
/// assert_eq!(TypeId::of::<<[u8;1] as CastIntoComposite<u16>>::Output>(), TypeId::of::<[u16;1]>())
/// ```
pub trait CastIntoComposite<T>
{
    type Output;
    /// Might lose some precision.
    /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
    fn cast_into_composite(self) -> Self::Output;
}

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
pub trait CastInto<T> : CastIntoComposite<T,Output = T> {}
impl<T,T2> CastInto<T> for T2 where T2 : CastIntoComposite<T,Output = T> {}

/// Might lose some precision.
/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`
pub trait CastFrom<T> { fn cast_from(value : T) -> Self; }
impl<Src,Dest> CastFrom<Dest> for Src where Dest : CastInto<Src> { fn cast_from(value : Dest) -> Self { value.cast_into_composite() } }

/* 
pub trait CastFromComposite<T> { fn cast_from(value : T) -> Self; }
// the type parameter `T` is not constrained by the impl trait, self type, or predicates. unconstrained type parameter
impl<Dest,T> CastFromComposite<Dest> for Dest::Output where Dest : CastIntoComposite<T> { fn cast_from(value : Dest) -> Self { value.cast_into() } }
*/
impl_composite_output_with_methods!(CastIntoComposite<CastToOut>, cast_into_composite);

/// Wrap the coef inside for a new type.
/// Used to differenciate the type Coef and float because they are the same
pub struct CoefWrapper(pub Coef);
impl<T> CastIntoComposite<CoefWrapper> for T where T : CastInto<float> + DefaultRange + UnitArithmetic
{
    type Output=Coef;

    fn cast_into_composite(self) -> Self::Output {
        (self - Self::MIN_RANGE).to_float() / Self::RANGE.to_float() + Self::MIN_RANGE.to_float()
    }
}



// Double recursive macro :)
macro_rules! impl_cast_to 
{ 
    ($itself: ty, $cast_into: ty) => 
    { 
        impl CastIntoComposite<$cast_into> for $itself
        {
            type Output = $cast_into;
            fn cast_into_composite(self) -> Self::Output { self as _ }
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
        impl CastIntoComposite<bool> for $itself
        {
            type Output=bool;
            fn cast_into_composite(self) -> Self::Output { self == (0 as $itself) }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastIntoComposite<bool> for bool { type Output = bool; fn cast_into_composite(self) -> Self::Output { self } }


pub trait CastFloat : CastInto<f32> + CastInto<f64> {}
impl<T> CastFloat for T where T : CastInto<f32> + CastInto<f64> {}

pub trait CastIntegerUnsigned : 
    CastInto<u8 > + 
    CastInto<u16> +
    CastInto<u32> +
    CastInto<u64> +
    CastInto<usize>
{}
impl<T> CastIntegerUnsigned for T where T :
    CastInto<u8 > + 
    CastInto<u16> +
    CastInto<u32> +
    CastInto<u64> +
    CastInto<usize>
{}

pub trait CastIntegerSigned : 
    CastInto<i8 > + 
    CastInto<i16> +
    CastInto<i32> +
    CastInto<i64> +
    CastInto<isize>
{}
impl<T> CastIntegerSigned for T where T :
    CastInto<i8 > + 
    CastInto<i16> +
    CastInto<i32> +
    CastInto<i64> +
    CastInto<isize>
{}

pub trait CastInteger : CastIntegerSigned + CastIntegerUnsigned {}
impl<T> CastInteger for T where T : CastIntegerSigned + CastIntegerUnsigned {}

pub trait CastBool : CastInto<bool> {}
impl<T> CastBool for T where T : CastInto<bool> {}

pub trait CastNumber : CastInteger + CastFloat {}
impl<T> CastNumber for T where T : CastInteger + CastFloat {}

pub trait CastPrimitive : CastNumber + CastBool + Copy {}
impl<T> CastPrimitive for T where T : CastNumber + CastBool + Copy {}