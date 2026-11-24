//use std::ops::Range;
//use std::range::Range;

use super::*;

mod immediate_render;
pub use immediate_render::*;

mod immediate_render_builder;
pub use immediate_render_builder::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{DrawCall, DrawParam,
        immediate_render::{DrawGeometry, DrawGeometryImmediate, ImmediateRender},
        immediate_render_builder::ImmediateRenderBuilder
    };
}

pub mod traits
{
    pub use super::{
        //DrawCallBuilder, 
        DrawParamExtension, 
        
        ImmediateRendering, 
        ImmediateStackableRendering,

        DrawStackExtension, 

        GetDrawParam, SetDrawParam,
        GetOptionalTexture, SetOptionalTexture,
    };
}

use prelude::*;


impl<const N: usize> Default for DrawGeometry<N>
{
    fn default() -> Self { Self::Immediate(___()) }
}

#[derive(Clone, Debug, Default)]
pub struct DrawCall<const N: usize = 3>
{
    pub geometry: DrawGeometry<N>,
    pub param: DrawParam,
}
impl<const N: usize> Deref for DrawCall<N>
{
    type Target = DrawParam;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl<const N: usize> DerefMut for DrawCall<N>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}
impl<const N: usize> DrawCall<N>
{
    pub fn is_geometry_empty(&self) -> bool { self.geometry.is_empty() }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct DrawParam
{
    pub camera  : Camera,
    pub viewport: Viewport,
    pub scissor : Scissor,
    pub texture : Option<Texture>,
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


impl GetScissor for DrawParam
{
    fn scissor(&self) -> Scissor { self.scissor }
}
impl SetScissor for DrawParam
{
    fn set_scissor(&mut self, scissor: Scissor) -> &mut Self
    {
        self.scissor = scissor;
        self
    }
}
impl GetOptionalTexture for DrawParam
{
    fn texture(&self) -> Option<Texture> { self.texture.clone_handle() }
}
impl SetOptionalTexture for DrawParam
{
    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self
    {
        self.texture = texture.into();
        self
    }
}

pub trait GetOptionalTexture
{
    fn texture(&self) -> Option<Texture>;
}
pub trait SetOptionalTexture
{
    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self;
}

pub trait GetDrawParam : GetCamera + GetScissor + GetViewport + GetOptionalTexture + GetPosition + GetScale
{
    fn draw_param(&self) -> DrawParam
    {
        DrawParam {
            camera: self.camera(),
            viewport: self.viewport(),
            scissor: self.scissor(),
            texture: self.texture(),
        }
    }
}
pub trait SetDrawParam : SetCamera + SetScissor + SetViewport + SetOptionalTexture + SetPosition + SetScale
{
    fn set_draw_param(&mut self, param: DrawParam) -> &mut Self
    {
        self.set_camera(param.camera);
        self.set_viewport(param.viewport);
        self.set_scissor(param.scissor);
        self.set_texture(param.texture);
        self
    }

    fn map_draw_param<F, O>(&mut self, f: F) -> O where Self: GetDrawParam,
        F: FnOnce(&mut DrawParam) -> O,
    {
        let mut param = self.draw_param();
        let o = f(&mut param);
        self.set_draw_param(param);
        o
    }
}

pub trait DrawParamExtension: GetDrawParam + SetDrawParam + RotateX + RotateY + RotateZ {}
impl<T> DrawParamExtension for T where T: GetDrawParam + SetDrawParam + RotateX + RotateY + RotateZ {}





pub trait DrawStackExtension:
    MatrixStackExtension + ViewportStackExtension + ScissorStackExtension
{}
impl<T> DrawStackExtension for T
where
    T: MatrixStackExtension + ViewportStackExtension + ScissorStackExtension,
{}


impl<const N: usize> Collection<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    type Ref<'a> = &'a Matrix<float, 4, 4> where Self: 'a, Matrix<float, 4, 4>: 'a;
}
impl<const N: usize> CollectionMut<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Matrix<float, 4, 4> where Self: 'a, Matrix<float, 4, 4>: 'a;
}

