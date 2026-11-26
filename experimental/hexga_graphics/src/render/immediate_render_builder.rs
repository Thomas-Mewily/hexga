use super::*;

#[derive(Debug)]
pub struct ImmediateRenderBuilder<const N: usize = 3> {
    pub render: ImmediateRender<N>,
    pub params: NonEmptyStack<DrawParam>,

    pub(crate) matrix_stack: NonEmptyStack<Matrix<float, 4, 4>>,
    pub(crate) viewports_stack: NonEmptyStack<Viewport>,
    pub(crate) scissor_stack: NonEmptyStack<Rect2i>,
}

impl<const N: usize> Default for ImmediateRenderBuilder<N> {
    fn default() -> Self {
        Self {
            render: ___(),
            params: ___(),
            matrix_stack: NonEmptyStack::new(Matrix::IDENTITY),
            viewports_stack: NonEmptyStack::new(Viewport::default()),
            scissor_stack: NonEmptyStack::new(Rect2i::default()),
        }
    }
}

impl<const N: usize> BuilderMesh<N> for ImmediateRenderBuilder<N> {
    fn extend(&mut self, vertex: impl IntoIterator<Item = VertexOf<N>>, index: impl IntoIterator<Item = VertexIndex>) {
        self.ensure_immediate_draw_call();
        self.render.geometry.extend(vertex, index);
        self.update_last_draw_call();
    }
    fn geometry(&mut self, vertex: &[VertexOf<N>], index: &[VertexIndex]) {
        self.ensure_immediate_draw_call();
        self.render.geometry.geometry(vertex, index);
        self.update_last_draw_call();
    }
}

impl<const N: usize> GetMatrix<float, 4, 4> for ImmediateRenderBuilder<N> {
    fn matrix(&self) -> Matrix<float, 4, 4> {
        self.params.camera.matrix()
    }
}
impl<const N: usize> SetMatrix<float, 4, 4> for ImmediateRenderBuilder<N> {
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self {
        self.map_draw_param(|p| p.camera.matrix = matrix);
        *self.matrix_stack.last_mut() = matrix;
        self
    }
}

impl<const N: usize> GetCamera for ImmediateRenderBuilder<N> {
    fn have_depth(&self) -> bool {
        self.params.camera.have_depth()
    }
}
impl<const N: usize> SetCamera for ImmediateRenderBuilder<N> {
    fn set_camera(&mut self, camera: Camera) -> &mut Self {
        self.map_draw_param(|p| p.camera = camera);
        *self.matrix_stack.last_mut() = camera.matrix;
        self
    }
}

impl<const N: usize> GetPosition for ImmediateRenderBuilder<N> {
    fn pos(&self) -> Vec3 {
        self.params.camera.pos()
    }
}
impl<const N: usize> SetPosition for ImmediateRenderBuilder<N> {
    fn set_pos(&mut self, pos: Vec3) -> &mut Self {
        self.map_draw_param(|p| {
            p.camera.set_pos(pos);
        });
        *self.matrix_stack.last_mut() = self.params.camera.matrix;
        self
    }
}
impl<const N: usize> GetScale for ImmediateRenderBuilder<N> {
    fn scale(&self) -> Vec3 {
        self.params.camera.scale()
    }
}
impl<const N: usize> SetScale for ImmediateRenderBuilder<N> {
    fn set_scale(&mut self, scale: Vec3) -> &mut Self {
        self.map_draw_param(|p| {
            p.camera.set_scale(scale);
        });
        *self.matrix_stack.last_mut() = self.params.camera.matrix;
        self
    }
}
impl<const N: usize> RotateX for ImmediateRenderBuilder<N> {
    fn rotate_x(&mut self, angle: Angle) -> &mut Self {
        self.map_draw_param(|p| {
            p.camera.rotate_x(angle);
        });
        *self.matrix_stack.last_mut() = self.params.camera.matrix;
        self
    }
}
impl<const N: usize> RotateY for ImmediateRenderBuilder<N> {
    fn rotate_y(&mut self, angle: Angle) -> &mut Self {
        self.map_draw_param(|p| {
            p.camera.rotate_y(angle);
        });
        *self.matrix_stack.last_mut() = self.params.camera.matrix;
        self
    }
}
impl<const N: usize> RotateZ for ImmediateRenderBuilder<N> {
    fn rotate_z(&mut self, angle: Angle) -> &mut Self {
        self.map_draw_param(|p| {
            p.camera.rotate_z(angle);
        });
        *self.matrix_stack.last_mut() = self.params.camera.matrix;
        self
    }
}

