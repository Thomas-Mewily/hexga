//! Provide GenVec
//!
//! A crate mainly inspired by RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West : <https://youtu.be/aKLntZcp27M>
//! and the SlotMap data structure <https://docs.rs/slotmap/latest/slotmap/>
#![allow(unused_imports)]
#![feature(get_disjoint_mut_helpers)]
use std::{collections::HashMap, fmt::Debug, hash::{Hash, Hasher}, iter::FusedIterator, marker::PhantomData, ops::{Index, IndexMut}};

use hexga_number::*;
#[allow(unused_imports)]
use hexga_core::prelude::*;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub mod gen_vec;
pub mod gen_id;
pub mod gen_multi_map;
pub mod table;

pub(crate) use prelude::*;
pub mod prelude
{
    pub use super::{
        gen_id::prelude::*,
        gen_vec::prelude::*,
        gen_multi_map::prelude::*,
        table::prelude::*,
    };
}
