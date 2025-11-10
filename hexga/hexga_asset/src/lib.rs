#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

// #![feature(clone_to_uninit)]

// use std::clone::CloneToUninit;
use std::{any::{Any, TypeId}, collections::HashMap, hash::Hash, iter::FusedIterator, marker::PhantomData, ops::{Deref, DerefMut}, pin::Pin, ptr::NonNull, sync::Arc};

use hexga_core::prelude::*;
use hexga_generational::{gen_vec, multi_map::{self, EntryID}, prelude::*, table};
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_singleton::prelude::*;


#[cfg(feature = "serde")]
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
pub use serde_impl::*;

mod untyped_assets;
pub use untyped_assets::*;

mod asset;
pub use asset::*;

mod asset_manager;
pub use asset_manager::*;

mod hot_reload;
pub use hot_reload::*;

mod asset_save;
pub use asset_save::*;

pub mod prelude
{
    pub use super::{Asset,Assets,HotReload,AssetSave};
}
