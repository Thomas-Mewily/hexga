#![allow(unused_imports)]

use hexga_map_on as map_on;
use std::{
    hash::{Hash, Hasher},
    iter::{Product, Sum, FusedIterator},
    num::{Saturating, Wrapping},
    fmt::{Debug,Display,Formatter,self},
    ops::*,
    marker::PhantomData,
    cmp::Ordering,
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
    pub use super::{
        max,min,mix,clamp, // abs
    
        zero, one, minus_one, half
    };
    pub use super::traits::*;
}

pub mod traits
{
    pub use crate::{
        Max,Min,Mix,Clamp,Abs,Pow,RemEuclid, 
        Floating,
        Increment, Decrement,
        arithmetic::traits::*,

        Zero, ZeroIter, UnwrapZero,
        One, OneIter,
        Two, Three, 
        OddOrEven, PositiveOrNegative,
        MinusOne, MinusOneIter,
        Half, TakeHalf,
        NaNValue, MinValue, MaxValue, Infinity, MinusInfinity,
        PartialOrdExtension,

        Constant,

        CastRangeFrom, CastRangeInto,
        CastFrom, CastInto,

        Unit,

        RangeDefault,
        Sample, SampleDefault, SampleDefaultInclusive,
        RangeStepExtension, RangeStepIter, RangeDefaultStepExtension, RangeDefaultStepInclusiveExtension,

        MapIntern, MapInternWith, Map, MapWith
    };
}
