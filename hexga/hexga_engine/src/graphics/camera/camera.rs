use super::*;

// Based on https://docs.rs/macroquad/latest/macroquad/camera/index.html
pub trait ICamera<F=float> : GetMatrix<F,4,4> where F:Float
{
    fn have_depth(&self) -> bool;
    //fn viewport(&self) -> Option<Rect2i>;

    fn camera(&self) -> CameraOf<F> { CameraOf { matrix: self.matrix(), depth: self.have_depth() }}

    /*
    fn push(&self) where Camera : CastFrom<CameraOf<F>>, Self:Sized
    {
        Cam.push_cam(self);
    }
    fn pop(&self) where Camera : CastFrom<CameraOf<F>>, Self:Sized
    {
        Cam.pop_cam();
    }
    fn scope<S>(&self, scope:S) where S: FnOnce(), Camera : CastFrom<CameraOf<F>>, Self:Sized
    {
        Cam.scope_cam(self, scope);
    }*/
}

pub type Camera3D = Camera3DOf<float>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera3DOf<F> where F:Float
{
    pub position: Vector3<F>,
    pub target: Vector3<F>,
    pub up: Vector3<F>,
    pub perspective: CameraPerspectiveOf<F>,
    pub viewport : Option<Rect2i>
}

/*
impl<F> Default for Camera3DOf<F> where F:Float
{
    fn default() -> Self {
        Self { position: Vector3::ZERO.with_z(one()), target: zero(), up: Vector3::Y, perspective: ___(), viewport: None }
    }
}
*/
impl<F> GetMatrix<F,4,4> for Camera3DOf<F> where F: Float
{
    fn matrix(&self) -> Matrix<F,4,4> { self.matrix() }
}



pub type CameraPerspective = CameraPerspectiveOf<float>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CameraPerspectiveOf<F> where F:Float
{
    pub aspect: F,
    pub fovy: AngleOf<F>,
    pub znear: F,
    pub zfar: F,
}
/*
impl<F> Default for CameraPerspectiveOf<F> where F:Float
{
    fn default() -> Self
    {
        Self { aspect: (16. / 9.).cast_into(), fovy: AngleOf::from_degree(F::cast_from(45.)), znear: F::cast_from(0.1), zfar: F::cast_from(100.0) }
    }
}*/
impl<F> From<CameraPerspectiveOf<F>> for Matrix4<F> where F:Float
{
    // Based on https://docs.rs/cgmath/latest/src/cgmath/projection.rs.html#108
    fn from(p: CameraPerspectiveOf<F>) -> Self
    {
        debug_assert!(p.fovy > AngleOf::ZERO && p.fovy < AngleOf::HALF);
        debug_assert!(p.aspect != F::ZERO);
        debug_assert!(p.znear > F::ZERO && p.zfar > F::ZERO && p.zfar != p.znear);

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
            Vector4::new(F::ZERO, F::ZERO, m22, m23),
            Vector4::new(F::ZERO, F::ZERO, m32, F::ZERO)
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
        // https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
        let view: Matrix<F, 4, 4> = Matrix4::<F>::look_at_rh(self.position, self.target, self.up);
        let proj = Matrix4::<F>::from(self.perspective);
        return Self::OPENGL_TO_WGPU_MATRIX * proj * view;
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

impl<F> ICamera<F> for Camera3DOf<F> where F:Float
{
    fn have_depth(&self) -> bool { true }
}

pub type Camera = CameraOf<float>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CameraOf<F>
{
    pub matrix : Matrix4<F>,
    pub depth : bool,
}
impl<T> MapIntern for CameraOf<T>
{
    type Item=T;
    fn map_intern<F>(mut self, f: F) -> Self where F: FnMut(Self::Item) -> Self::Item {
        self.matrix = self.matrix.map_intern(f);
        self
    }
}
impl<T> Map for CameraOf<T>
{
    type WithType<R> = CameraOf<R>;

    fn map<R,F>(self, f: F) -> Self::WithType<R> where F: FnMut(Self::Item) -> R {
        Self::WithType { matrix: self.matrix.map(f), depth: self.depth }
    }
}

impl<F> GetPosition<F,3> for CameraOf<F> where F: Float
{
    fn pos(&self) -> Vector<F,3>  { self.matrix.pos() }
}
impl<F> SetPosition<F,3> for CameraOf<F> where F: Float
{
    fn set_pos(&mut self, pos : Vector<F,3>) -> &mut Self { self.matrix.set_pos(pos); self }
}
impl<F> GetScale<F,3> for CameraOf<F> where F: Float
{
    fn scale(&self) -> Vector<F,3>  { self.matrix.scale() }
}
impl<F> SetScale<F,3> for CameraOf<F> where F: Float
{
    fn set_scale(&mut self, scale : Vector<F,3>) -> &mut Self { self.matrix.set_scale(scale); self }
}
impl<F> RotateX<F> for CameraOf<F> where F: Float
{
    fn rotate_x(&mut self, angle : AngleOf<F>) -> &mut Self { self.matrix.rotate_x(angle); self }
}
impl<F> RotateY<F> for CameraOf<F> where F: Float
{
    fn rotate_y(&mut self, angle : AngleOf<F>) -> &mut Self { self.matrix.rotate_y(angle); self }
}
impl<F> RotateZ<F> for CameraOf<F> where F: Float
{
    fn rotate_z(&mut self, angle : AngleOf<F>) -> &mut Self { self.matrix.rotate_z(angle); self }
}
impl<F> GetMatrix<F,4,4> for CameraOf<F> where F: Float
{
    fn matrix(&self) -> Matrix<F,4,4> { self.matrix }
}
impl<F> SetMatrix<F,4,4> for CameraOf<F> where F: Float
{
    fn set_matrix(&mut self, matrix : Matrix<F,4,4>) -> &mut Self { self.matrix = matrix; self }
}

impl<F> CameraOf<F> where F: Float
{
    pub const CAMERA_2D : Self = Self { matrix: Matrix4::IDENTITY, depth: false };
    pub const CAMERA_3D : Self = Self { matrix: Matrix4::IDENTITY, depth: true };

    pub fn new() -> Self { Self::___() }

    pub fn with_matrix(self, matrix: Matrix4<F>) -> Self { Self { matrix, ..self }}
    pub fn with_depth(self, depth: bool) -> Self { Self { depth, ..self }}
}
impl<F> Default for CameraOf<F> where F: Float
{
    fn default() -> Self { Self::CAMERA_3D }
}
impl<F> ICamera<F> for CameraOf<F> where F: Float
{
    fn have_depth(&self) -> bool { self.depth }
}
