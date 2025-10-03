//!
//! ### Useful type like
//!
//! - [ColorRgba] with any precision (also handle the conversion between different primitive precision)
//! - [ColorHsla] of various precision
//! ### More advanced type
//!
//! If you need more control about the precision, each type have another more generic base type :
//!
//! - [ColorRgba] and [ColorHsla] also use a [float] precision that can be changed using [ColorRgbaOf] and [ColorRgbaOf]

#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Deref, DerefMut, Index, IndexMut, Range}};


use hexga_core::prelude::*;
use hexga_math::grid::*;
use hexga_math::prelude::*;

#[allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, IoError, IoErrorKind, Save, Load};

use std::ops::*;

pub mod image;
use image::*;
pub mod color;

use prelude::*;
pub mod prelude
{
    pub use super::color::prelude::*;
    pub use super::image::prelude::*;
}