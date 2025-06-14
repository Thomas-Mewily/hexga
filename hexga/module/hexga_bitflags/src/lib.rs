//! A crate that handle bit flag logic with `BitFlags<Enum,Rep>`

mod bitmask;
pub use bitmask::*;

mod bitflags;
mod bitflags2;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub mod prelude;