use super::*;


pub struct CameraManager
{
    // pub(crate) default_camera: Camera,
    pub(crate) cameras: NonEmptyStack<Camera>,
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
}

impl ScopedDraw for CameraManager
{
    fn begin_draw(&mut self) 
    {
        assert_eq!(self.cameras.len(), 1, "Forget to pop a camera");
        self.cameras.replace(Camera::CAMERA_3D);
    }

    fn end_draw(&mut self) {
        
    }
}


impl CameraManager
{
    pub(crate) fn new(camera_buffer: wgpu::Buffer, camera_bind_group: wgpu::BindGroup) -> Self { Self { cameras: ___(), camera_buffer, camera_bind_group } }
}

impl CameraManager
{
    pub fn replace<C>(&mut self, cam: &C) -> &mut Self where C: ICamera 
    { 
        self.cameras.replace(cam.to_camera());
        self.apply();
        self 
    }
    pub fn apply(&mut self) 
    {
        let m = self.matrix();
        //info!("pushed matrix");
        //info!("{}", m);
        Gpu.queue.write_buffer(&Cam.camera_buffer, 0, m.as_u8_slice());
    }
}
impl GetMatrix<float,4,4> for CameraManager
{
    fn matrix(&self) -> Matrix<float,4,4> {
        self.cameras.matrix()
    }
}
impl SetMatrix<float,4,4> for CameraManager
{
    fn set_matrix(&mut self, matrix : Matrix<float,4,4>) -> &mut Self {
        self.cameras.set_matrix(matrix); self
    }
}

impl ICamera for CameraManager
{
    fn have_depth(&self) -> bool { self.cameras.have_depth() }
    fn viewport(&self) -> Option<Rect2P> { self.cameras.viewport() }
}

impl GetPosition<float,3> for CameraManager
{
    fn pos(&self) -> Vector<float,3> { self.cameras.pos() }
}
impl SetPosition<float,3> for CameraManager
{
    fn set_pos(&mut self, pos : Vector<float,3>) -> &mut Self { self.cameras.set_pos(pos); self.apply(); self }
}
impl GetScale<float,3> for CameraManager
{
    fn scale(&self) -> Vector<float,3> { self.cameras.scale() }
}
impl SetScale<float,3> for CameraManager
{
    fn set_scale(&mut self, scale : Vector<float,3>) -> &mut Self { self.cameras.set_scale(scale); self.apply(); self }
}
impl RotateX<float> for CameraManager
{
    fn rotate_x(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_x(angle); self.apply(); self }
}
impl RotateY<float> for CameraManager
{
    fn rotate_y(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_y(angle); self.apply(); self }
}
impl RotateZ<float> for CameraManager
{
    fn rotate_z(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_z(angle); self.apply(); self }
}