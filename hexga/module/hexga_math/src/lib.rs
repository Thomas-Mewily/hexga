//! # A Math library that contains
//!
//! ### N Dimension stuff
//!
//! This crate define N dimensionnal math stuff (2d, 3d, 4d, ... nd) like vector/point of any type (float, int, uint, or even user defined) :
//!
//! - [Vector] (fixed size array wrapper)
//! - [Rectangle]
//! - [Grid]
//! - [Matrix]
//!
//!
//! ### Useful type like
//!
//! - [Angle]
//! - [Time],
//! - [ColorRGBA] with any precision (also handle the conversion between different primitive precision)
//! - [ColorHSLA] of various precision
//!
//! ### Generic Casting trait
//! The crate also provide generic traits for casting with the same behavior as the [as keyword](https://practice.course.rs/type-conversions/as.html) :
//! - [CastInto], [CastFrom] and [CastIntoComposite],
//!
//! ### Generic Remapping trait
//!
//! Similar traits for casting remapping the range of an primitive to another primitive range also exist :
//! - [CastRangeInto], [CastRangeFrom] and [CastRangeIntoComposite]
//!
//! ### Quick start with the prelude
//!
//! There are some quick typedef in the prelude :
//!
//! - [int], [uint] and [float]  : The default primitive precision used in the typedef. (can be change with the feature flags)
//! - [Point2], [Point3], [Point4] for Vector of [int],
//! - [Vec2], [Vec3], [Vec4] for Vector of [float],
//! - [Rect2], [Rect3], [Rect4] for Rectangle of [float],
//! - [Rect2P], [Rect3P], [Rect4P] for Rectangle of [int] (`P` for point),
//! - [Mat2], [Mat3], [Mat4] for Matrix of [float], and [Mat2P], [Mat3P], [Mat4P] use [int],
//! - [Grid2], [Grid3], [Grid3]  can only be indexed by [Point] by default.
//!
//! ### More advanced type
//!
//! If you need more control about the precision, each type have another more generic long base type :
//!
//! - [Grid] type uses a [Point] for the indexing precision, but that can be changed by using with the [GridBase] type.
//! - [Angle] and [Time] use a [float] precision that can be changed using [AngleOf] and [TimeOf]
//! - [ColorRGBA] and [ColorHSLA] also use a [float] precision that can be changed using [ColorRGBAOf] and [ColorRGBAOf]
#![allow(unused_imports)]

#![feature(get_disjoint_mut_helpers)]

use std::ops::*;
use std::iter::{Sum,Product};
use std::fmt::{Debug, Display, Formatter};
type DResult = std::fmt::Result;
use std::hash::*;
use std::marker::PhantomData;
use std::cmp::Ordering;
use hexga_core::prelude::*;
use std::ops::{Range, RangeInclusive};

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io_derive")]
use hexga_io::{Save, Load, IoSave, IoLoad};

pub mod prelude;
use prelude::*;

pub use hexga_typedef as typedef;
pub use hexga_typedef::*;

pub mod number;
use number::prelude::*;

pub mod range;
use range::*;

pub mod extension;
use extension::*;

pub use hexga_array::*;
use hexga_array::*;

mod geometry;
pub use geometry::*;

mod other;
pub use other::*;

mod color;
pub use color::*;

pub mod graph;
use graph::*;