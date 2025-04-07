#![allow(unused_imports)]

use std::cmp::Ordering;
use std::ops::*;
use std::iter::{Sum,Product};
use std::num::{Saturating, Wrapping};
pub use hexga_map_on::*;

mod constant;
pub use constant::*;

mod increase;
pub use increase::*;

mod number;
pub use number::*;