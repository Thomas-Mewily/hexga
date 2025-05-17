#![allow(unused_imports)]

#![feature(get_disjoint_mut_helpers)]

use std::ops::*;
use std::iter::{Sum,Product};
use std::fmt::{Debug, Display, Formatter};
type DResult =  std::fmt::Result;
use std::hash::*;
use std::marker::PhantomData;
use std::cmp::Ordering;
use hexga_core::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub use hexga_typedef as typedef;
pub use hexga_typedef::*;

pub mod number;
use number::*;

pub mod range;
use range::*;

pub mod extension;
use extension::*;

pub use hexga_array::*;
use hexga_array::*;

pub mod prelude;
use prelude::*;

mod geometry;
pub use geometry::*;

mod other;
pub use other::*;

mod graphics;
pub use graphics::*;