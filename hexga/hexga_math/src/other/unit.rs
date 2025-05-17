use super::*;

macro_rules! new_unit
{
    ($name : ident) => {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
        struct $name<T>(T);

        map_on_operator_binary_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) => 
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T : std::ops::$trait_name<T,Output=T>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        map_on_operator_assign_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) => 
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T : std::ops::$trait_name<T>
                {
                    fn $fn_name(&mut self, rhs : Self) { self.0.$fn_name(rhs.0); }
                }
            }
        );

        impl<T> std::ops::Div<Self> for $name<T> where T : std::ops::Div<T,Output=T>
        {
            type Output = T;
            fn div(self, rhs : Self) -> Self::Output { self.0.div(rhs.0) }
        }

        impl<T> std::ops::Rem<Self> for $name<T> where T : std::ops::Rem<T,Output=T>
        {
            type Output = Self;
            fn rem(self, rhs : Self) -> Self::Output { Self(self.0.rem(rhs.0)) }
        }
        impl<T> std::ops::RemAssign<Self> for $name<T> where T : std::ops::RemAssign<T>
        {
            fn rem_assign(&mut self, rhs : Self) { self.0.rem_assign(rhs.0); }
        }

        map_on_constant!
        (
            (($trait_name: tt, $constant_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T : $trait_name { const $constant_name : Self = Self(T::$constant_name); }
            }
        );

        impl<T> Sum for $name<T> where T : std::ops::Add<T,Output=T> + Zero
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        map_on_operator_unary_arithmetic_unit!
        (
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T : $trait_name<Output = T>
                {
                    type Output = $name<T>;
                    fn $fn_name(self) -> Self { Self(self.0.$fn_name()) }
                }
            }
        );
    };
}
pub(crate) use new_unit;


macro_rules! new_number
{
    ($name : ident) => {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
        struct $name<T>(T);

        map_on_operator_binary!(
            (($trait_name: tt, $fn_name: tt)) => 
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T : std::ops::$trait_name<T,Output=T>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        map_on_operator_assign!(
            (($trait_name: tt, $fn_name: tt)) => 
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T : std::ops::$trait_name<T>
                {
                    fn $fn_name(&mut self, rhs : Self) { self.0.$fn_name(rhs.0); }
                }
            }
        );

        map_on_constant!
        (
            (($trait_name: tt, $constant_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T : $trait_name { const $constant_name : Self = Self(T::$constant_name); }
            }
        );

        impl<T> Sum for $name<T> where T : std::ops::Add<T,Output=T> + Zero
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        map_on_operator_unary!
        (
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T : $trait_name<Output = T>
                {
                    type Output = $name<T>;
                    fn $fn_name(self) -> Self { Self(self.0.$fn_name()) }
                }
            }
        );
    };
}
pub(crate) use new_number;