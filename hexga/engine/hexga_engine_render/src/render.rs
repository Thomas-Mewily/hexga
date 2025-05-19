use crate::*;

pub trait LoopDraw
{
    fn draw(&mut self);
}
impl LoopDraw for () { fn draw(&mut self) {} }

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
    fn texture_set_wrap        (&mut self, id : Texture, wrap : (TextureWrap, TextureWrap));
    fn texture_generate_mipmaps(&mut self, id : Texture);
    fn texture_read_pixels     (&mut self, id : Texture, source : &mut TextureSource);
    fn texture_update_portion  (&mut self, id : Texture, pos : (u32, u32), size : (u32, u32), data : &[u8]);
    fn delete_texture          (&mut self, id : Texture);

    fn new_render_pass   (&mut self, texture : Texture, depth : Option<Texture>) -> RenderPass;
    fn delete_render_pass(&mut self, id: RenderPass);

    fn new_pipeline   (&mut self, data : &PipelineData) -> Pipeline;
    fn apply_pipeline (&mut self, pipeline: Pipeline);
    fn delete_pipeline(&mut self, pipeline: Pipeline);

    fn new_shader   (&mut self, data : &ShaderData) -> Shader;
    fn delete_shader(&mut self, program: Shader);

    fn apply_viewport(&mut self, pos : (u32, u32), size : (u32, u32));
    fn apply_scissor (&mut self, pos : (u32, u32), size : (u32, u32));

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
