use super::*;

// TODO: move this module in math


pub trait Abs
{
    type Output;
    fn abs(self) -> Self::Output;
}

map_on_integer_unsigned!(
    ($primitive_name: ty) =>
    {
        impl Abs for $primitive_name
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self::Output { self }
        }
    }
);

macro_rules! impl_abs {
    ($primitive_name: ty) =>
    {
        impl Abs for $primitive_name
        {
            type Output = Self;
            #[inline(always)]
            fn abs(self) -> Self::Output { self.abs() }
        }
    };
}
map_on_integer_signed!(impl_abs);
map_on_float!(impl_abs);

impl<T> Abs for Wrapping<T> where T: Abs
{
    type Output=Wrapping<T::Output>;
    fn abs(self) -> Self::Output { Wrapping(self.0.abs()) }
}
impl<T> Abs for Saturating<T> where T: Abs
{
    type Output=Saturating<T::Output>;
    fn abs(self) -> Self::Output { Saturating(self.0.abs()) }
}

impl<T,T2, const N : usize> Abs for [T;N] where T: Abs<Output = T2>
{
    type Output=[T2;N];
    fn abs(self) -> Self::Output { self.map(|v| v.abs()) }
}