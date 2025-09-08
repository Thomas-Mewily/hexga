use super::*;

use std::iter::FusedIterator;

pub mod prelude
{
    pub(crate) use super::{impl_new_unit_or_number,new_unit,new_number};
}

macro_rules! impl_new_unit_or_number
{
    ($name:ident) => {

        impl<T> Sum for $name<T> where T: std::ops::Add<T,Output=T> + Zero
        {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Self::add)
            }
        }

        impl<T,T2> CastFrom<$name<T2>> for $name<T> where T: CastFrom<T2>
        {
            fn cast_from(value: $name<T2>) -> Self {
                $name(T::cast_from(value.0))
            }
        }

        impl<T> WrappedType<T> for $name<T>
        {
            unsafe fn inner_value(self) -> T {
                self.0
            }

            unsafe fn from_inner_value(inner_value: T) -> Self {
                Self(inner_value)
            }
        }

        impl<T, I> RangeSampleExtension<I> for Range<$name<T>> where Range<T> : RangeSampleExtension<I,Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<Range<T> as RangeSampleExtension<I>>::Output>;
            type Item = $name<T>;

            fn sample(self, nb_sample: I) -> Self::Output
            {
                WrappedIterator::new(<Range<T> as RangeSampleExtension<I>>::sample(unsafe { self.start.inner_value() }..unsafe { self.end.inner_value() }, nb_sample))
            }
        }
        impl<T, I> RangeSampleExtension<I> for RangeInclusive<$name<T>> where RangeInclusive<T> : RangeSampleExtension<I,Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeInclusive<T> as RangeSampleExtension<I>>::Output>;
            type Item = $name<T>;

            fn sample(self, nb_sample: I) -> Self::Output
            {
                let (start, end) = self.into_inner();
                WrappedIterator::new(<RangeInclusive<T> as RangeSampleExtension<I>>::sample(unsafe { start.inner_value() }..=unsafe { end.inner_value() }, nb_sample))
            }
        }
        impl<T, I> RangeSampleExtension<I> for RangeTo<$name<T>> where RangeTo<T> : RangeSampleExtension<I,Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeTo<T> as RangeSampleExtension<I>>::Output>;
            type Item = $name<T>;

            fn sample(self, nb_sample: I) -> Self::Output
            {
                WrappedIterator::new(<RangeTo<T> as RangeSampleExtension<I>>::sample(..unsafe { self.end.inner_value() }, nb_sample))
            }
        }
        impl<T, I> RangeSampleExtension<I> for RangeToInclusive<$name<T>> where RangeToInclusive<T> : RangeSampleExtension<I,Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeToInclusive<T> as RangeSampleExtension<I>>::Output>;
            type Item = $name<T>;

            fn sample(self, nb_sample: I) -> Self::Output
            {
                WrappedIterator::new(<RangeToInclusive<T> as RangeSampleExtension<I>>::sample(..=unsafe { self.end.inner_value() }, nb_sample))
            }
        }
        impl<T, I> RangeSampleExtension<I> for RangeFrom<$name<T>> where RangeFrom<T> : RangeSampleExtension<I,Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeFrom<T> as RangeSampleExtension<I>>::Output>;
            type Item = $name<T>;

            fn sample(self, nb_sample: I) -> Self::Output
            {
                WrappedIterator::new(<RangeFrom<T> as RangeSampleExtension<I>>::sample(unsafe{self.start.inner_value()}.., nb_sample))
            }
        }


        impl<T> RangeStepExtension for Range<$name<T>> where Range<T> : RangeStepExtension<Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<Range<T> as RangeStepExtension>::Output>;
            type Item = $name<T>;

            fn step(self, step : Self::Item) -> Self::Output
            {
                WrappedIterator::new(<Range<T> as RangeStepExtension>::step(unsafe { self.start.inner_value() }..unsafe { self.end.inner_value() }, unsafe { step.inner_value() }))
            }
        }
        impl<T> RangeStepExtension for RangeInclusive<$name<T>> where RangeInclusive<T> : RangeStepExtension<Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeInclusive<T> as RangeStepExtension>::Output>;
            type Item = $name<T>;

            fn step(self, step : Self::Item) -> Self::Output
            {
                let (start, end) = self.into_inner();
                WrappedIterator::new(<RangeInclusive<T> as RangeStepExtension>::step(unsafe { start.inner_value() }..=unsafe { end.inner_value() }, unsafe { step.inner_value() }))
            }
        }
        impl<T> RangeStepExtension for RangeTo<$name<T>> where RangeTo<T> : RangeStepExtension<Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeTo<T> as RangeStepExtension>::Output>;
            type Item = $name<T>;

            fn step(self, step : Self::Item) -> Self::Output
            {
                WrappedIterator::new(<RangeTo<T> as RangeStepExtension>::step(..unsafe { self.end.inner_value() }, unsafe { step.inner_value() }))
            }
        }
        impl<T> RangeStepExtension for RangeToInclusive<$name<T>> where RangeToInclusive<T> : RangeStepExtension<Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeToInclusive<T> as RangeStepExtension>::Output>;
            type Item = $name<T>;

            fn step(self, step : Self::Item) -> Self::Output
            {
                WrappedIterator::new(<RangeToInclusive<T> as RangeStepExtension>::step(..=unsafe { self.end.inner_value() }, unsafe { step.inner_value() }))
            }
        }
        impl<T> RangeStepExtension for RangeFrom<$name<T>> where RangeFrom<T> : RangeStepExtension<Item=T>
        {
            type Output = WrappedIterator<$name<T>,T,<RangeFrom<T> as RangeStepExtension>::Output>;
            type Item = $name<T>;

            fn step(self, step : Self::Item) -> Self::Output
            {
                WrappedIterator::new(<RangeFrom<T> as RangeStepExtension>::step(unsafe { self.start.inner_value() }.., unsafe { step.inner_value() }))
            }
        }
    }
}
pub(crate) use impl_new_unit_or_number;


