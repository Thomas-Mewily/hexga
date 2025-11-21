#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#![feature(mapped_lock_guards)]
// #![feature(clone_to_uninit)]


// use std::clone::CloneToUninit;
use std::{
    any::{Any, TypeId}, collections::HashMap, default, hash::Hash, iter::FusedIterator, marker::PhantomData, ops::{Deref, DerefMut}, path::{Path, PathBuf}, sync::{Arc, LazyLock, MappedRwLockReadGuard, MappedRwLockWriteGuard, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak}
};

use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::{Singleton, prelude::*};

mod asset;
pub use asset::*;

mod asset_manager;
pub use asset_manager::*;

#[cfg(feature = "serde")]
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub mod prelude
{
    pub use super::{Asset};
}