use crate::*;

pub trait Integer             : NumberInteger + Abs + Primitive {}
impl<T> Integer for T where T : NumberInteger + Abs + Primitive {}

pub trait IntegerUnsigned             : NumberIntegerUnsigned + Primitive {}
impl<T> IntegerUnsigned for T where T : IntegerUnsigned + Primitive {}

pub trait IntegerSigned             : NumberIntegerSigned + Primitive {}
impl<T> IntegerSigned for T where T : IntegerSigned + Primitive {}

/// iX uX fX bool
pub trait Primitive : RangeDefault + CastPrimitive + PrimitiveType {}
impl<T> Primitive for T where T : RangeDefault + CastPrimitive + PrimitiveType {}

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
impl_composite_output_with_methods!(ToUnsigned, to_unsigned);


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
impl_composite_output_with_methods!(ToSigned, to_signed);
