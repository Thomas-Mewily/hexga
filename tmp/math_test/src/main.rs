#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


use hexga::prelude::*;

fn dbg_mix<S,F>(src: S, dest:S, coef:F) where S:Mix<F> + Copy + Debug, F:Float
{
    dbg!(&src);
    dbg!(&dest);
    dbg!(src.mix(dest, coef));
}

fn x()
{
let v = vec2(0., 1.);

    let size = point2(20, 40);
    let mut g = size.to_grid(|v| -2);
    let mut g2 = size.to_grid(|v| v.sum_axis());
    let m = g.max(g2);
    println!("{}", m);


    dbg!(v);
    dbg!(m);
    dbg!(Mat4::from_fn(|v| v.sum_axis() as float * 5.));

    let x = 127u8.to_float_range();

    for coef in (0.0..=1.).sample(10)
    {
        dbg!(coef);
        dbg!(false.mix(true, coef));
    }
    println!();
    println!();


    let x = 10.to_f32();
    let x = [1i32,2,3].to_f32();

    let a = max([1,5], [2,3]);
    dbg!(a);
    println!("hello world");
    dbg_mix(0., 1., 1.);
    dbg_mix(0., 1., 0.);
    dbg_mix(0., 1., 0.5);
}

fn main()
{
    //let i = Image::from_fn_coef((4, 4), |v| hsl(v.x, v.y, 1.).to_rgba_of::<float>());
    let i = Image::from_fn_coef((4, 4), |v| rgb(v.x, v.y, 1.0).to_u8_range());

    i.save_to_disk("./test");

    dbg!(i);
}
