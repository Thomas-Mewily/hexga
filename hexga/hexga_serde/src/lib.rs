use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use hexga_encoding::prelude::*;
use serde::{Deserialize, Serialize};

mod io;
pub use io::*;

mod markup;
pub use markup::*;

pub mod de;
pub mod ser;

pub mod prelude
{
    pub use super::{io::*,markup::*};
}