impl<const N: usize> CollectionIndex<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    type Output = Matrix<float, 4, 4>;
    type Ref<'a> = &'a Matrix<float, 4, 4> where Self: 'a;
}
impl<const N: usize> CollectionIndexMut<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Matrix<float, 4, 4> where Self: 'a;
}
impl<const N: usize> Last<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a { Some(self.matrix_stack.last()) }
}
impl<const N: usize> LastMut<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a { Some(self.matrix_stack.last_mut()) }
}
impl<const N: usize> Push<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    type Output = ();
    fn push(&mut self, matrix: Matrix<float, 4, 4>) { Push::push(&mut self.matrix_stack, matrix); }
}
impl<const N: usize> Pop<Matrix<float, 4, 4>> for ImmediateRenderBuilder<N>
{
    fn pop(&mut self) -> Option<Matrix<float, 4, 4>> { Pop::pop(&mut self.matrix_stack) }
}

impl<const N: usize> Collection<Viewport> for ImmediateRenderBuilder<N>
{
    type Ref<'a> = &'a Viewport where Self: 'a, Viewport: 'a;
}
impl<const N: usize> CollectionMut<Viewport> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Viewport where Self: 'a, Viewport: 'a;
}

impl<const N: usize> CollectionIndex<Viewport> for ImmediateRenderBuilder<N>
{
    type Output = Viewport;
    type Ref<'a> = &'a Viewport where Self: 'a;
}
impl<const N: usize> CollectionIndexMut<Viewport> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Viewport where Self: 'a;
}
impl<const N: usize> Last<Viewport> for ImmediateRenderBuilder<N>
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a { Some(self.viewports_stack.last()) }
}
impl<const N: usize> LastMut<Viewport> for ImmediateRenderBuilder<N>
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a { Some(self.viewports_stack.last_mut()) }
}
impl<const N: usize> Push<Viewport> for ImmediateRenderBuilder<N>
{
    type Output = ();
    fn push(&mut self, viewport: Viewport) { Push::push(&mut self.viewports_stack, viewport); }
}
impl<const N: usize> Pop<Viewport> for ImmediateRenderBuilder<N>
{
    fn pop(&mut self) -> Option<Viewport> { Pop::pop(&mut self.viewports_stack) }
}

impl<const N: usize> Collection<Scissor> for ImmediateRenderBuilder<N>
{
    type Ref<'a> = &'a Scissor where Self: 'a, Scissor: 'a;
}
impl<const N: usize> CollectionMut<Scissor> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Scissor where Self: 'a, Scissor: 'a;
}

impl<const N: usize> CollectionIndex<Scissor> for ImmediateRenderBuilder<N>
{
    type Output = Scissor;
    type Ref<'a> = &'a Scissor where Self: 'a;
}
impl<const N: usize> CollectionIndexMut<Scissor> for ImmediateRenderBuilder<N>
{
    type RefMut<'a> = &'a mut Scissor where Self: 'a;
}
impl<const N: usize> Last<Scissor> for ImmediateRenderBuilder<N>
{
    fn last<'a>(&'a self) -> Option<Self::Ref<'a>> where Self::Output: 'a { Some(self.scissor_stack.last()) }
}
impl<const N: usize> LastMut<Scissor> for ImmediateRenderBuilder<N>
{
    fn last_mut<'a>(&'a mut self) -> Option<Self::RefMut<'a>> where Self::Output: 'a { Some(self.scissor_stack.last_mut()) }
}
impl<const N: usize> Push<Scissor> for ImmediateRenderBuilder<N>
{
    type Output = ();
    fn push(&mut self, scissor: Scissor) { Push::push(&mut self.scissor_stack, scissor); }
}
impl<const N: usize> Pop<Scissor> for ImmediateRenderBuilder<N>
{
    fn pop(&mut self) -> Option<Scissor> { Pop::pop(&mut self.scissor_stack) }
}

