#![allow(unused_imports)]
#![allow(unused_macros)]

use std::ops::*;
use std::fmt::{Debug, Display, Formatter, Result as DResult};
use std::marker::PhantomData;

use hexga_number::*;
pub use hexga_core::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub mod prelude;

pub mod non_empty_stack;
