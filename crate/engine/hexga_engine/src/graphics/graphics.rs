use super::*;

#[derive(Debug)]
pub struct Graphics
{
    pub(crate) immediate: ImmediateRenderBuilder,
    pub(crate) immediate_mesh: Option<Mesh>,

    //pub(crate) camera_buffer: GpuVec<Camera>,
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) texture_bind_group: wgpu::BindGroupLayout,

    //pub(crate) binding: GpuBinding,
    //pub(crate) render: GpuRender,

    //pub(crate) immediate_mesh: Option<Mesh>,
    //pub(crate) background_color : Option<Color>,
    //pub(crate) white_pixel: Option<Texture>,
    pub(crate) pipeline: RenderPipeline,
    pub(crate) white_pixel: Option<Texture>,
}

/*
pub trait IGraphics
{
    //fn background_color(&mut self) ->
}*/

impl Graphics
{
    pub(crate) fn new(surface: &GpuSurface, size: Vec2I) -> Self
    {
        let surface_caps = surface.wgpu.get_capabilities(&Gpu.adapter());
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.x as _,
            height: size.y as _,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let shader = Gpu.device().create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = Gpu.device().create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = Gpu.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::wgpu_vertex_description()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(VertexIndex::GPU_INDEX_FORMAT),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // Useful for optimizing shader compilation on Android
            cache: None,
            multiview: None,
        });

        Graphics {
            pipeline,
            immediate: ___(),
            immediate_mesh: None,
            white_pixel: None,
            camera_buffer: todo!(),
            camera_bind_group: todo!(),
            texture_bind_group: todo!(),
        }
    }

    pub(crate) fn begin_draw(&mut self) {}

    pub(crate) fn end_draw(&mut self)
    {
        todo!()

        /*
        if self.white_pixel.is_none()
        {
            self.white_pixel = Some(Texture::from(Image::sized_one(ColorU8::WHITE)));
        }

        let Ok(mut window) = WINDOW.try_get_mut()
        else
        {
            return;
        };
        let Some(surface) = window.surface()
        else
        {
            return;
        };

        let max_scissor = surface.size().to_rect();
        let max_viewport = max_scissor.cast_into();

        let surface = surface.surface();
        let surface_texture = match surface.wgpu.get_current_texture()
        {
            Ok(s) => s,
            Err(_) => return,
        };



        let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Todo: expose it in a field
        let background_color = Some(Color::WHITE);

        let mut encoder = Gpu.device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(background_color.unwrap_or_default().convert()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.camera_bind_group, &[]);

            if !self.immediate.big_mesh.is_empty()
            {
                match self.immediate_mesh.as_mut()
                {
                    Some(m) => self.immediate.big_mesh.build_in(m),
                    None =>
                    {
                        self.immediate_mesh = Some(self.immediate.big_mesh.build());
                    },
                };
                let Mesh { vertices, indices } = self.immediate_mesh.as_ref().unwrap();




                for dc in self.immediate.draw_call.iter()
                {
                    //dbg!(&dc);

                    let Viewport { area: viewport, depth } = dc.viewport;
                    let (viewport_min_depth, viewport_max_depth) = (dc.viewport.depth.start, dc.viewport.depth.end);
                    let mut scissor = dc.scissor;

                    viewport = max_viewport.intersect_or_empty(viewport);
                    scissor = max_scissor.intersect_or_empty(scissor);



                    if viewport.is_empty() || scissor.size.is_empty()
                    {
                        continue;
                    }

                    // Todo: use:
                    //self.camera_buffer.reset() // + remove some constraint
                    let camera = &[dc.param.camera.matrix()];
                    Gpu.queue().write_buffer(&self.camera_buffer, 0, unsafe { bit::try_transmute_slice_unchecked(camera) }.unwrap() );

                    /*
                    let viewport : Rect2i = viewport.cast_into();
                    let scissor : Rect2i = scissor.cast_into();
                    dbg!(viewport);

                    if viewport.width().is_zero() { continue; }
                    if scissor.width().is_zero() { continue; }
                    */


                    rpass.set_viewport(viewport.pos.x as _, viewport.pos.y as _, viewport.size.x as _, viewport.size.y as _, viewport_min_depth, viewport_max_depth);
                    rpass.set_scissor_rect(scissor.pos.x as _, scissor.pos.y as _, scissor.size.x as _, scissor.size.y as _);

                    let texture_index = 1;
                    let offset = &[];
                    rpass.set_bind_group(texture_index, &dc.texture.shared.bind_group, offset);

                    /*
                    match &dc.texture
                    {
                        DrawTexture::None =>
                        {
                            rpass.set_bind_group(texture_index, &self.white_pixel.as_ref().unwrap().shared.bind_group, offset)
                        },
                        DrawTexture::Texture(texture) =>
                        {
                            rpass.set_bind_group(texture_index, &texture.shared.bind_group, offset)
                        },
                        DrawTexture::Asset(asset) => match asset.get_or_placeholder()
                        {
                            Some(texture) => rpass.set_bind_group(1, &texture.shared.bind_group, offset),
                            None => rpass.set_bind_group(texture_index, &self.white_pixel.as_ref().unwrap().shared.bind_group, offset),
                        },
                    };
                    */
                    //let texture = dc.texture.as_ref().unwrap_or(self.white_pixel.as_ref().unwrap());
                    //rpass.set_bind_group(1, bindgroup, &[]);

                    match &dc.geometry
                    {
                        DrawGeometry::Immediate(im) =>
                        {
                            if im.is_empty() { continue; }
                            let (vertices_begin, vertices_len) = (im.vertices_begin, im.vertices_len);
                            let vertices_end = im.vertices_begin+im.vertices_len;

                            let (indices_begin, indices_len) = (im.indices_begin, im.indices_len);
                            let indices_end = im.indices_begin+im.indices_len;

                            rpass.set_vertex_buffer(0, vertices.wgpu_slice(vertices_begin..vertices_end));
                            rpass.set_index_buffer(indices.wgpu_slice(indices_begin..indices_end), VertexIndex::GPU_INDEX_FORMAT);
                            //rpass.draw_indexed(0 ..(indices_len as _), 0, 0..1);
                            // Indice are relative to global big mesh, not relative to the current vertices slice passed to wgpu, hence the -(vertices_begin as i32)
                            rpass.draw_indexed(0 ..(indices_len as _), -(vertices_begin as i32), 0..1);
                        },
                    }
                }
            }
        }
        */
    }
}