/// The combined interface for building immediate geometry and setting its draw parameters.
pub trait ImmediateRendering<const N: usize = 3>: BuilderMesh<N> + GetDrawParam + SetDrawParam {}
impl<T, const N: usize> ImmediateRendering<N> for T where T: BuilderMesh<N> + GetDrawParam + SetDrawParam {}

/// Immediate rendering with support for scoped draw parameter changes.
pub trait ImmediateStackableRendering<const N: usize = 3>: ImmediateRendering<N> + DrawStackExtension {}
impl<T, const N: usize> ImmediateStackableRendering<N> for T where T: ImmediateRendering<N> + DrawStackExtension {}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn geometry_updates_draw_ranges()
    {
        let mut builder = ImmediateRenderBuilder::default();
        builder.geometry(&[Vertex::new(); 4], &[0, 1, 2, 0, 2, 3]);
        builder.extend([Vertex::new(); 3], [0, 1, 2]);

        let DrawGeometry::Immediate(geometry) = builder.render.draw_call.last().unwrap().geometry else { panic!("Expected immediate geometry"); };
        assert_eq!(geometry.vertices.start, 0);
        assert_eq!(geometry.vertices.end, 7);
        assert_eq!(geometry.indices.start, 0);
        assert_eq!(geometry.indices.end, 9);
        assert_eq!(builder.render.geometry.indices(), &[0, 1, 2, 0, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn parameter_changes_preserve_state_and_split_draw_calls()
    {
        let mut builder = ImmediateRenderBuilder::default();
        let viewport = Viewport {
            area: vec2(640., 480.).to_rect(),
            ..Default::default()
        };
        let scissor = Vec2i::new(320, 240).to_rect();
        builder.set_viewport(viewport);
        builder.set_scissor(scissor);
        builder.geometry(&[Vertex::new(); 3], &[0, 1, 2]);
        builder.set_camera(Camera::CAMERA_2D);
        builder.geometry(&[Vertex::new(); 3], &[0, 1, 2]);

        assert_eq!(builder.viewport(), viewport);
        assert_eq!(builder.scissor(), scissor);
        assert_eq!(builder.render.draw_call.len(), 2);
        let calls: Vec<_> = builder.render.draw_call.iter().collect();
        assert_eq!(calls[0].viewport, viewport);
        assert_eq!(calls[1].viewport, viewport);
        assert_eq!(calls[1].scissor, scissor);
        assert_eq!(calls[1].camera, Camera::CAMERA_2D);
        let DrawGeometry::Immediate(geometry) = calls[1].geometry else { panic!("Expected immediate geometry"); }   ;
        assert_eq!(geometry.vertices.start, 3);
        assert_eq!(geometry.vertices.end, 6);
        assert_eq!(geometry.indices.start, 3);
        assert_eq!(geometry.indices.end, 6);
    }

    #[test]
    fn duplication_uses_the_index_for_each_draw_stack()
    {
        let mut builder = ImmediateRenderBuilder::<3>::default();
        let matrix = Matrix::ZERO;
        let viewport = Viewport { depth: (0.25..0.75).into(), ..Viewport::default() };
        let scissor = Vec2i::new(4, 4).to_rect();

        builder.push_matrix(matrix);
        builder.push_viewport(viewport);
        builder.push_scissor(scissor);
        assert!(builder.dup_matrix());
        assert!(builder.dup_viewport());
        assert!(builder.dup_scissor());

        for _ in 0..2
        {
            assert_eq!(builder.pop_matrix(), Some(matrix));
            assert_eq!(builder.pop_viewport(), Some(viewport));
            assert_eq!(builder.pop_scissor(), Some(scissor));
        }
    }
}
