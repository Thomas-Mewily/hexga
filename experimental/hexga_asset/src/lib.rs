#![allow(unused_imports)]
use hexga_core::prelude::*;
use hexga_encoding::prelude::*;
use hexga_io::prelude::*;
use hexga_utils::dirty::prelude::*;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor, ser::SerializeStruct};

mod reload;
pub use reload::*;

mod file;
pub use file::*;

mod load_save;
pub use load_save::*;

mod asset;
pub use asset::*;

mod asset_data;
pub use asset_data::*;

pub mod prelude
{
    pub use super::FileData;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{
        FsLoad, FsProvider, FsSave,
        Persistant, Reload, 
    };
}