impl<const N: usize> GetViewport for ImmediateRenderBuilder<N> {
    fn viewport(&self) -> Viewport {
        self.params.viewport
    }
}
impl<const N: usize> SetViewport for ImmediateRenderBuilder<N> {
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self {
        self.map_draw_param(|p| p.viewport = viewport);
        *self.viewports_stack.last_mut() = viewport;
        self
    }
}
impl<const N: usize> GetScissor for ImmediateRenderBuilder<N> {
    fn scissor(&self) -> Rect2i {
        self.params.scissor
    }
}
impl<const N: usize> SetScissor for ImmediateRenderBuilder<N> {
    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self {
        self.map_draw_param(|p| p.scissor = scissor);
        *self.scissor_stack.last_mut() = scissor;
        self
    }
}
impl<const N: usize> GetOptionalTexture for ImmediateRenderBuilder<N> {
    fn texture(&self) -> Option<Texture> {
        self.params.texture.clone_handle()
    }
}
impl<const N: usize> SetOptionalTexture for ImmediateRenderBuilder<N> {
    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self {
        self.map_draw_param(|p| p.texture = texture.into());
        self
    }
}

impl<const N: usize> SetDrawParam for ImmediateRenderBuilder<N> {
    fn set_draw_param(&mut self, param: DrawParam) -> &mut Self {
        if self.params.last() == &param {
            return self;
        }
        self.update_last_draw_call();
        *self.params.last_mut() = param;
        self
    }
}
impl<const N: usize> GetDrawParam for ImmediateRenderBuilder<N> {}

impl<const N: usize> ImmediateRenderBuilder<N> {
    fn ensure_immediate_draw_call(&mut self) {
        let param = self.params.last().clone();
        if let Some(last) = self.render.draw_call.last_mut() {
            if let DrawGeometry::Immediate(immediate) = &last.geometry {
                if immediate.is_empty() {
                    last.param = param;
                    return;
                }
                if last.param == param {
                    return;
                }
            }
        }

        let start = self.render.geometry.nb_vertex();
        let index_start = self.render.geometry.nb_index();
        self.render.draw_call.push(DrawCall {
            geometry: DrawGeometry::Immediate(DrawGeometryImmediate {
                vertices: Range { start, end: start },
                indices: Range {
                    start: index_start,
                    end: index_start,
                },
            }),
            param,
        });
    }

    fn update_last_draw_call(&mut self) {
        if let Some(DrawCall {
            geometry: DrawGeometry::Immediate(immediate),
            ..
        }) = self.render.draw_call.last_mut()
        {
            immediate.indices.end = self.render.geometry.nb_index();
            immediate.vertices.end = self.render.geometry.nb_vertex();
        }
    }

    fn pop_immediate_if_empty(&mut self) {
        if matches!(self.render.draw_call.last(), Some(DrawCall { geometry: DrawGeometry::Immediate(immediate), .. }) if immediate.is_empty()) {
            self.render.draw_call.pop();
        }
    }

    pub fn retain(&mut self, mesh: GMesh<N>) -> &mut Self {
        self.update_last_draw_call();
        self.pop_immediate_if_empty();

        self.render.draw_call.push(DrawCall {
            geometry: DrawGeometry::Retain(mesh),
            param: self.params.last().clone(),
        });
        self
    }

    pub fn clear(&mut self) {
        self.render.clear();
        self.params = NonEmptyStack::new(___());
        self.matrix_stack = NonEmptyStack::new(Matrix::IDENTITY);
        self.viewports_stack = NonEmptyStack::new(___());
        self.scissor_stack = NonEmptyStack::new(___());
    }
}

/*
impl<const N: usize> DrawCallBuilder<N> for ImmediateRenderBuilder<N>
{
    fn set_param(&mut self, param: DrawParam)
    {
        if self.params.last() == &param
        {
            return;
        }

        self.update_last_draw_call();
        *self.params.last_mut() = param;
    }

    fn param(&self) -> DrawParam { self.params.last().clone() }
}
*/
/*
pub trait DrawCallBuilder<const N: usize = 3>: BuilderMesh<N> + DrawParamExtension + GetCamera
{
    fn param(&self) -> DrawParam { self.draw_param() }
    fn set_param(&mut self, param: DrawParam) -> &mut Self { self.set_draw_param(param) }
    fn map_draw_param<F, O>(&mut self, f: F) -> O where F: FnOnce(&mut DrawParam) -> O { self.map_draw_param(f) }
}
impl<S,const N: usize> DrawCallBuilder<N> for S where S: BuilderMesh<N> + DrawParamExtension + GetCamera {}
*/

/*
impl Builder for ImmediateRenderBuilder
{
    type Output = ImmediateRender;

    fn build(&self) -> Self::Output {

    }

    fn build_in(&self, dest: &mut Self::Output) {
        dest.draw_call.clear();
        dest.draw_call = self.draw_call
    }
}
*/
/*
impl RenderImmediate for ImmediateRenderBuilder
{

}

impl RenderImmediate for ImmediateRenderBuilder
{

}*/
