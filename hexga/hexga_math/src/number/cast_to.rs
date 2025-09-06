use crate::*;

// Similar to Trait From / Into, but those trait suppose no loss when converting, so it is impossible to cast a f32 to a i64 for exemple

/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html)
/// keyword: `4f32 as u64`, but generic friendly.
///
/// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
///
/// The [CastIntoComposite] trait is the most generic way to use it.
///
/// The Output type can be a little bit different for composite type, but it should still be related to the generic type.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(i32::cast_from(255u8), 255i32);
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// ```
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let float32 = 2.5f32;
/// let float64 = 2.5f64;
/// let float32_to_64 = f64::cast_from(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// let float32_to_64 = <f32 as CastInto<f64>>::cast_into(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// // The most generic way to do it
/// let float32_to_64 = <f32 as CastIntoComposite<f64>>::cast_into_composite(float32);
/// assert_eq!(float32_to_64, float32_to_64);
/// ```
///
/// Only the [CastIntoComposite] will be working with composite :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let vec_f32 = Vector2::<f32>::new(0.5, 0.5);
/// let vec_f64 = Vector2::<f64>::new(0.5, 0.5);
/// let vec_f32_to_f64 = <Vector2::<f32> as CastIntoComposite<f64>>::cast_into_composite(vec_f32);
/// assert_eq!(vec_f32_to_f64, vec_f64);
/// ```
///
/// There is no `CastFromComposite` trait because it is impossible to impl it.
pub trait CastIntoComposite<T>
{
    type Output;
    /// Might lose some precision.
    /// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html) keyword: `4f32 as u64`, but can be used in a generic way.
    fn cast_into_composite(self) -> Self::Output;
}


/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html)
/// keyword: `4f32 as u64`, but generic friendly.
///
/// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
///
/// The [CastIntoComposite] trait is the most generic way to use it.
///
/// The Output type can be a little bit different for composite type, but it should still be related to the generic type.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(i32::cast_from(255u8), 255i32);
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// ```
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let float32 = 2.5f32;
/// let float64 = 2.5f64;
/// let float32_to_64 = f64::cast_from(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// let float32_to_64 = <f32 as CastInto<f64>>::cast_into(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// // The most generic way to do it
/// let float32_to_64 = <f32 as CastIntoComposite<f64>>::cast_into_composite(float32);
/// assert_eq!(float32_to_64, float32_to_64);
/// ```
///
/// Only the [CastIntoComposite] will be working with composite :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let vec_f32 = Vector2::<f32>::new(0.5, 0.5);
/// let vec_f64 = Vector2::<f64>::new(0.5, 0.5);
/// let vec_f32_to_f64 = <Vector2::<f32> as CastIntoComposite<f64>>::cast_into_composite(vec_f32);
/// assert_eq!(vec_f32_to_f64, vec_f64);
/// ```
///
/// There is no `CastFromComposite` trait because it is impossible to impl it.
pub trait CastInto<T> : CastIntoComposite<T,Output = T> + Sized { fn cast_into(self) -> Self::Output { self.cast_into_composite() } }
impl<T,T2> CastInto<T> for T2 where T2 : CastIntoComposite<T,Output = T> {}

/// Same semantics as the [as](https://practice.course.rs/type-conversions/as.html)
/// keyword: `4f32 as u64`, but generic friendly.
///
/// Like the [as](https://practice.course.rs/type-conversions/as.html) keyword, the result might lose some precision.
///
/// The [CastIntoComposite] trait is the most generic way to use it.
///
/// The Output type can be a little bit different for composite type, but it should still be related to the generic type.
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// assert_eq!(i32::cast_from(255u8), 255i32);
/// assert_eq!(i32::cast_from(12.3f32), 12);
/// ```
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let float32 = 2.5f32;
/// let float64 = 2.5f64;
/// let float32_to_64 = f64::cast_from(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// let float32_to_64 = <f32 as CastInto<f64>>::cast_into(float32);
/// assert_eq!(float32_to_64, float32_to_64);
///
/// // The most generic way to do it
/// let float32_to_64 = <f32 as CastIntoComposite<f64>>::cast_into_composite(float32);
/// assert_eq!(float32_to_64, float32_to_64);
/// ```
///
/// Only the [CastIntoComposite] will be working with composite :
///
/// ```rust
/// use hexga_math::prelude::*;
///
/// let vec_f32 = Vector2::<f32>::new(0.5, 0.5);
/// let vec_f64 = Vector2::<f64>::new(0.5, 0.5);
/// let vec_f32_to_f64 = <Vector2::<f32> as CastIntoComposite<f64>>::cast_into_composite(vec_f32);
/// assert_eq!(vec_f32_to_f64, vec_f64);
/// ```
///
/// There is no `CastFromComposite` trait because it is impossible to impl it.
pub trait CastFrom<T> { fn cast_from(value : T) -> Self; }
impl<Src,Dest> CastFrom<Dest> for Src where Dest : CastInto<Src> { fn cast_from(value : Dest) -> Self { value.cast_into_composite() } }

/*
pub trait CastFromComposite<T> { fn cast_from_composite(value : T) -> Self; }
// the type parameter `T` is not constrained by the impl trait, self type, or predicates. unconstrained type parameter
impl<Dest,T> CastFromComposite<Dest> for Dest::Output where Dest : CastIntoComposite<T> { fn cast_from_composite(value : Dest) -> Self { value.cast_into() } }
*/

//impl_composite_output_with_methods!(CastIntoComposite<CastToOut>, cast_into_composite);

/*
new_number!(
    /// Wrap the coef inside for a new type.
    /// Used to differenciate the type Coef and float because they are the same for CastIntoComposite impl
    CoefWrapperOf
);

pub type CoefWrapper = CoefWrapperOf<float>;

impl<T> CastIntoComposite<CoefWrapper> for T where T: CastInto<float> + RangeDefault + UnitArithmetic
{
    type Output=Coef;

    fn cast_into_composite(self) -> Self::Output {
        (self - Self::RANGE_MIN).to_float() / Self::RANGE.to_float() + Self::RANGE_MIN.to_float()
    }
}
*/


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

        impl CastIntoComposite<$itself> for bool
        {
            type Output=$itself;
            fn cast_into_composite(self) -> Self::Output { if self { <$itself>::ONE } else { <$itself>::ZERO } }
        }
    };
}
map_on_number!(impl_cast_to_bool);
impl CastIntoComposite<bool> for bool { type Output = bool; fn cast_into_composite(self) -> Self::Output { self } }

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
