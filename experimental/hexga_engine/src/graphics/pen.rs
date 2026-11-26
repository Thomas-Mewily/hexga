use super::*;

#[derive(Clone, Copy)]
pub struct Pen;

impl SingletonEmptyStruct for Pen {
    fn is_init() -> bool {
        GRAPHICS.try_get_mut().is_ok()
    }
}

impl BuilderMesh for Pen {
    fn extend(&mut self, vertex: impl IntoIterator<Item = Vertex>, index: impl IntoIterator<Item = VertexIndex>) {
        GRAPHICS.get_mut().extend(vertex, index)
    }
    fn geometry(&mut self, vertex: &[vertex::VertexOf<3>], index: &[VertexIndex]) {
        GRAPHICS.get_mut().geometry(vertex, index);
    }
}

impl SetDrawParam for Pen {
    fn set_draw_param(&mut self, param: DrawParam) -> &mut Self {
        GRAPHICS.get_mut().set_draw_param(param);
        self
    }
}
impl GetDrawParam for Pen {
    fn draw_param(&self) -> DrawParam {
        GRAPHICS.get_mut().draw_param()
    }
}

impl GetMatrix<float, 4, 4> for Pen {
    fn matrix(&self) -> Matrix<float, 4, 4> {
        GRAPHICS.get_mut().matrix()
    }
}
impl SetMatrix<float, 4, 4> for Pen {
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self {
        GRAPHICS.get_mut().set_matrix(matrix);
        self
    }
}

impl GetCamera for Pen {
    fn have_depth(&self) -> bool {
        GRAPHICS.get_mut().have_depth()
    }
}
impl SetCamera for Pen {
    fn set_camera(&mut self, camera: Camera) -> &mut Self {
        GRAPHICS.get_mut().set_camera(camera);
        self
    }
}

impl GetPosition for Pen {
    fn pos(&self) -> Vec3 {
        GRAPHICS.get_mut().pos()
    }
}
impl SetPosition for Pen {
    fn set_pos(&mut self, pos: Vec3) -> &mut Self {
        GRAPHICS.get_mut().set_pos(pos);
        self
    }
}
impl GetScale for Pen {
    fn scale(&self) -> Vec3 {
        GRAPHICS.get_mut().scale()
    }
}
impl SetScale for Pen {
    fn set_scale(&mut self, scale: Vec3) -> &mut Self {
        GRAPHICS.get_mut().set_scale(scale);
        self
    }
}

impl<T> Push<T> for Pen
where
    Graphics: Push<T>,
{
    type Output = <Graphics as Push<T>>::Output;
    fn push(&mut self, value: T) -> Self::Output {
        GRAPHICS.get_mut().push(value)
    }
}
impl<Output: ?Sized> Collection<Output> for Pen
where
    Graphics: Collection<Output>,
{
    type Ref<'a>
        = <Graphics as Collection<Output>>::Ref<'a>
    where
        Self: 'a,
        Output: 'a;
}

impl<Idx> CollectionIndex<Idx> for Pen
where
    Graphics: CollectionIndex<Idx>,
{
    type Output = <Graphics as CollectionIndex<Idx>>::Output;
    type Ref<'a>
        = <Graphics as CollectionIndex<Idx>>::Ref<'a>
    where
        Self: 'a,
        Self::Output: 'a;
}
impl Dup<Matrix<float, 4, 4>> for Pen {
    fn dup(&mut self) -> bool {
        Dup::<Matrix<float, 4, 4>>::dup(&mut *GRAPHICS.get_mut())
    }
}

impl Dup<Scissor> for Pen {
    fn dup(&mut self) -> bool {
        Dup::<Scissor>::dup(&mut *GRAPHICS.get_mut())
    }
}

impl Dup<Viewport> for Pen {
    fn dup(&mut self) -> bool {
        Dup::<Viewport>::dup(&mut *GRAPHICS.get_mut())
    }
}

impl<T> Pop<T> for Pen
where
    Graphics: Pop<T>,
{
    fn pop(&mut self) -> Option<T> {
        GRAPHICS.get_mut().pop()
    }
}

impl RotateX for Pen {
    fn rotate_x(&mut self, angle: Angle) -> &mut Self {
        GRAPHICS.get_mut().rotate_x(angle);
        self
    }
}
impl RotateY for Pen {
    fn rotate_y(&mut self, angle: Angle) -> &mut Self {
        GRAPHICS.get_mut().rotate_y(angle);
        self
    }
}
impl RotateZ for Pen {
    fn rotate_z(&mut self, angle: Angle) -> &mut Self {
        GRAPHICS.get_mut().rotate_z(angle);
        self
    }
}

impl GetViewport for Pen {
    fn viewport(&self) -> Viewport {
        GRAPHICS.get_mut().viewport()
    }
}
impl SetViewport for Pen {
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self {
        GRAPHICS.get_mut().set_viewport(viewport);
        self
    }
}
impl GetScissor for Pen {
    fn scissor(&self) -> Scissor {
        GRAPHICS.get_mut().scissor()
    }
}
impl SetScissor for Pen {
    fn set_scissor(&mut self, scissor: Scissor) -> &mut Self {
        GRAPHICS.get_mut().set_scissor(scissor);
        self
    }
}
impl GetOptionalTexture for Pen {
    fn texture(&self) -> Option<Texture> {
        GRAPHICS.get_mut().texture()
    }
}
impl SetOptionalTexture for Pen {
    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self {
        GRAPHICS.get_mut().set_texture(texture);
        self
    }
}

/*
impl SetBackgroundColor for Pen
{
    fn set_background_color(&mut self, color: Color) -> &mut Self
    {
        GRAPHICS.get_mut().set_background_color(color);
        self
    }
}
*/
