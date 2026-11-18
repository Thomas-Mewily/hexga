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

mod utils;
pub use utils::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        clamp, // abs
        abs,
        
        half,
        max,
        min,
        minus_one,
        //mix,
        lerp,
        remap,
        one,
        zero,
    };
}

pub mod traits
{
    pub use crate::{
        Abs, Pow, Decay,

        convert::cast_primitive::*,
        CastFrom, CastInto, 

        RangeDefault, CastRangeFrom, CastRangeInto, 
        Clamp, Lerp, Remap,
        Constant, 
        Decrement, Increment,
        Float, Trigo, Half, Infinity, 
        IterSample, IterSampleDefault, IterSampleDefaultInclusive, IterStep, IterStepDefault, IterStepDefaultInclusive, IterStepMax, 
        Map, MapInto, MapZip, MapZipInto,
        Max, MaxValue, Min, MinValue, 
        MinusOne, NaNValue, 
        OddOrEven,
        Zero, One, TakeHalf, Three, Two, MinusInfinity,
        ZeroIter, OneIter, MinusOneIter, UnwrapZero,
        PrimitiveType, OverflowBehavior, 
        PartialOrdExtension, PositiveOrNegative, 
        RemEuclid, Unit, arithmetic::traits::*,

        //RefMutMap
    };
}
