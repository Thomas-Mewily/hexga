#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::{any::{Any, TypeId}, collections::HashMap, fmt::Display, hint::black_box, marker::PhantomData, num::Wrapping};
use criterion::{BenchmarkId, Criterion};

//use hexga_graphics::Image;
use hexga_core::prelude::*;
use hexga_math::prelude::*;
use hexga_generational::{gen_vec::{GenVecOf, Generation}, prelude::*};
use hexga_engine::{Event, EventLoop, GpuVec2, GpuVertex, MultiMediaConfig, Pen, WindowConfig};
use hexga_undo_redo::*; //prelude::*;

pub struct TestCtx;
impl EventLoop for TestCtx
{
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        Pen.begin_draw();
        Pen.begin_pass();

        Pen.set_pos(zero());

        Pen.color(Color::RED).pos2(vec2(-1., 1.)).down()
           .color(Color::BLUE).pos2(vec2(1., 1.)).down()
           .color(Color::GREEN).pos2(vec2(0.0, -1.)).down()
           .make_triangle();

        

        /* 
        let Rect2 { pos, size } = Rect2::new_centered(zero(), 1.8.splat2());
        let pos = GpuVec2::from(pos).with_z(0.);
        let size = GpuVec2::from(size);
        let color = Color::WHITE;

        let vertex =
        [
            GpuVertex::new().with_pos(pos).with_color(color),
            GpuVertex::new().with_pos(pos.moved_x(size.x)).with_color(color),
            GpuVertex::new().with_pos(pos.moved_x(size.x).moved_y(size.y)).with_color(color),
            GpuVertex::new().with_pos(pos.moved_y(size.y)).with_color(color),
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        Pen.static_geometry(vertex, indices);
        Pen.draw_triangle_test();
        */

        Pen.end_pass();
        Pen.end_draw();
    }

    fn handle_event(&mut self, event : &Event) -> bool {
        println!("{:?}", event);
        true
    }
}

fn main() 
{
    for _ in 0..10 { println!(); }
    MultiMediaConfig::new()
        .with_window_config(WindowConfig::new().title("hello"))
        .run(|| TestCtx);
}
