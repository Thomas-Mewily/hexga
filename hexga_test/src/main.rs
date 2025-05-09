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
use hexga_undo_redo::*; //prelude::*;


fn main() 
{
    for _ in 0..10 { println!(); }

    let size = point2(2, 3);
    let grid = Grid2::from_fn(size, |p|  p.x + 10 * p.y);
    let smaller_size = point2(1, 2);

    let top_right_grid_size = point2(1, 2);
    let offset = Vector2::ONE;
    let top_right_grid = Grid2::from_fn(smaller_size, |p|  { let p = p + offset; p.x + 10 * p.y });
    
    dbg!(rect2p(0, 0, 2, 3).is_inside(point2(1, 1)));

    let top_right_grid_from_view = grid.subgrid(top_right_grid_size.to_rect().moved_by(offset));
    assert_eq!(top_right_grid, top_right_grid_from_view);
    
    println!("Hello, world3!");
}
