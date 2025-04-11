#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::{collections::HashMap, fmt::Display};

use hexga_math::prelude::*;

/*
use hexga_generational::gen_vec::GenVec;
use hexga_math::*;
use have_len::*;
*/

fn dbg_mat<T>(g : &Grid2<T>) where T : Display
{
    for y in (0..g.size_y()).rev()
    {
        for x in 0..g.size_x()
        {
            print!("{:2} ", g[point2(x,y)]);
        }
        println!();
    }
    println!();
}

fn main() 
{
    let g = GridParam2::from_fn_with_param(point2(2, 4), |p| p.x + 10 * p.y, "toto".to_owned());
    let h = g.subgrid(rect2p(1, 1, 1, 3));
    
    dbg_mat(g.grid());
    dbg_mat(h.grid());

    dbg!(h.param());
    dbg!(g.param());


    /* 
    let g = point2(2, 4).to_grid(|p| p.x + 10 * p.y);

    dbg_mat(&g);
    dbg_mat(&g.crop_margin(zero(), one()));

    dbg_mat(&g.crop_margin(one(), zero()));
    dbg!(rect2p(5,5,10,10).crop_margin_uncheck(100.splat2(), 100.splat2()));
*/

/* 
    let a = rect2p(5,5,10,10);
    let b = rect2p(5,5,10,10).crop_margin(-1.splat2(), zero());
    dbg!(a);
    dbg!(b);
    assert_eq!(a, b);

*/


    


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
