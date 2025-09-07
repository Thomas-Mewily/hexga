use super::*;

pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;

pub mod prelude
{
    pub use super::rgba::prelude::*;
    pub use super::hsla::prelude::*;
    pub use super::icolor::prelude::*;
}