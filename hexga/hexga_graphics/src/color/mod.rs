pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;

pub mod prelude
{
    pub use super::rgba::{Color,ColorU8,rgb,rgba};
    pub use super::hsla::{ColorHsla};
    pub use super::icolor::{ToColorComposite,ToColor,IColor};
}