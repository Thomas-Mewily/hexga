use std::num::NonZero;

use crate::*;

/// Define the `0` representation : The absorbing element of the multiplication such that `x * X::ZERO = X::ZERO`
pub trait Zero : Sized
{
    /// The absorbing element of the multiplication such that `x * X::ZERO = X::ZERO`
    const ZERO : Self;
    #[inline(always)] fn zero() -> Self { Self::ZERO }

    #[inline(always)] fn set_zero(&mut self) -> &mut Self { *self = Self::ZERO; self }

    #[inline(always)] fn is_zero(&self) -> bool where Self : PartialEq<Self> { self == &Self::ZERO }
    #[inline(always)] fn is_non_zero(&self) -> bool where Self : PartialEq<Self> { !self.is_zero() }
}
pub const fn zero<T : Zero>() -> T { T::ZERO }
map_on_number!(($primitive_name: ty) => { impl Zero for $primitive_name { const ZERO : Self = 0 as Self; } });

impl<T> Zero for Wrapping<T> where T : Zero  { const ZERO : Self = Wrapping(T::ZERO); }
impl<T> Zero for Saturating<T> where T : Zero  { const ZERO : Self = Saturating(T::ZERO); }

pub trait UnwrapZero
{
    type Output : Zero;
    fn unwrap_or_zero(self) -> Self::Output;
}
impl<T> UnwrapZero for Option<T> where T : Zero { type Output = T; fn unwrap_or_zero(self) -> Self::Output { self.unwrap_or(T::ZERO) }}
impl<T,E> UnwrapZero for Result<T,E> where T : Zero { type Output = T; fn unwrap_or_zero(self) -> Self::Output { self.unwrap_or(T::ZERO) }}

impl<T, const N : usize> Zero for [T;N] where T : Zero
{
    const ZERO : Self = [T::ZERO; N];
}
impl Zero for bool { const ZERO : Self = false; }



/// Define the `1` representation : The neutral element of the multiplication such that `x * X::ONE = x`
pub trait One : Sized
{
    /// The neutral element of the multiplication such that `x * X::ONE = x`
    const ONE  : Self;
    #[inline(always)] fn one() -> Self where Self : Sized { Self::ONE }

    #[inline(always)] fn set_one(&mut self) -> &mut Self { *self = Self::ONE; self }

    #[inline(always)] fn is_one(&self) -> bool where Self : PartialEq<Self> { self == &Self::ONE }
    #[inline(always)] fn is_non_one(&self) -> bool where Self : PartialEq<Self> { !self.is_one() }
}
pub const fn one<T : One>() -> T { T::ONE }

map_on_number!(($primitive_name: ty) => { impl One for $primitive_name { const ONE : Self = 1 as Self; } });

impl<T> One for Wrapping<T> where T : One  { const ONE : Self = Wrapping(T::ONE); }
impl<T> One for Saturating<T> where T : One  { const ONE : Self = Saturating(T::ONE); }

impl<T, const N : usize> One for [T;N] where T : One
{
    const ONE : Self = [T::ONE; N];
}
impl One for bool { const ONE : Self = true; }


/// Define the `-1` representation
pub trait MinusOne : Sized
{
    const MINUS_ONE  : Self;
    #[inline(always)] fn minus_one() -> Self where Self : Sized { Self::MINUS_ONE }

    #[inline(always)] fn set_minus_one(&mut self) -> &mut Self { *self = Self::MINUS_ONE; self }

    #[inline(always)] fn is_minus_one(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::MINUS_ONE }
    #[inline(always)] fn is_non_minus_one(self) -> bool where Self : PartialEq<Self> + Copy { !self.is_minus_one() }
}
pub const fn minus_one<T : MinusOne>() -> T { T::MINUS_ONE }

map_on_number!(($primitive_name: ty) => { impl MinusOne for $primitive_name { const MINUS_ONE : Self = 1 as Self; } });
impl<T> MinusOne for Wrapping<T> where T : MinusOne  { const MINUS_ONE : Self = Wrapping(T::MINUS_ONE); }
impl<T> MinusOne for Saturating<T> where T : MinusOne  { const MINUS_ONE : Self = Saturating(T::MINUS_ONE); }

