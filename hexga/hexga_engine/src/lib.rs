use hexga::prelude::*;
use miniquad::EventHandler;

use std::{fmt::Debug, ops::*};


mod event;
pub use event::*;

mod conf;
pub use conf::*;

mod pen;
pub use pen::*;

mod context;
pub use context::*;