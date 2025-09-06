// Based on https://docs.rs/macroquad/latest/macroquad/camera/index.html
use super::*;

pub trait ICamera
{
    fn matrix(&self) -> Mat4;
    fn have_depth(&self) -> bool;
    fn viewport(&self) -> Option<Rect2P>;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera
{
    pub matrix  : Mat4,
    pub depth   : bool,
    pub viewport: Option<Rect2P>,
}
impl Camera
{
    pub const CAMERA_2D : Self = Self{ matrix: Mat4::IDENTITY, depth: false, viewport: None };
    pub const CAMERA_3D : Self = Self{ matrix: Mat4::IDENTITY, depth: true, viewport: None };

    pub fn new() -> Self { Self::___() }

    pub fn with_matrix(self, matrix: Mat4) -> Self { Self { matrix, ..self }}
    pub fn with_depth(self, depth: bool) -> Self { Self { depth, ..self }}
    pub fn with_viewport(self, viewport: Option<Rect2P>) -> Self { Self { viewport, ..self }}
}
impl Default for Camera
{
    fn default() -> Self { Self::CAMERA_3D }
}
impl ICamera for Camera
{
    fn matrix(&self) -> Mat4 { self.matrix }
    fn have_depth(&self) -> bool { self.depth }
    fn viewport(&self) -> Option<Rect2P> { self.viewport }
}

pub mod prelude
{
    pub use super::{Camera};
}