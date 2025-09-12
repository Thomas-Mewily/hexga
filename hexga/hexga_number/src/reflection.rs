use super::*;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OverflowPolicy { None, Wrapping, Saturating }

impl OverflowPolicy
{
    #[inline(always)] pub const fn is_none(self) -> bool { matches!(self, Self::None) }
    #[inline(always)] pub const fn is_wrapping(self) -> bool { matches!(self, Self::Wrapping) }
    #[inline(always)] pub const fn is_saturating(self) -> bool { matches!(self, Self::Saturating) }
}

pub trait OverflowBehavior
{
    const OVERFLOW_BEHAVIOR : OverflowPolicy = OverflowPolicy::None;
}

map_on_integer!(
    ($primitive_name: ty) =>
    {
        impl OverflowBehavior for $primitive_name {}
    };
);

impl<T> OverflowBehavior for Wrapping<T>   { const OVERFLOW_BEHAVIOR : OverflowPolicy = OverflowPolicy::Wrapping;   }
impl<T> OverflowBehavior for Saturating<T> { const OVERFLOW_BEHAVIOR : OverflowPolicy = OverflowPolicy::Saturating; }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NumberType
{
    IntegerSigned,
    IntegerUnsigned,
    Float,
    Bool,
}
impl NumberType
{
    pub const fn is_integer_signed(self) -> bool { matches!(self, Self::IntegerSigned) }
    pub const fn is_integer_unsigned(self) -> bool { matches!(self, Self::IntegerUnsigned) }
    pub const fn is_float(self) -> bool { matches!(self, Self::Float) }
    pub const fn is_bool(self) -> bool { matches!(self, Self::Bool) }
    pub const fn is_integer(self) -> bool { self.is_integer_signed() || self.is_integer_unsigned() }
}

pub trait PrimitiveType
{
    const PRIMITIVE_NUMBER_TYPE : NumberType;
}
map_on_integer_unsigned!(
    ($typename:ident) =>
    {
        impl PrimitiveType for $typename
        {
            const PRIMITIVE_NUMBER_TYPE : NumberType = NumberType::IntegerUnsigned;
        }
    }
);
map_on_integer_signed!(
    ($typename:ident) =>
    {
        impl PrimitiveType for $typename
        {
            const PRIMITIVE_NUMBER_TYPE : NumberType = NumberType::IntegerSigned;
        }
    }
);
map_on_float!(
    ($typename:ident) =>
    {
        impl PrimitiveType for $typename
        {
            const PRIMITIVE_NUMBER_TYPE : NumberType = NumberType::Float;
        }
    }
);
impl PrimitiveType for bool
{
    const PRIMITIVE_NUMBER_TYPE : NumberType = NumberType::Bool;
}