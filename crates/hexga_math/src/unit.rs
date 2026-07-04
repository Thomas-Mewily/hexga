use super::*;

#[doc(hidden)]
#[macro_export]
macro_rules! impl_new_unit_or_number {
    ($name:ident) => {
        impl<P> ::core::iter::Sum for $name<P>
        where
            P: ::core::ops::Add<P, Output = P> + $crate::Zero,
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self { iter.fold(Self::ZERO, <Self as ::core::ops::Add<Self>>::add) }
        }

        impl<P, P2> $crate::CastFrom<$name<P2>> for $name<P>
        where
            P: $crate::CastFrom<P2>,
        {
            fn cast_from(value: $name<P2>) -> Self { $name(P::cast_from(value.0)) }
        }
    };
}
pub(crate) use impl_new_unit_or_number;

#[macro_export]
macro_rules! new_unit
{
    ($(#[$attr:meta])* $name:ident) => {

        $(#[$attr])*
        #[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name<P>(pub(crate) P);

        impl<P> $crate::Unit for $name<P> where P: $crate::Number + $crate::PrimitiveType + $crate::OverflowBehavior
        {
            type Precision = P;
            #[inline]
            fn inner_value(self) -> P { Self::inner_value(self) }
            #[inline]
            fn from_inner_value(inner_value: P) -> Self { Self::from_inner_value(inner_value) }
        }

        impl<P> $name<P>
        {
            /// Return the inner value.
            /// This expose how the inner value is stored, but it's impl details and it may change.
            #[doc(hidden)]
            #[inline]
            fn inner_value(self) -> P { self.0 }

            /// Create from the inner value.
            /// This expose how the inner value is stored, but it's impl details and it may change.
            #[doc(hidden)]
            #[inline]
            const fn from_inner_value(inner_value: P) -> Self { Self(inner_value) }
        }

        $crate::map_on_operator_binary_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<P> ::core::ops::$trait_name<Self> for $name<P> where P: ::core::ops::$trait_name<P,Output=P>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        $crate::map_on_operator_assign_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<P> ::core::ops::$trait_name<Self> for $name<P> where P: ::core::ops::$trait_name<P>
                {
                    fn $fn_name(&mut self, rhs : Self) { self.0.$fn_name(rhs.0); }
                }
            }
        );

        impl<P> ::core::ops::Mul<P> for $name<P> where P: ::core::ops::Mul<P,Output=P>
        {
            type Output = Self;
            fn mul(self, rhs : P) -> Self::Output { Self(self.0.mul(rhs)) }
        }
        impl<P> ::core::ops::MulAssign<P> for $name<P> where P: ::core::ops::MulAssign<P>
        {
            fn mul_assign(&mut self, rhs : P) { self.0.mul_assign(rhs); }
        }

        impl<P> ::core::ops::Div<P> for $name<P> where P: ::core::ops::Div<P,Output=P>
        {
            type Output = Self;
            fn div(self, rhs : P) -> Self::Output { Self(self.0.div(rhs)) }
        }
        impl<P> ::core::ops::DivAssign<P> for $name<P> where P: ::core::ops::DivAssign<P>
        {
            fn div_assign(&mut self, rhs : P) { self.0.div_assign(rhs); }
        }

        impl<P> ::core::ops::Rem<Self> for $name<P> where P: ::core::ops::Rem<P,Output=P>
        {
            type Output = Self;
            fn rem(self, rhs : Self) -> Self::Output { Self(self.0.rem(rhs.0)) }
        }
        impl<P> ::core::ops::RemAssign<Self> for $name<P> where P: ::core::ops::RemAssign<P>
        {
            fn rem_assign(&mut self, rhs : Self) { self.0.rem_assign(rhs.0); }
        }

        $crate::map_on_constant_unit!
        (
            (($trait_name: tt, $constant_name: tt)) =>
            {
                impl<P> $trait_name for $name<P> where P: $trait_name { const $constant_name : Self = Self(P::$constant_name); }
            }
        );

        $crate::map_on_operator_unary_arithmetic_unit!
        (
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<P> $trait_name for $name<P> where P: $trait_name<Output = P>
                {
                    type Output = $name<P>;
                    fn $fn_name(self) -> Self { Self(self.0.$fn_name()) }
                }
            }
        );

        $crate::impl_new_unit_or_number!($name);
    };
}
pub(crate) use new_unit;

#[allow(unused_macros)]
macro_rules! new_number
{
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name<T>(pub T);

        map_on_operator_binary!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> ::core::ops::$trait_name<Self> for $name<T> where T: ::core::ops::$trait_name<T,Output=T>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        impl<T: Min> Min for $name<T> {
            fn min_elementwise(self, other: Self) -> Self {
                $name(self.0.min_elementwise(other.0))
            }
        }

        impl<T: Max> Max for $name<T> {
            fn max_elementwise(self, other: Self) -> Self {
                $name(self.0.max_elementwise(other.0))
            }
        }

        map_on_operator_assign!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> ::core::ops::$trait_name<Self> for $name<T> where T: ::core::ops::$trait_name<T>
                {
                    fn $fn_name(&mut self, rhs : Self) { self.0.$fn_name(rhs.0); }
                }
            }
        );

        map_on_constant!
        (
            (($trait_name: tt, $constant_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T: $trait_name { const $constant_name : Self = Self(T::$constant_name); }
            }
        );

        impl<T> Product for $name<T> where T: ::core::ops::Mul<T,Output=T> + One
        {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, Self::mul)
            }
        }

        map_on_operator_unary!
        (
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T: $trait_name<Output = T>
                {
                    type Output = $name<T>;
                    fn $fn_name(self) -> Self { Self(self.0.$fn_name()) }
                }
            }
        );

        impl_new_unit_or_number!($name);
    };
}
pub(crate) use new_number;



