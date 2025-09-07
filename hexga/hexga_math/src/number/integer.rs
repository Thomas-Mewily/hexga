use crate::*;

pub trait Integer            : NumberInteger + Abs + Primitive + CastPrimitive {}
impl<T> Integer for T where T: NumberInteger + Abs + Primitive + CastPrimitive {}

pub trait IntegerUnsigned            : Integer + NumberIntegerUnsigned {}
impl<T> IntegerUnsigned for T where T: Integer + NumberIntegerUnsigned {}

pub trait IntegerSigned            : Integer + NumberIntegerSigned {}
impl<T> IntegerSigned for T where T: Integer + NumberIntegerSigned {}

/// iX uX fX bool
pub trait Primitive : RangeDefault + CastPrimitive + PrimitiveType + CastRangePrimitive + Default + PartialEq + PartialOrd + Copy + Debug {}
impl<T> Primitive for T where T: RangeDefault + CastPrimitive + PrimitiveType + CastRangePrimitive + Default + PartialEq + PartialOrd + Copy + Debug {}

/// For type that have an unsigned equivalent
pub trait ToUnsigned
{
    /// The unsigned equivalence
    type Output; // : ToSigned;
    fn to_unsigned(self) -> Self::Output;
}
macro_rules! impl_signed
{
    ($primitive_name: ty, $unsigned_primitive_name: ty) =>
    {
        impl ToUnsigned for $primitive_name  { type Output=$unsigned_primitive_name; #[inline(always)] fn to_unsigned(self) -> Self::Output  {self as _ }}
    };

    ($primitive_name: ty) =>
    {
        impl ToUnsigned for $primitive_name  { type Output=$primitive_name; #[inline(always)] fn to_unsigned(self) -> Self::Output  { self }}
    };
}
impl_signed!(i8 , u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(isize, usize);
map_on_integer_unsigned!(impl_signed);
map_on_float!(impl_signed);
impl<T> ToUnsigned for T where T:CompositeGeneric, T::Inside: ToUnsigned
{
    type Output=T::WithType<<T::Inside as ToUnsigned>::Output>;
    fn to_unsigned(self) -> Self::Output { self.map(|v| v.to_unsigned()) }
}


/// For type that have a signed equivalent
pub trait ToSigned
{
    /// The signed equivalence
    type Output; // : ToUnsigned;
    fn to_signed(self) -> Self::Output;
}
macro_rules! impl_unsigned
{
    ($primitive_name: ty, $signed_primitive_name: ty) =>
    {
        impl ToSigned for $primitive_name { type Output=$signed_primitive_name; #[inline(always)] fn to_signed(self) -> Self::Output { self as _  }}
    };

    ($primitive_name: ty) =>
    {
        impl ToSigned for $primitive_name  { type Output=$primitive_name; #[inline(always)] fn to_signed(self) -> Self::Output  { self }}
    };
}
impl_unsigned!(u8 , i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(usize, isize);
map_on_integer_signed!(impl_unsigned);
map_on_float!(impl_unsigned);
impl<T> ToSigned for T where T:CompositeGeneric, T::Inside: ToSigned
{
    type Output=T::WithType<<T::Inside as ToSigned>::Output>;
    fn to_signed(self) -> Self::Output { self.map(|v| v.to_signed()) }
}