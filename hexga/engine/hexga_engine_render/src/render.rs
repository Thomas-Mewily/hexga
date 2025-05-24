use crate::*;

/* 
pub trait LoopDraw
{
    fn draw(&mut self);
}
impl LoopDraw for () { fn draw(&mut self) {} }
*/

// #proper_error
// miniquad render wrapper
pub trait RenderBackend
{
    fn new_buffer   (&mut self, data: BufferData, source : BufferSource) -> RawBufferID;
    fn buffer_update(&mut self, dest: RawBufferID, source: &BufferSource);
    fn delete_buffer(&mut self, id : RawBufferID);

    fn new_texture             (&mut self, data : &TextureData) -> RawTextureID;
    fn texture_update          (&mut self, dest : RawTextureID, data : &TextureData);
    fn texture_set_mag_filter  (&mut self, id : RawTextureID, filter: FilterMode);
    fn texture_set_wrap        (&mut self, id : RawTextureID, wrap : (TextureWrap, TextureWrap));
    fn texture_generate_mipmaps(&mut self, id : RawTextureID);
    fn texture_read_pixels     (&mut self, id : RawTextureID, dest : &mut TextureSource);
    fn texture_update_portion  (&mut self, id : RawTextureID, rect : Rect2P, source : &texture::TextureSource);
    fn delete_texture          (&mut self, id : RawTextureID);

    fn new_render_pass   (&mut self, texture : RawTextureID, depth : Option<RawTextureID>) -> RenderPassID;
    fn delete_render_pass(&mut self, id: RenderPassID);

    fn new_pipeline   (&mut self, data : &PipelineData) -> RawPipelineID;
    fn apply_pipeline (&mut self, pipeline: RawPipelineID);
    fn delete_pipeline(&mut self, pipeline: RawPipelineID);

    fn new_shader   (&mut self, data : &ShaderData) -> Result<RawShaderID, ShaderError>;
    fn delete_shader(&mut self, program: RawShaderID);

    fn apply_viewport(&mut self, viewport : Rect2P);
    fn apply_scissor (&mut self, scissor : Rect2P);

    fn apply_bindings_view(&mut self, binding : BindingsView);
    fn apply_bindings(&mut self, binding : &Bindings) { self.apply_bindings_view(binding.view()); }

    fn apply_uniforms(&mut self, uniforms: UniformsSource) { self.apply_uniforms_from_bytes(&*uniforms.source); }
    fn apply_uniforms_from_bytes(&mut self, uniform_ptr: &[u8]);

    fn clear(&mut self, data : ClearData);

    fn begin_default_pass(&mut self, action: PassAction);
    fn begin_pass(&mut self, pass: Option<RenderPassID>, action: PassAction);
    fn end_render_pass(&mut self);

    fn end_frame(&mut self);
    fn draw(&mut self, base_element: usize, num_elements: usize, num_instances: usize);
}


impl RenderBackend for ()
{
    fn new_buffer   (&mut self, _: BufferData, _ : BufferSource) -> RawBufferID { RawBufferID::default() }
    fn buffer_update(&mut self, _: RawBufferID, _: &BufferSource) {}
    fn delete_buffer(&mut self,_id : RawBufferID) {}

    fn new_texture             (&mut self, _ : &TextureData) -> RawTextureID { RawTextureID::default() }
    fn texture_update          (&mut self, _ : RawTextureID, _ : &TextureData) {}
    fn texture_set_mag_filter  (&mut self, _ : RawTextureID, _: FilterMode) {}
    fn texture_set_wrap        (&mut self, _ : RawTextureID, _ : (TextureWrap, TextureWrap)) {}
    fn texture_generate_mipmaps(&mut self, _ : RawTextureID) {}
    fn texture_read_pixels     (&mut self, _ : RawTextureID, _ : &mut TextureSource) {}
    fn texture_update_portion  (&mut self, _ : RawTextureID, _ : Rect2P, _ : &texture::TextureSource) {}
    fn delete_texture          (&mut self, _ : RawTextureID) {}
    
    fn new_render_pass   (&mut self, _ : RawTextureID, _ : Option<RawTextureID>) -> RenderPassID { RenderPassID::default() }
    fn delete_render_pass(&mut self, _: RenderPassID) {}
    fn new_pipeline   (&mut self, _ : &PipelineData) -> RawPipelineID { RawPipelineID::default() }
    fn apply_pipeline (&mut self, _: RawPipelineID) {}
    fn delete_pipeline(&mut self, _: RawPipelineID) {}
    fn new_shader   (&mut self, _ : &ShaderData) -> Result<RawShaderID, ShaderError> { Ok(RawShaderID::default()) }
    fn delete_shader(&mut self, _: RawShaderID) {}
    fn apply_viewport(&mut self, _ : Rect2P) {}
    fn apply_scissor (&mut self, _ : Rect2P) {}
    fn apply_bindings_view(&mut self, _ : BindingsView) {}
    fn apply_uniforms_from_bytes(&mut self, _: &[u8]) {}
    fn clear(&mut self, _ : ClearData) {}
    fn begin_default_pass(&mut self, _: PassAction) {}
    fn begin_pass(&mut self, _: Option<RenderPassID>, _: PassAction) {}
    fn end_render_pass(&mut self) {}
    fn end_frame(&mut self) {}
    fn draw(&mut self, _: usize, _: usize, _: usize) {}
}