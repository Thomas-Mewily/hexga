use std::range::Range;

use super::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{DrawCall, DrawGeometry, DrawGeometryImmediate, ImmediateRender, ImmediateRenderBuilder};
}

pub mod traits
{
    pub use super::{DrawCallBuilder, DrawParamAttribute};
}

#[derive(Debug, Default)]
pub struct ImmediateRenderBuilder<const N: usize = 3>
{
    pub draw_call: NonEmptyStack<DrawCall>,
    pub big_mesh: MeshBuilder,
    pub params: NonEmptyStack<DrawParam>,
}

impl BuilderMesh for ImmediateRenderBuilder
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = VertexOf>, index: impl IntoIterator<Item = VertexIndex>) { self.big_mesh.geometry(vertex, index); }
}

impl GetMatrix<float, 4, 4> for ImmediateRenderBuilder
{
    fn matrix(&self) -> Matrix<float, 4, 4> { self.params.camera.matrix() }
}
impl SetMatrix<float, 4, 4> for ImmediateRenderBuilder
{
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self
    {
        self.param_map(|p| p.camera.matrix = matrix);
        self
    }
}

impl GetCamera for ImmediateRenderBuilder
{
    fn have_depth(&self) -> bool { self.params.camera.have_depth() }
}
impl SetCamera for ImmediateRenderBuilder
{
    fn set_camera(&mut self, camera: Camera) -> &mut Self
    {
        self.param_map(|p| p.camera = camera);
        self
    }
}

impl GetPosition for ImmediateRenderBuilder
{
    fn pos(&self) -> Vec3 { self.params.camera.pos() }
}
impl SetPosition for ImmediateRenderBuilder
{
    fn set_pos(&mut self, pos: Vec3) -> &mut Self
    {
        self.param_map(|p| {
            p.camera.set_pos(pos);
        });
        self
    }
}
impl GetScale for ImmediateRenderBuilder
{
    fn scale(&self) -> Vec3 { self.params.camera.scale() }
}
impl SetScale for ImmediateRenderBuilder
{
    fn set_scale(&mut self, scale: Vec3) -> &mut Self
    {
        self.param_map(|p| {
            p.camera.set_scale(scale);
        });
        self
    }
}
impl RotateX for ImmediateRenderBuilder
{
    fn rotate_x(&mut self, angle: Angle) -> &mut Self
    {
        self.param_map(|p| {
            p.camera.rotate_x(angle);
        });
        self
    }
}
impl RotateY for ImmediateRenderBuilder
{
    fn rotate_y(&mut self, angle: Angle) -> &mut Self
    {
        self.param_map(|p| {
            p.camera.rotate_y(angle);
        });
        self
    }
}
impl RotateZ for ImmediateRenderBuilder
{
    fn rotate_z(&mut self, angle: Angle) -> &mut Self
    {
        self.param_map(|p| {
            p.camera.rotate_z(angle);
        });
        self
    }
}

impl DrawParamAttribute for ImmediateRenderBuilder
{
    fn viewport(&self) -> Viewport { self.params.viewport }
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self
    {
        self.param_map(|p| p.viewport = viewport);
        self
    }

    fn scissor(&self) -> Rect2i { self.params.scissor }

    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self
    {
        self.param_map(|p| p.scissor = scissor);
        self
    }

    fn texture(&self) -> Option<Texture> { self.params.texture.clone_handle() }

    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self
    {
        self.param_map(|p| p.texture = texture.into());
        self
    }
}

impl ImmediateRenderBuilder
{
    pub(crate) fn update_last_draw_call(&mut self)
    {
        let mesh = &mut self.big_mesh;

        let immediate_mode = match &mut self.draw_call.last_mut().geometry
        {
            DrawGeometry::Immediate(immediate) =>
            {
                immediate.indices.end = mesh.nb_index();
                immediate.vertices.end = mesh.nb_vertex();
            }
        };
    }
}

impl DrawCallBuilder for ImmediateRenderBuilder
{
    fn set_param(&mut self, param: DrawParam)
    {
        if self.draw_call.param == param
        {
            return;
        }

        self.update_last_draw_call();

        if !self.draw_call.is_geometry_empty()
        {
            let mut draw_call = self.draw_call.last().clone();
            draw_call.geometry = DrawGeometry::Immediate(DrawGeometryImmediate {
                vertices: Range {
                    start: self.big_mesh.nb_vertex(),
                    end: self.big_mesh.nb_vertex(),
                },
                indices: Range {
                    start: self.big_mesh.nb_index(),
                    end: self.big_mesh.nb_index(),
                },
            });
            self.draw_call.push(draw_call);
        }
        self.draw_call.param = param;
    }

    fn param(&self) -> DrawParam { self.params.last().clone() }
}

pub trait DrawCallBuilder<const N: usize = 3>: BuilderMesh + DrawParamAttribute + GetCamera
{
    fn param(&self) -> DrawParam;
    fn set_param(&mut self, param: DrawParam);

