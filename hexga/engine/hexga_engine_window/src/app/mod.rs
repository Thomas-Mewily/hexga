use std::collections::HashMap;

use crate::*;

pub mod prelude { pub use super::modules::IAppContext; }

pub use modules::*;

#[doc(hidden)]
pub mod modules;