impl<T, const N : usize> MinusOne for [T;N] where T : MinusOne
{
    const MINUS_ONE : Self = [T::MINUS_ONE; N];
}



/// Define the `0.5` representation
pub trait Half : Sized
{
    const HALF  : Self;

    #[inline(always)] fn is_half(self) -> bool where Self : PartialEq<Self> { self == Self::HALF }
    #[inline(always)] fn is_non_half(self) -> bool where Self : PartialEq<Self> { !self.is_half() }
}
pub const fn half<T : Half>() -> T { T::HALF }

map_on_float!(($primitive_name: ty) => { impl Half for $primitive_name { const HALF : Self = 0.5; } });
impl<T> Half for Wrapping<T> where T : Half  { const HALF : Self = Wrapping(T::HALF); }
impl<T> Half for Saturating<T> where T : Half  { const HALF : Self = Saturating(T::HALF); }

impl<T, const N : usize> Half for [T;N] where T : Half
{
    const HALF : Self = [T::HALF; N];
}


/// Define the Not a Number (NaN) representation
pub trait NaNValue : Sized
{
    const NAN : Self;
    #[inline(always)] fn is_nan(&self) -> bool where Self : PartialEq { self == &Self::NAN }
    #[inline(always)] fn is_not_nan(&self) -> bool where Self : PartialEq { !self.is_nan() }
}
map_on_float!(($primitive_name: ty) =>  { impl NaNValue for $primitive_name { const NAN : Self = Self::NAN; }};);

impl<T, const N : usize> NaNValue for [T;N] where T : NaNValue
{
    const NAN : Self = [T::NAN; N];
}



pub trait MinValue : Sized
{
    const MIN : Self;
    #[inline(always)] fn is_min_value(&self) -> bool where Self : PartialEq { self == &Self::MIN }
    #[inline(always)] fn is_not_min_value(&self) -> bool where Self : PartialEq { !self.is_min_value() }
}

map_on_number!(($primitive_name: ty) => { impl MinValue for $primitive_name { const MIN : Self = Self::MIN; }});
impl MinValue for bool { const MIN : Self = false; }
impl<T> MinValue for Wrapping<T> where T : MinValue  { const MIN : Self = Wrapping(T::MIN); }
impl<T> MinValue for Saturating<T> where T : MinValue  { const MIN : Self = Saturating(T::MIN); }

impl<T, const N : usize> MinValue for [T;N] where T : MinValue
{
    const MIN : Self = [T::MIN; N];
}

pub trait MaxValue : Sized
{
    const MAX : Self;
    #[inline(always)] fn is_max_value(&self) -> bool where Self : PartialEq { self == &Self::MAX }
    #[inline(always)] fn is_non_max_value(&self) -> bool where Self : PartialEq { !self.is_max_value() }
}
map_on_number!(($primitive_name: ty) => { impl MaxValue for $primitive_name { const MAX : Self = Self::MAX; } });
impl MaxValue for bool { const MAX : Self = true; }
impl<T> MaxValue for Wrapping<T> where T : MaxValue  { const MAX : Self = Wrapping(T::MAX); }
impl<T> MaxValue for Saturating<T> where T : MaxValue  { const MAX : Self = Saturating(T::MAX); }

impl<T, const N : usize> MaxValue for [T;N] where T : MaxValue
{
    const MAX : Self = [T::MAX; N];
}

/*
pub trait Constant             : Zero + One + MinusOne + Half + NaNValue + MinValue + MaxValue {}
impl<T> Constant for T where T : Zero + One + MinusOne + Half + NaNValue + MinValue + MaxValue {}
*/

#[macro_export]
macro_rules! map_on_constant_unit {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (Zero, ZERO),
                (NaNValue, NAN),
                (MinValue, MIN),
                (MaxValue, MAX)
            ),
            $($macro_arms)*
        );
    };
}

#[macro_export]
macro_rules! map_on_constant {
    ($($macro_arms:tt)*) => {
        $crate::map_on!
        (
            (
                (Zero, ZERO),
                (One, ONE),
                (MinusOne, MINUS_ONE),
                (Half, HALF),
                (NaNValue, NAN),
                (MinValue, MIN),
                (MaxValue, MAX)
            ),
            $($macro_arms)*
        );
    };
}