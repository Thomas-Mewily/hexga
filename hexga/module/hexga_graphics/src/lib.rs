#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::Write, ops::{Deref, DerefMut, Index, IndexMut, Range}};

use hexga_core::prelude::*;
use hexga_math::prelude::*;
use hexga_math::grid::*;

#[allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, IoError, IoErrorKind, Save, Load};

pub mod image;
use image::*;

pub mod prelude
{
    pub use super::image::{Image, ToImage};
}