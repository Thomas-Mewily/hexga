#![allow(unused)]
#![allow(dead_code)]

use hexga_core::prelude::*;
use hexga_math::prelude::*;
use hexga_utils::prelude::*;
use hexga_bitflags::bit_index;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};
use std::collections::HashSet;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize};

pub mod event_loop;
pub mod log;
pub mod window;
pub mod input;
mod free_fn;

use prelude::*;


pub mod prelude
{
    pub use super::{
        event_loop::prelude::*,
        log::prelude::*,
        window::prelude::*,
        input::prelude::*,
    };
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::
    {
        event_loop::traits::*,
    };
}