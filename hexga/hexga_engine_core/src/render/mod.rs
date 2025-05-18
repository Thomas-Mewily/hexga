//! mainly based on miniquad
use crate::*;

mod buffer;
use buffer::*;

mod untyped_slice;
use untyped_slice::*;

mod render_pass;
use render_pass::*;

mod vertex;
use vertex::*;

mod shader;
use shader::*;

mod pipeline;
use pipeline::*;

mod texture;
use texture::*;

mod bindings;
use bindings::*;

pub mod prelude
{
    //pub use super::buffer::Buffer;
    pub use super::texture::Texture;
}


// #proper_error
// miniquad render wrapper
pub trait Render
{
    fn new_buffer   (&mut self, data: BufferData, source : BufferSource) -> Option<Buffer>;
    fn buffer_update(&mut self, dest: Buffer, source: &BufferSource) -> Result<(), ()>;
    fn delete_buffer(&mut self, id : Buffer);

    fn new_texture             (&mut self, data : &TextureData) -> Texture;
    fn texture_update          (&mut self, dest : Texture, source : &TextureData);
    fn texture_set_mag_filter  (&mut self, id : Texture, filter: FilterMode);
    fn texture_set_wrap        (&mut self, id : Texture, wrap : TextureWrap2);
    fn texture_generate_mipmaps(&mut self, id : Texture);
    fn texture_read_pixels     (&mut self, id : Texture, source : &mut TextureSource);
    fn texture_update_view     (&mut self, id : Texture, view : &mut TextureView);
    fn delete_texture          (&mut self, id : Texture);

    fn new_render_pass   (&mut self, texture : Texture, depth : Option<Texture>) -> RenderPass;
    fn delete_render_pass(&mut self, id: RenderPass);

    fn new_pipeline   (&mut self, data : &PipelineData) -> Pipeline;
    fn apply_pipeline (&mut self, pipeline: Pipeline);
    fn delete_pipeline(&mut self, pipeline: Pipeline);

    fn new_shader   (&mut self, data : &ShaderData) -> Shader;
    fn delete_shader(&mut self, program: Shader);

    fn apply_viewport(&mut self, rect : Rect2P);
    fn apply_scissor(&mut self, rect : Rect2P);

    fn apply_bindings_view(&mut self, binding : BindingsView);
    fn apply_bindings(&mut self, binding : &Bindings) { self.apply_bindings_view(binding.view()); }

    fn apply_uniforms(&mut self, uniforms: UniformsSource) { self.apply_uniforms_from_bytes(uniforms.as_slice()); }
    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: &[u8]);

    fn clear(&mut self, data : ClearData);

    fn begin_default_pass(&mut self, action: PassAction);
    fn begin_pass(&mut self, pass: Option<RenderPass>, action: PassAction);
    fn end_render_pass(&mut self);

    fn end_frame(&mut self);
    fn draw(&mut self, base_element: usize, num_elements: usize, num_instances: usize);
}



