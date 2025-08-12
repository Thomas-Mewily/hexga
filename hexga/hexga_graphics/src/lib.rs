//!
//! ### Useful type like
//!
//! - [ColorRGBA] with any precision (also handle the conversion between different primitive precision)
//! - [ColorHSLA] of various precision
//! ### More advanced type
//!
//! If you need more control about the precision, each type have another more generic base type :
//!
//! - [ColorRGBA] and [ColorHSLA] also use a [float] precision that can be changed using [ColorRGBAOf] and [ColorRGBAOf]

#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Deref, DerefMut, Index, IndexMut, Range}};

use hexga_core::prelude::*;
use hexga_math::prelude::*;
use hexga_math::grid::*;
use hexga_math::impl_fixed_array_like_with_op;

#[allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, IoError, IoErrorKind, Save, Load};

use std::ops::*;

mod extension;
use extension::*;

pub mod image;
use image::*;

pub mod color;
use color::*;

pub mod prelude
{
    pub use super::image::prelude::*;
    pub use super::color::prelude::*;
}