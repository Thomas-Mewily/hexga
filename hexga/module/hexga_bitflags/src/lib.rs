//! A crate that handle bit flag logic with `BitFlags<Enum,Rep>`

mod bit_flags;
pub use bit_flags::*;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub mod prelude;