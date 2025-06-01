//use hexga_engine::context::Context;
use hexga_engine_base::modules::*;

use crate::*;
use super::convert::*;

pub struct QuadContext
{
    tmp_vertex  : Vec<miniquad::BufferId>,
    tmp_textures : Vec<miniquad::TextureId>,
    render       : Box<dyn miniquad::RenderingBackend>,
    texture      : SlotVec<miniquad::TextureId>,
}

use render::*;

impl RenderBackend for QuadContext
{
    fn new_buffer   (&mut self, data: buffer::BufferData, source : buffer::BufferSource) -> buffer::RawBufferID
    {
        self.render.new_buffer(data.kind.convert(), data.usage.convert(), source.convert()).convert()
    }

    fn buffer_update(&mut self, dest: buffer::RawBufferID, source: &buffer::BufferSource)
    {
        self.render.buffer_update(dest.convert(), source.convert());
    }

    fn delete_buffer(&mut self, id : buffer::RawBufferID) {
        self.render.delete_buffer(id.convert());
    }

    fn new_texture(&mut self, data : &texture::TextureData) -> texture::RawTextureID
    {
        let src = match &data.source
        {
            texture::TextureSource::Empty => miniquad::TextureSource::Empty,
            texture::TextureSource::RGBA8(items) => miniquad::TextureSource::Bytes(&items),
        };

        let access = data.param.access.convert();

        let t = self.render.new_texture(
            access,
            src,
            data.param.convert()
        );

        let [wrap_x, wrap_y] = data.param.wrap.into();
        if wrap_x != wrap_y
        {
            self.render.texture_set_wrap(t, wrap_x.convert(), wrap_y.convert());
        }

        let id = self.texture.insert(t);
        texture::RawTextureID { index: id.index() }
    }

    fn texture_update(&mut self, dest : texture::RawTextureID, data : &texture::TextureData) {

        let src = match &data.source
        {
            texture::TextureSource::Empty => return,
            texture::TextureSource::RGBA8(items) => items,
        };

        let id = self.texture[dest.index];

        self.render.texture_update(id, src);
    }

    fn texture_set_mag_filter  (&mut self, id : texture::RawTextureID, filter: texture::FilterMode)
    {
        let id = self.texture[id.index];
        self.render.texture_set_mag_filter(id, filter.convert());
    }

    fn texture_set_wrap        (&mut self, id : texture::RawTextureID, wrap : (texture::TextureWrap, texture::TextureWrap)) {
        let id = self.texture[id.index];
        self.render.texture_set_wrap(id, wrap.0.convert(), wrap.1.convert());
    }

    fn texture_generate_mipmaps(&mut self, id : texture::RawTextureID) {
        let id = self.texture[id.index];
        self.render.texture_generate_mipmaps(id);
    }

    fn texture_read_pixels     (&mut self, id : texture::RawTextureID, dest : &mut texture::TextureSource)
    {
        let dest = match dest
        {
            texture::TextureSource::Empty => return,
            texture::TextureSource::RGBA8(items) => items.as_mut_slice(),
        };
        let id = self.texture[id.index];
        self.render.texture_read_pixels(id, dest);
    }

    fn texture_update_portion  (&mut self, id : texture::RawTextureID, portion : Rect2P, source : &texture::TextureSource) {
        let src = match &source
        {
            texture::TextureSource::Empty => return,
            texture::TextureSource::RGBA8(items) => items.as_slice(),
        };
        let id = self.texture[id.index];
        let (pos, size) = portion.into();
        self.render.texture_update_part(id, pos.x as _, pos.y as _, size.x as _, size.y as _, src);
    }

    fn delete_texture(&mut self, id : texture::RawTextureID) {
        let id = self.texture.remove_index(id.index).unwrap();
        self.render.delete_texture(id);
    }

    fn new_render_pass   (&mut self, texture : texture::RawTextureID, depth : Option<texture::RawTextureID>) -> render_pass::RawRenderPassID {
        let id = self.texture[texture.index];
        let depth = depth.map(|v| self.texture[v.index]);
        let miniquad_pass_id = self.render.new_render_pass(id, depth);
        render_pass::RawRenderPassID{ index: unsafe { std::mem::transmute(miniquad_pass_id) } }
    }

    fn delete_render_pass(&mut self, id: render_pass::RawRenderPassID) {
        self.render.delete_render_pass(unsafe { std::mem::transmute(id.index) });
    }

