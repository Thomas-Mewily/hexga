#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::HashMap;

use hexga_math::prelude::*;


/*
use hexga_generational::gen_vec::GenVec;
use hexga_math::*;
use have_len::*;
*/

fn main() 
{

    let g = point2(2, 4).to_grid(|p| p.x + 10 * p.y);

    for y in (0..g.size_y()).rev()
    {
        for x in 0..g.size_x()
        {
            print!("{:2} ", g[point2(x,y)]);
        }
        println!();
    }

    //g.subslice(g.rect().crop())


    /* 
    let mut v = Vec2::new(1.,2.);

    let a = 10.degree();
    let a = [1,2,3].radian();

    let x = 10.hundred() + 1.billion();
    let y = 1*i32::HUNDRED + 1*i32::BILLION;

    let v = vec2(1.,2.).degree();

    let x = 1.0.kilo().giga().femto().degree();


    let mut ve = GenVec::new();
    let l = ve.len();
    let l = ve.is_empty();
    ve.insert("ok");

    for v in ve
    {

    }

    let mut h = HashMap::new();
    h.insert("ok", 42);

    for v in &h
    {

    }

    
    let v =  Color::rgb(1., 0., 1.).to_hsla();
    */

    /* 
    let mut grid = Grid2::from_fn(point2(2, 3), |p| p.x + p.y);
    dbg!(grid);
    */
    println!("Hello, world3!");
}
