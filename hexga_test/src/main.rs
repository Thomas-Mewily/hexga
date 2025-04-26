#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::{any::{Any, TypeId}, collections::HashMap, fmt::Display, hint::black_box, marker::PhantomData, num::Wrapping};
use criterion::{BenchmarkId, Criterion};

//use hexga_graphics::Image;
use hexga_math::prelude::*;
use hexga_generational::{gen_vec::{GenVecOf, Generation}, prelude::*};
use hexga_undo_redo::prelude::*;

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

    let mut u = CommandStackSequenceFlatten::new();

    let mut v = Vec::new();

    u.execute(|s|
        {
            v.push_action(42, s);
            v.push_action(50, s);
        }
    );

    //v.undo(&mut u);

    dbg!(&u);
    dbg!(&v);

    /* 
    u.begin();
    
    u.end();*/


    /* 
    let g = Grid2::from_vec(2.splat2(), vec!["toto", "f", "bar", "x"]).unwrap();
    println!("{}", g.view().format().with_separator(" "));

    let custom_serde : HashMap::<TypeId, fn(&dyn Any)>;
*/
    /* 
    let g = GridParam2::from_fn_with_param(point2(2, 4), "toto".to_owned(), |p| p.x + 10 * p.y);
    let h = g.subgrid(rect2p(1, 1, 1, 3));
    
    let x = g.get(one());

    dbg_mat(g.grid());
    dbg_mat(h.grid());

    dbg!(h.param());
    dbg!(g.param());

    let mut size = 2;
    */
    

    /*
    for _ in 0..16
    {
        println!("generating...");
        size *= 2;
        let img = Image::from_fn(point2(size, size), |p| 
        {
            if (p.x / 8 + p.y / 8) % 2 == 0 
            {
                Color::WHITE
            }else{
                Color::BLACK
            }.with_r(p.x as float / size as float).with_g(p.y as float / size as float).to_rgba_u8()
        });
        println!("saving...");
        let path = format!("img/hello_{size}_px.png");
        //img.tmp_write_to_png_bytes_inside(&path);
        println!("done generating {path}");
        println!();

    }*/


    /* 
    let mut criterion: Criterion = Criterion::default().nresamples(20).sample_size(15);
    let mut group = criterion.benchmark_group(format!("new grid"));

    for size in [16,32,64,128,256,512,1024,2048,4096] //,256]
    {
        group.bench_with_input(BenchmarkId::new("sequencial", size), &size, 
            |b, size| 
            b.iter(|| { black_box(Grid2::from_fn((*size).splat2(), |p| p.sum_axis())); })
        );

        group.bench_with_input(BenchmarkId::new("parallel", size), &size, 
        |b, size| 
            b.iter(|| { black_box(Grid2::from_fn_par((*size).splat2(), |p| p.sum_axis())); })
        );
    }
    group.finish();
    */

    /* 
    let x = 4.0f32;
    let y : usize = usize::cast_from(x);
    */
    //dbg_mat(img.grid());

    

    


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
