#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(mapped_lock_guards)]
// #![feature(clone_to_uninit)]

/*
TODO: redo it ?

trait for:
hot_reload()
save()

+1 better path/id separation:

1 path => 1 ID
1 ID => N paths

asset correct path/remove extension

special type of asset that are load/unloaded automatically when not used for X second

*/

// use std::clone::CloneToUninit;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    default,
    hash::Hash,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::{
        Arc, LazyLock, MappedRwLockReadGuard, MappedRwLockWriteGuard, PoisonError, RwLock,
        RwLockReadGuard, RwLockWriteGuard, Weak,
    },
};

use hexga_core::prelude::*;
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;

mod asset;
pub use asset::*;

mod asset_manager;
pub use asset_manager::*;

mod bytes;
pub use bytes::*;

mod hot_reload;
pub use hot_reload::*;

#[cfg(feature = "serde")]
pub use serde::{
    Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct,
};

pub mod prelude
{
    pub use super::Asset;
}
