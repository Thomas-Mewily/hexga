pub mod rgba;
pub use rgba::*;

pub mod hsla;
pub use hsla::*;

mod icolor;
pub use icolor::*;

pub mod prelude
{
    pub use super::rgba::{Color,ColorU8,ColorRgba,ColorRgbaOf,rgb,rgba};
    pub use super::hsla::{ColorHsla,ColorHslaOf};
    pub use super::icolor::{ToColorComposite,ToColor,IColor};
}