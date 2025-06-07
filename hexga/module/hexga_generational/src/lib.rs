//! Provide GenVec
//!
//! A crate mainly inspired by RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West : <https://youtu.be/aKLntZcp27M>
//! and the SlotMap data structure <https://docs.rs/slotmap/latest/slotmap/>

#![feature(get_disjoint_mut_helpers)]

use hexga_number::prelude::*;
#[allow(unused_imports)]
use hexga_core::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

use gen_vec::*;
pub mod gen_vec;

/*
use gen_hash_map::*;
pub mod gen_hash_map;
*/

pub mod prelude;
