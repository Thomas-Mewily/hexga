//! ## Definitions
//!
//! - [`Rgba`] and [`Hsla`] using [`float`] precision.
//! - [`Image`] for storing and loading images.
//!   (Similar to `hexga_math::grid`, but supports additional formats when saving.)
//!
//! ### Advanced Types
//!
//! For finer control over precision, each color type also provides a more generic base form:
//!
//! - [`Rgba`] and [`Hsla`] are type aliases for [`RgbaOf<float>`] and [`HslaOf<float>`], respectively.
//!
//! Likewise, image types are built on a generic base:
//!
//! - [`Image`] is an alias for [`ImageBase<RgbaU8>`] or [`ImageBaseOf<RgbaU8, int>`].

#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(formatting_options)] // For image, to display aligned value they need to be fomatted in a temporary formatter

use std::{io::Write, ops::{Deref, DerefMut, Index, IndexMut, Range}};


use hexga_core::prelude::*;
use hexga_math::grid::*;
use hexga_math::prelude::*;

#[allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{Save, IoLoad, IoError, IoErrorKind, Save, Load};

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