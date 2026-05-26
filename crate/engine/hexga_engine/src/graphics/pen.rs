use super::*;

#[derive(Clone, Copy)]
pub struct Pen;

impl SingletonEmptyStruct for Pen
{
    fn is_init() -> bool { GRAPHICS.try_get_mut().is_ok() }
}

impl BuilderMesh for Pen
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = Vertex>, index: impl IntoIterator<Item = VertexIndex>)
    {
        GRAPHICS.get_mut().geometry(vertex, index)
    }
}

impl GetMatrix<float, 4, 4> for Pen
{
    fn matrix(&self) -> Matrix<float, 4, 4> { GRAPHICS.get_mut().matrix() }
}
impl SetMatrix<float, 4, 4> for Pen
{
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self
    {
        GRAPHICS.get_mut().set_matrix(matrix);
        self
    }
}

impl GetCamera for Pen
{
    fn have_depth(&self) -> bool { GRAPHICS.get_mut().have_depth() }
}
impl SetCamera for Pen
{
    fn set_camera(&mut self, camera: Camera) -> &mut Self
    {
        GRAPHICS.get_mut().set_camera(camera);
        self
    }
}

impl GetPosition for Pen
{
    fn pos(&self) -> Vec3 { GRAPHICS.get_mut().pos() }
}
impl SetPosition for Pen
{
    fn set_pos(&mut self, pos: Vec3) -> &mut Self
    {
        GRAPHICS.get_mut().set_pos(pos);
        self
    }
}
impl GetScale for Pen
{
    fn scale(&self) -> Vec3 { GRAPHICS.get_mut().scale() }
}
impl SetScale for Pen
{
    fn set_scale(&mut self, scale: Vec3) -> &mut Self
    {
        GRAPHICS.get_mut().set_scale(scale);
        self
    }
}
impl RotateX for Pen
{
    fn rotate_x(&mut self, angle: Angle) -> &mut Self
    {
        GRAPHICS.get_mut().rotate_x(angle);
        self
    }
}
impl RotateY for Pen
{
    fn rotate_y(&mut self, angle: Angle) -> &mut Self
    {
        GRAPHICS.get_mut().rotate_y(angle);
        self
    }
}
impl RotateZ for Pen
{
    fn rotate_z(&mut self, angle: Angle) -> &mut Self
    {
        GRAPHICS.get_mut().rotate_z(angle);
        self
    }
}

impl DrawParamAttribute for Pen
{
    fn viewport(&self) -> Viewport { GRAPHICS.get_mut().viewport() }
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self
    {
        GRAPHICS.get_mut().set_viewport(viewport);
        self
    }

    fn scissor(&self) -> Rect2i { GRAPHICS.get_mut().scissor() }

    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self
    {
        GRAPHICS.get_mut().set_scissor(scissor);
        self
    }

    fn texture(&self) -> Option<Texture> { GRAPHICS.get_mut().texture() }

    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self
    {
        GRAPHICS.get_mut().set_texture(texture);
        self
    }
}
