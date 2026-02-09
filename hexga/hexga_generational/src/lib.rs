//! Provide GenVec
//!
//! A crate mainly inspired by RustConf 2018 - Closing Keynote - Using Rust For Game Development by Catherine West : <https://youtu.be/aKLntZcp27M>
//! and the SlotMap data structure <https://docs.rs/slotmap/latest/slotmap/>
#![allow(unused_imports)]

#![feature(get_disjoint_mut_helpers)]
use std::{
    borrow::Borrow,
    hash::{BuildHasher, RandomState},
};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[allow(unused_imports)]
use hexga_core::prelude::*;
use hexga_number::*;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

pub mod gen_id;
pub mod gen_seq;
pub mod gen_map;
pub mod gen_set;

/*
// Todo: update it
// Not generic over the container type
pub mod multi_map;
pub mod table;
*/

pub(crate) use prelude::*;
pub mod prelude
{
    pub use super::{
        gen_id::prelude::*,
        gen_seq::prelude::*,
        gen_map::prelude::*,
        gen_set::prelude::*,

        //multi_map::prelude::*,
        //table::prelude::*,
    };
}
