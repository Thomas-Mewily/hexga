//! # A Math library that contains
//!
//! ## N Dimension stuff and Array Programming
//!
//! This crate define N dimensional math stuff (2d, 3d, 4d, ... nd) like vector/point of any type (float, int, uint, or even user defined) :
//!
//! - [`Vector`] (fixed size array wrapper)
//! - [`Rectangle`]
//! - [`Grid`]
//! - [`Matrix`]
//!
//! The *same* common functions such as [`min`], [`max`], [`mix`],
//! [`abs`], and [`clamp`] work for
//! - primitive values(`u32`, `i32`, `f32`, `bool`, ...)
//! - and on composite types (`Array`, `Vector`, `Color`, `Rectangle`, `Matrix`, `Grid`, ...) that implement the [`Map`] / [`MapWith`] / [`MapIntern`] / [`MapInternWith`] traits.
//!
//! Any external type implementing these traits automatically gains support for these common functions.
//!
//! ## Useful type like
//!
//! - [`Angle`]
//! - [`Time`],
//!
//! ## Generic Casting trait
//! The crate also provide generic traits for casting with the same behavior as the [as keyword](https://practice.course.rs/type-conversions/as.html) :
//! - [`CastInto`], [`CastFrom`],
//!
//! ## Generic Remapping trait
//!
//! Similar traits for casting remapping the range of an primitive to another primitive range also exist :
//! - [`CastRangeInto`], [`CastRangeFrom`]
//!
//! ## Quick start with the prelude
//!
//! There are some quick typedef in the prelude :
//!
//! - [`int`], [`uint`] and [`float`]  : The default primitive precision used in the typedef. (can be change with the feature flags)
//! - [`Vec2i`], [`Vec3i`], [`Vec4i`] for Vector of [`int`],
//! - [`Vec2`], [`Vec3`], [`Vec4`] for Vector of [`float`],
//! - [`Rect2`], [`Rect3`], [`Rect4`] for Rectangle of [`float`],
//! - [`Rect2i`], [`Rect3i`], [`Rect4i`] for Rectangle of [`int`] (`P` for point),
//! - [`Mat2`], [`Mat3`], [`Mat4`] for Matrix of [`float`], and [`Mat2i`], [`Mat3i`], [`Mat4i`] use [`int`],
//! - [`Grid2`], [`Grid3`], [`Grid3`]  can only be indexed by `VecXi` by default.
//!
//! ## More advanced type
//!
//! If you need more control about the precision, each type have another more generic base type :
//!
//! - [`Grid`] type uses a `VecXi` for the indexing precision, but that can be changed by using with the [`hexga_math::grid::GridBase`] type.
//! - [`Angle`] and [`Time`] use a [`float`] precision that can be changed using [`AngleOf`] and [`TimeOf`]

#![allow(unused_imports)]
#![feature(formatting_options)] // For grid view, to display aligned value they need to be fomatted in a temporary formatter

use hexga_core::prelude::*;
use std::ops::*;
use std::hash::{Hash, Hasher};
use std::{fmt, num::{Saturating, Wrapping}};
use std::marker::PhantomData;
use std::cmp::Ordering;
use rayon::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};


pub use hexga_typedef as typedef;
mod utils;
pub use utils::*;
pub mod convert;
pub mod map_on;
pub mod number;
pub mod range;
pub mod array;
pub mod unit;
mod geometry;
pub use geometry::*;


use prelude::*;
pub mod prelude
{
    pub use super::typedef::*;
    pub use super::utils::prelude::*;
    pub use super::convert::*;
    pub use super::map_on::*;
    pub use super::number::*;
    pub use super::range::*;
    pub use super::array::*;
    pub use super::unit::*;
    pub use super::geometry::prelude::*;
}

