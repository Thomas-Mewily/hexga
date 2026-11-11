use super::*;

mod camera;
pub use camera::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::camera::{Camera, Camera3D};
}

pub mod traits
{
    pub use super::camera::{GetCamera, SetCamera};
}
