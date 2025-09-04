pub use log::{info, warn, error, debug};

pub mod prelude
{
    pub use super::{info,warn,error,debug};
    pub use crate::dbg_here;
}