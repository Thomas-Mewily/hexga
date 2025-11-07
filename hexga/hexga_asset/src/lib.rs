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

mod assets;
pub use assets::*;

mod untyped_assets;
pub use untyped_assets::*;

mod asset;
pub use asset::*;

mod asset_manager;
pub use asset_manager::*;

pub mod prelude
{
    pub use super::{Asset,Assets,AssetsUntyped};
}
