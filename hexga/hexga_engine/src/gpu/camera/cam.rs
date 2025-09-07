use super::*;

pub mod prelude
{
    pub use super::Cam;
}

ctx_singleton!(
    Cam,
    CameraManager,
    { Pen::try_as_ref().map(|pen| pen.camera()) },
    { Pen::try_as_mut().map(|pen| pen.camera_mut()) }
);