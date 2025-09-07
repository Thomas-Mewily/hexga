use hexga::math::angle::AngleOf;

// Based on https://docs.rs/macroquad/latest/macroquad/camera/index.html
use super::*;

pub trait ICamera
{
    fn matrix(&self) -> Mat4;
    fn have_depth(&self) -> bool;
    fn viewport(&self) -> Option<Rect2P>;
}

pub type Camera3D = Camera3DOf<float>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera3DOf<F> where F:Float
{
    pub position: Vector3<F>,
    pub target: Vector3<F>,
    pub up: Vector3<F>,
    pub perspective: CameraPerspectiveOf<F>

    //pub projection: Projection,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CameraPerspectiveOf<F> where F:Float
{
    pub aspect: F,
    pub fovy: AngleOf<F>,
    pub znear: F,
    pub zfar: F,
}
impl<F> From<CameraPerspectiveOf<F>> for Matrix4<F> where F:Float
{
    // Based on https://docs.rs/cgmath/latest/src/cgmath/projection.rs.html#108
    fn from(p: CameraPerspectiveOf<F>) -> Self 
    {
        assert!(p.fovy > AngleOf::ZERO && p.fovy < AngleOf::HALF);
        assert!(p.aspect != F::ZERO);
        assert!(p.znear > F::ZERO && p.zfar > F::ZERO && p.zfar != p.znear);

        let two = F::TWO;
        let f = (p.fovy / two).cot();

        let m00 = f / p.aspect;
        let m11 = f;
        let m22 = (p.zfar + p.znear) / (p.znear - p.zfar);
        let m23 = -F::ONE;
        let m32 = (two * p.zfar * p.znear) / (p.znear - p.zfar);

        Matrix4::from_col
        (
    Vector4::new
            (
            Vector4::new(m00, F::ZERO, F::ZERO, F::ZERO),
            Vector4::new(F::ZERO, m11, F::ZERO, F::ZERO),
            Vector4::new(F::ZERO, F::ZERO, m22, m32),
            Vector4::new(F::ZERO, F::ZERO, m23, F::ZERO)
            )
        )
    }
}


/* 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection 
{
    Perspective,
    Orthographics,
}*/

impl<F> Camera3DOf<F> where F:Float
{
    fn matrix(&self) -> Matrix4<F>
    {
        let view: Matrix<F, 4, 4> = Matrix4::<F>::look_to_rh(self.position, self.target, self.up);
        let proj = Matrix4::<F>::from(self.perspective);
        return Self::OPENGL_TO_WGPU_MATRIX * view * proj;
    }

    pub(crate) const OPENGL_TO_WGPU_MATRIX : Matrix4<F> = Matrix4::from_col
    (
        vector4
        (
            vector4(F::ONE, F::ZERO, F::ZERO, F::ZERO),
            vector4(F::ZERO, F::ONE, F::ZERO, F::ZERO),
            vector4(F::ZERO, F::ZERO, F::HALF, F::ZERO),
            vector4(F::ZERO, F::ZERO, F::HALF, F::ONE)
        )
    );
}

impl<F> ICamera for Camera3DOf<F> where F:Float
{
    fn matrix(&self) -> Mat4 { self.matrix().to_float() }

    fn have_depth(&self) -> bool {
        todo!()AA
    }

    fn viewport(&self) -> Option<Rect2P> {
        todo!()
    }
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