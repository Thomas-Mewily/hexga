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
use hexga_render::{Conf, Event, EventLoop};
use hexga_undo_redo::*; //prelude::*;


pub struct Ctx;
impl EventLoop for Ctx
{
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        
    }

    fn handle_event(&mut self, event : &Event) -> bool {
        println!("{:?}", event);
        true
    }
}

fn main() 
{
    for _ in 0..10 { println!(); }
    Conf::new().title("hello").run(|| Ctx);
}
