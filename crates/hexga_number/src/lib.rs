#![allow(unused_imports)]

use hexga_map_on as map_on;
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    iter::{FusedIterator, Product, Sum},
    marker::PhantomData,
    num::{Saturating, Wrapping},
    ops::*,
};

//use hexga_typedef::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

macro_rules! trait_marker {
    (
        $(#[$meta:meta])*
        $name:ident : $($bounds:tt)+
    ) => {
        $(#[$meta])*
        pub trait $name: $($bounds)+ {}
        impl<T> $name for T where T: $($bounds)+ {}
    };
}

// pub use hexga_typedef as typedef;

pub use map_on::*;

mod reflection;
pub use reflection::*;

mod arithmetic;
pub use arithmetic::*;

mod map;
pub use map::*;

mod constant;
pub use constant::*;

mod range;
pub use range::*;

mod convert;
pub use convert::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        clamp, // abs

        half,
        max,
        min,
        minus_one,
        mix,
        one,
        zero,
    };
}

pub mod traits
{
    pub use crate::{
        Abs, CastFrom, CastInto, convert::cast_primitive::*,
        CastRangeFrom, CastRangeInto, Clamp, Constant, Decrement, Floating, Half, Increment, Infinity, IterSample, IterSampleDefault,
        IterSampleDefaultInclusive, IterStep, IterStepDefault, IterStepDefaultInclusive, IterStepMax, Map, MapIntern, MapInternWith, MapWith, Max, MaxValue,
        Min, MinValue, MinusInfinity, MinusOne, MinusOneIter, Mix, NaNValue, OddOrEven, One, OneIter, OverflowBehavior, PartialOrdExtension,
        PositiveOrNegative, Pow, PrimitiveType, RangeDefault, RemEuclid, TakeHalf, Three, Two, Unit, UnwrapZero, Zero, ZeroIter, arithmetic::traits::*,
    };
}
