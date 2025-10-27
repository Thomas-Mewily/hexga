use super::*;

use std::{any::{Any, TypeId}, io::BufWriter, ops::{Deref, DerefMut}};

use hexga::prelude::*;

use ron::ser::PrettyConfig;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


mod identifier;
pub(crate) use identifier::*;

mod serializer;
pub use serializer::*;


mod markup;
pub use markup::*;