#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub use hexga::*;
pub mod event_loop;
pub mod log;
pub mod window;
pub mod input;
pub mod clipboard;
mod free_fn;

use prelude::*;


pub mod prelude
{
    pub use super::{
        event_loop::prelude::*,
        log::prelude::*,
        window::prelude::*,
        input::prelude::*,
        clipboard::prelude::*,
    };
    pub use hexga::prelude::*;
}