    fn new_pipeline   (&mut self, data : &pipeline::PipelineData) -> pipeline::RawPipelineID
    {
        let pipeline::PipelineData
        {
            buffer_layout,
            attributes,
            shader,
            param
        } = data;

        let miniquad_layout : Vec<miniquad::BufferLayout> = buffer_layout.iter().map(|v| v.convert()).collect();
        let miniquad_attribute : Vec<miniquad::VertexAttribute> = attributes.iter()
        .map(|v|
            miniquad::VertexAttribute
            {
                name: unsafe { std::mem::transmute::<&str, &'static str>(&v.name) },
                format: v.format.convert(),
                buffer_index: v.buffer_index.index,
                gl_pass_as_float: v.gl_pass_as_float
            }
        ).collect();

        let miniquad_shader_id = unsafe { std::mem::transmute(shader.index) };

        let pipeline = self.render.new_pipeline(&miniquad_layout,  &miniquad_attribute, miniquad_shader_id, param.convert());
        pipeline::RawPipelineID{ index: unsafe { std::mem::transmute(pipeline) } }
    }

    fn apply_pipeline (&mut self, pipeline: pipeline::RawPipelineID) {
        self.render.apply_pipeline(unsafe { std::mem::transmute(pipeline.index) });
    }

    fn delete_pipeline(&mut self, pipeline: pipeline::RawPipelineID) {
        self.render.delete_pipeline(unsafe { std::mem::transmute(pipeline.index) });
    }

    fn new_shader   (&mut self, data : &shader::ShaderData) -> Result<shader::RawShaderID, shader::ShaderError>
    {
        let shader::ShaderData { source, meta } = data;
        let (vertex, fragment) = match source
        {
            shader::ShaderSource::GLSL(shader_source_glsl) => (shader_source_glsl.vertex.as_str(), shader_source_glsl.fragment.as_str()),
            _ => unreachable!(),
        };
        let shader = self.render.new_shader(miniquad::ShaderSource::Glsl { vertex, fragment }, meta.convert());

        match shader
        {
            Ok(shader) => Ok(shader::RawShaderID{ index: unsafe { std::mem::transmute(shader) } }),
            Err(err) => Err(err.convert())
        }
    }

    fn delete_shader(&mut self, program: shader::RawShaderID) {
        self.render.delete_shader(unsafe { std::mem::transmute(program.index) });
    }

    fn apply_viewport(&mut self, viewport : Rect2P)
    {
        let (pos, size) = viewport.into();
        self.render.apply_viewport(pos.x as _, pos.y as _, size.x as _, size.y as _);
    }

    fn apply_scissor (&mut self, scissor : Rect2P) {
        let (pos, size) = scissor.into();
        self.render.apply_scissor_rect(pos.x as _, pos.y as _, size.x as _, size.y as _);
    }

    fn apply_bindings_view(&mut self, binding : bindings::BindingsView)
    {
        self.tmp_vertex.clear();
        for v in binding.vertex_buffers.iter()
        {
            self.tmp_vertex.push(unsafe { std::mem::transmute(v.index) });
        }

        self.tmp_textures.clear();
        for v in binding.images.iter()
        {
            self.tmp_textures.push(self.texture[v.index]);
        }

        self.render.apply_bindings_from_slice(
            &self.tmp_vertex,
            unsafe { std::mem::transmute(binding.index_buffer.index) },
            &self.tmp_textures,
        );
    }

    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: &[u8]) {
        self.render.apply_uniforms_from_bytes(uniform_ptr.as_ptr(), uniform_ptr.len());
    }

    fn clear(&mut self, data : render_pass::ClearData)
    {
        let render_pass::ClearData { color, depth, stencil } = data;
        let color = color.map(|c| c.convert());
        self.render.clear(color, depth, stencil);
    }

    fn begin_default_pass(&mut self, action: render_pass::PassAction) {
        self.render.begin_default_pass(action.convert());
    }

    fn begin_pass(&mut self, pass: Option<render_pass::RawRenderPassID>, action: render_pass::PassAction) {
        self.render.begin_pass(
            pass.map(|v| unsafe { std::mem::transmute(v.index) }),
            action.convert()
        );
    }

    fn end_render_pass(&mut self) {
        self.render.end_render_pass();
    }

    fn end_frame(&mut self) {
        self.render.commit_frame();
    }

    fn draw(&mut self, base_element: usize, num_elements: usize, num_instances: usize) {
        self.render.draw(base_element as _, num_elements as _, num_instances as _);
    }
}

impl ContextWindow for QuadContext
{
    fn get_clipboard(&mut self) -> Option<String> {
        miniquad::window::clipboard_get()
    }

    fn set_clipboard(&mut self, text : &str) {
        miniquad::window::clipboard_set(text);
    }

    fn dpi_scale(&mut self) -> f32 {
        miniquad::window::dpi_scale() as _
    }

    fn is_dpi_hight(&mut self) -> bool {
        miniquad::window::high_dpi()
    }

    fn quit(&mut self) {
        miniquad::window::quit();
    }

    fn request_quit(&mut self) {
        miniquad::window::request_quit();
    }

    fn get_position(&mut self) -> Point2 {
        let (x,y) = miniquad::window::get_window_position();
        point2(x as _, y as _)
    }

    fn set_position(&mut self, pos : Point2) {
        miniquad::window::set_window_position(pos.x as _,pos.y as _);
    }

    fn get_screen_size_tuple(&mut self) -> Point2 {
        let (x,y) = miniquad::window::screen_size();
        point2(x as _, y as _)
    }

    fn set_size(&mut self, size : Point2) {
        miniquad::window::set_window_size(size.x as  _, size.y as _);
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        miniquad::window::set_fullscreen(fullscreen);
    }

    fn show_keyboard(&mut self, show: bool) {
        miniquad::window::show_keyboard(show);
    }

    fn show_mouse(&mut self, show: bool) {
        miniquad::window::show_mouse(show);
    }

    fn grab_mouse(&mut self, grab: bool) {
        miniquad::window::set_cursor_grab(grab);
    }

    fn set_mouse_cursor(&mut self, cursor_icon: window::CursorIcon) {
        miniquad::window::set_mouse_cursor(cursor_icon.convert());
    }
}

//+ ContextRender

//impl ContextMultiMedia for

/*
pub trait QuadRunner
{
    fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static;
}

impl QuadRunner for MultiMediaParam
{
    fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static
    {
        miniquad::start(self.clone().convert(), move ||
        {
            let ctx = Box::new(QuadContext{ render: miniquad::window::new_rendering_backend(), texture: ___(), tmp_vertex: ___(), tmp_textures: ___() });
            unsafe { context::set_context(Some(Context::new(ctx, self))); }
            Box::new(super::QuadState { state : state() })
        }
        );
    }
}
*/