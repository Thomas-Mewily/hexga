//! Provide GenVec
//! 
//! A crate mainly inspired by RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West : https://youtu.be/aKLntZcp27M
//! and the SlotMap data structure https://docs.rs/slotmap/latest/slotmap/

pub(crate) use hexga_number::*;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub mod gen_vec;
pub mod prelude;
