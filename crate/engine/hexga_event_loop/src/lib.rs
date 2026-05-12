#![allow(unused)]
#![allow(dead_code)]

use hexga_bitflags::bit_index;
use hexga_core::prelude::*;
use hexga_math::prelude::*;
use hexga_utils::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub mod event_loop;
mod free_fn;
pub mod input;
pub mod log;
pub mod monitor;
pub mod window;

use prelude::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        event_loop::prelude::*, input::prelude::*, log::prelude::*, monitor::prelude::*,
        window::prelude::*,
    };
}

pub mod traits
{
    pub use super::event_loop::traits::*;
}
