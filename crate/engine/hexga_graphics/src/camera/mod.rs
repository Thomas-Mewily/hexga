use super::*;

mod camera;
pub use camera::*;

pub mod prelude
{
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{ICamera};
}