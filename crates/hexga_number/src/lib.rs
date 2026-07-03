#![allow(unused_imports)]

use hexga_map_on as map_on;
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::num::{Saturating, Wrapping};
use std::fmt::{Debug,Display,Formatter,self};
use std::ops::*;
use std::marker::PhantomData;
use std::hash::Hash;
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

/*
pub mod prelude
{
    pub use super::typedef::*;
    
    pub use super::arithmetic::*;
    pub use super::map::*;
    pub use super::constant::*;
    pub use super::convert::*;
    pub use super::range::prelude::*;
    pub use super::reflection::prelude::*;
}
use prelude::*;

*/
/*
pub mod traits
{

}
*/