macro_rules! new_unit
{
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
        pub struct $name<T>(pub(crate) T);

        impl<T> $name<T>
        {
            /// Return the inner value.
            /// Unsafe because it expose the inner value, but the unit is not specified and may change
            pub unsafe fn inner_value(self) -> T { self.0 }

            /// Create from the inner value.
            /// Unsafe because it expose the inner value, but the unit is not specified and may change
            pub const unsafe fn from_inner_value(inner_value : T) -> Self { Self(inner_value) }
        }

        map_on_operator_binary_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T: std::ops::$trait_name<T,Output=T>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        map_on_operator_assign_arithmetic_unit!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T: std::ops::$trait_name<T>
                {
                    fn $fn_name(&mut self, rhs : Self) { self.0.$fn_name(rhs.0); }
                }
            }
        );

        impl<T> std::ops::Mul<T> for $name<T> where T: std::ops::Mul<T,Output=T>
        {
            type Output = Self;
            fn mul(self, rhs : T) -> Self::Output { Self(self.0.mul(rhs)) }
        }
        impl<T> std::ops::MulAssign<T> for $name<T> where T: std::ops::MulAssign<T>
        {
            fn mul_assign(&mut self, rhs : T) { self.0.mul_assign(rhs); }
        }

        impl<T> std::ops::Div<T> for $name<T> where T: std::ops::Div<T,Output=T>
        {
            type Output = Self;
            fn div(self, rhs : T) -> Self::Output { Self(self.0.div(rhs)) }
        }
        impl<T> std::ops::DivAssign<T> for $name<T> where T: std::ops::DivAssign<T>
        {
            fn div_assign(&mut self, rhs : T) { self.0.div_assign(rhs); }
        }

        impl<T> std::ops::Rem<Self> for $name<T> where T: std::ops::Rem<T,Output=T>
        {
            type Output = Self;
            fn rem(self, rhs : Self) -> Self::Output { Self(self.0.rem(rhs.0)) }
        }
        impl<T> std::ops::RemAssign<Self> for $name<T> where T: std::ops::RemAssign<T>
        {
            fn rem_assign(&mut self, rhs : Self) { self.0.rem_assign(rhs.0); }
        }

        map_on_constant_unit!
        (
            (($trait_name: tt, $constant_name: tt)) =>
            {
                impl<T> $trait_name for $name<T> where T: $trait_name { const $constant_name : Self = Self(T::$constant_name); }
            }
        );

        map_on_operator_unary_arithmetic_unit!
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
                impl<T> std::ops::$trait_name<Self> for $name<T> where T: std::ops::$trait_name<T,Output=T>
                {
                    type Output = Self;
                    fn $fn_name(self, rhs : Self) -> Self::Output { Self(self.0.$fn_name(rhs.0)) }
                }
            }
        );

        map_on_operator_assign!(
            (($trait_name: tt, $fn_name: tt)) =>
            {
                impl<T> std::ops::$trait_name<Self> for $name<T> where T: std::ops::$trait_name<T>
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

        impl<T> Product for $name<T> where T: std::ops::Mul<T,Output=T> + One
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



// To construct basic wrapped type from their inner type
pub trait WrappedType<T> : Sized
{
    /// Return the inner value.
    /// Unsafe because it expose the inner value, but the unit is not specified and may change
    unsafe fn inner_value(self) -> T;

    /// Create from the inner value.
    /// Unsafe because it expose the inner value, but the unit is not specified and may change
    unsafe fn from_inner_value(inner_value : T) -> Self;
}


pub struct WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>
{
    pub it : It,
    phantom : PhantomData<(Wrapped,Precision)>,
}
impl<Wrapped,Precision,It> WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>
{
    pub const fn new(it : It) -> Self {
        Self { it, phantom: PhantomData }
    }
}


impl<Wrapped,Precision,It> Debug for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", &self.it)
    }
}

impl<Wrapped,Precision,It> Copy for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Copy{}
impl<Wrapped,Precision,It> Clone for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Clone
{
    fn clone(&self) -> Self {
        Self { it: self.it.clone(), phantom: PhantomData }
    }
}

impl<Wrapped,Precision,It> PartialEq for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : PartialEq
{
    fn eq(&self, other: &Self) -> bool { PartialEq::eq(&self, &other) }
}
impl<Wrapped,Precision,It> Eq for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Eq {}

impl<Wrapped,Precision,It> PartialOrd for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.it, &other.it)
    }
}
impl<Wrapped,Precision,It> Ord for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.it, &other.it)
    }
}

impl<Wrapped,Precision,It> Hash for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.it.hash(state);
    }
}

impl<Wrapped,Precision,It> Iterator for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : Iterator
{
    type Item= Wrapped;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|v| unsafe { Wrapped::from_inner_value(v) })
    }
}

impl<Wrapped,Precision,It> DoubleEndedIterator for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.it.next_back().map(|v| unsafe { Wrapped::from_inner_value(v) })
    }
}

impl<Wrapped,Precision,It> FusedIterator for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : FusedIterator{}
impl<Wrapped,Precision,It> ExactSizeIterator for WrappedIterator<Wrapped,Precision,It> where It : Iterator<Item = Precision>, Wrapped : WrappedType<Precision>, It : ExactSizeIterator{}