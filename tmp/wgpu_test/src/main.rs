#![allow(dead_code)]
#![allow(unused_imports)]

use hexga::prelude::*;
use std::ops::*;
use std::sync::Arc;
use winit::window::Window;


mod ctx;
pub use ctx::*;

mod app;
pub use app::*;

mod vertex;
pub use vertex::*;


impl App for ()
{
    fn draw(&self) 
    {
        /*
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = Ctx.encoder.as_mut().unwrap();
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_pipeline(&self.render_pipeline);
        // 消费存放的 vertex_buffer
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // 消费存放的 vertex_index_buffer
        rpass.set_index_buffer(
            self.vertex_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        ); // 1.
            // 调用draw_indexed，传入对应数量的顶点数量
        rpass.draw_indexed(0..VERTEX_INDEX_LIST.len() as u32, 0, 0..1);
        // 顶点有原来的固定3个顶点，调整为根据 VERTEX_LIST 动态来计算
        rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        */
        
    }
}

fn main() 
{
    println!("Hello, world!");
    ().run();
}