impl BuilderMesh for Graphics
{
    fn geometry(&mut self, vertex: impl IntoIterator<Item = Vertex>, index: impl IntoIterator<Item = VertexIndex>) { self.immediate.geometry(vertex, index); }
}

impl GetMatrix<float, 4, 4> for Graphics
{
    fn matrix(&self) -> Matrix<float, 4, 4> { self.immediate.matrix() }
}
impl SetMatrix<float, 4, 4> for Graphics
{
    fn set_matrix(&mut self, matrix: Matrix<float, 4, 4>) -> &mut Self
    {
        self.immediate.set_matrix(matrix);
        self
    }
}

impl GetCamera for Graphics
{
    fn have_depth(&self) -> bool { self.immediate.have_depth() }
}
impl SetCamera for Graphics
{
    fn set_camera(&mut self, camera: Camera) -> &mut Self
    {
        self.immediate.set_camera(camera);
        self
    }
}

impl GetPosition for Graphics
{
    fn pos(&self) -> Vec3 { self.immediate.pos() }
}
impl SetPosition for Graphics
{
    fn set_pos(&mut self, pos: Vec3) -> &mut Self
    {
        self.immediate.set_pos(pos);
        self
    }
}
impl GetScale for Graphics
{
    fn scale(&self) -> Vec3 { self.immediate.scale() }
}
impl SetScale for Graphics
{
    fn set_scale(&mut self, scale: Vec3) -> &mut Self
    {
        self.immediate.set_scale(scale);
        self
    }
}
impl RotateX for Graphics
{
    fn rotate_x(&mut self, angle: Angle) -> &mut Self
    {
        self.immediate.rotate_x(angle);
        self
    }
}
impl RotateY for Graphics
{
    fn rotate_y(&mut self, angle: Angle) -> &mut Self
    {
        self.immediate.rotate_y(angle);
        self
    }
}
impl RotateZ for Graphics
{
    fn rotate_z(&mut self, angle: Angle) -> &mut Self
    {
        self.immediate.rotate_z(angle);
        self
    }
}

impl DrawParamAttribute for Graphics
{
    fn viewport(&self) -> Viewport { self.immediate.viewport() }
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self
    {
        self.immediate.set_viewport(viewport);
        self
    }

    fn scissor(&self) -> Rect2i { self.immediate.scissor() }

    fn set_scissor(&mut self, scissor: Rect2i) -> &mut Self
    {
        self.immediate.set_scissor(scissor);
        self
    }

    fn texture(&self) -> Option<Texture> { self.immediate.texture() }

    fn set_texture(&mut self, texture: impl Into<Option<Texture>>) -> &mut Self
    {
        self.immediate.set_texture(texture);
        self
    }
}

pub(crate) trait ExternLibConvert<Output>
{
    fn convert(self) -> Output;
}
impl<T> ExternLibConvert<wgpu::Color> for T
where
    T: IColor,
    f64: CastRangeFrom<<T as IColor>::Component>,
{
    fn convert(self) -> wgpu::Color
    {
        let RgbaOf { r, g, b, a } = IColor::to_rgba_of::<f64>(self);
        wgpu::Color { r, g, b, a }
    }
}