    fn param_map<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(&mut DrawParam) -> O,
    {
        let mut param = self.param();
        let o = f(&mut param);
        self.set_param(param);
        o
    }
}

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

#[derive(Clone, Debug, Default)]
pub struct ImmediateRender
{
    pub draw_call: Vec<DrawCall>,
}

#[derive(Clone, Debug)]
pub enum DrawGeometry
{
    Immediate(DrawGeometryImmediate),
}
impl DrawGeometry
{
    pub fn is_empty(&self) -> bool
    {
        match self
        {
            DrawGeometry::Immediate(immediate) => immediate.is_empty(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices: Range<usize>,
    pub indices: Range<usize>,
}
impl DrawGeometryImmediate
{
    pub fn is_empty(&self) -> bool
    {
        debug_assert_eq!(self.vertices.is_empty(), self.indices.is_empty());
        self.vertices.is_empty() || self.indices.is_empty()
    }
}

/*
#[derive(Clone, Debug, Default)]
pub struct DrawGeometryImmediate
{
    pub vertices_begin: usize,
    pub vertices_len: usize,

    pub indices_begin: usize,
    pub indices_len: usize,
}
*/
/*
pub struct DrawGeometrySliceIndice
{
    pub begin: usize,
    pub len: usize,
}
*/

impl Default for DrawGeometry
{
    fn default() -> Self { Self::Immediate(___()) }
}

#[derive(Clone, Debug, Default)]
pub struct DrawCall
{
    pub geometry: DrawGeometry,
    pub param: DrawParam,
}
impl Deref for DrawCall
{
    type Target = DrawParam;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl DerefMut for DrawCall
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}
impl DrawCall
{
    pub fn is_geometry_empty(&self) -> bool { self.geometry.is_empty() }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct DrawParam
{
    pub camera: Camera,
    pub viewport: Viewport,
    pub scissor: Rect2i,
    pub texture: Option<Texture>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Viewport
{
    pub area: Rect2,
    pub depth: Range<float>,
}

impl Default for Viewport
{
    fn default() -> Self
    {
        Self {
            area: ___(),
            depth: Range { start: 0., end: 1. },
        }
    }
}

impl GetMatrix<float, 4, 4> for DrawParam
{
    fn matrix(&self) -> Matrix<float, 4, 4> { self.camera.matrix }
}
impl SetMatrix<float, 4, 4> for DrawParam
{
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self
    {
        self.camera.matrix = matrix;
        self
    }
}

impl GetCamera for DrawParam
{
    fn have_depth(&self) -> bool { self.camera.have_depth() }
}

impl SetCamera for DrawParam
{
    fn set_camera(&mut self, camera: CameraOf<f32>) -> &mut Self
    {
        self.camera = camera;
        self
    }
}
impl GetPosition for DrawParam
{
    fn pos(&self) -> Vec3 { self.camera.pos() }
}
impl SetPosition for DrawParam
{
    fn set_pos(&mut self, pos: Vec3) -> &mut Self
    {
        self.camera.set_pos(pos);
        self
    }
}
impl GetScale for DrawParam
{
    fn scale(&self) -> Vec3 { self.camera.scale() }
}
impl SetScale for DrawParam
{
    fn set_scale(&mut self, scale: Vec3) -> &mut Self
    {
        self.camera.set_scale(scale);
        self
    }
}
impl RotateX for DrawParam
{
    fn rotate_x(&mut self, angle: Angle) -> &mut Self
    {
        self.camera.rotate_x(angle);
        self
    }
}
impl RotateY for DrawParam
{
    fn rotate_y(&mut self, angle: Angle) -> &mut Self
    {
        self.camera.rotate_y(angle);
        self
    }
}
impl RotateZ for DrawParam
{
    fn rotate_z(&mut self, angle: Angle) -> &mut Self
    {
        self.camera.rotate_z(angle);
        self
    }
}

impl DrawParamAttribute for DrawParam
{
    fn viewport(&self) -> Viewport { self.viewport }

    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self
    {
        self.viewport = viewport;
        self
    }

    fn scissor(&self) -> Rect2i { self.scissor }

    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self
    {
        self.scissor = scissor;
        self
    }

    fn texture(&self) -> Option<Texture> { self.texture.clone() }

    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self
    {
        self.texture = texture.into();
        self
    }
}

pub trait DrawParamAttribute: GetCamera + SetCamera + GetPosition + SetPosition + GetScale + SetScale + RotateX + RotateY + RotateZ
{
    fn viewport(&self) -> Viewport;
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self;

    fn scissor(&self) -> Rect2i;
    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self;

    fn texture(&self) -> Option<Texture>;
    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self;

    fn draw_param(&self) -> DrawParam
    {
        DrawParam {
            camera: self.camera(),
            viewport: self.viewport(),
            scissor: self.scissor(),
            texture: self.texture(),
        }
    }
    fn set_draw_param(&mut self, param: DrawParam) -> &mut Self
    {
        self.set_viewport(param.viewport);
        self.set_camera(param.camera);
        self.set_scissor(param.scissor);
        self.set_texture(param.texture);
        self
    }
}
