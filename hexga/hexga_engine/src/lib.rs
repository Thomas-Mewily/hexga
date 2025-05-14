#![allow(unused_imports)]
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;
use miniquad::EventHandler;

use std::{fmt::Debug, ops::*};

mod log;
pub use log::*;

mod event;
pub use event::*;

mod conf;
pub use conf::*;

mod gpu;
pub use gpu::*;

mod pen;
pub use pen::*;

mod context;
pub use context::*;