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
use hexga_engine::{Event, EventLoop, MultiMediaConfig, Pen, WindowConfig};
use hexga_undo_redo::*; //prelude::*;

pub struct TestCtx;
impl EventLoop for TestCtx
{
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        Pen.begin_pass();

        Pen.geometry([Gpu], indexs)

        Pen.end_pass();
        Pen.commit_frame();
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
