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


fn img()
{
    //let i = Image::from_fn_coef((4, 4), |v| hsl(v.x, v.y, 1.).to_rgba_of::<float>());
    let i = ImageOf::<ColorU16>::from_fn_coef((1024, 10), |v| rgb(v.x, v.y, 0.0).to_u16_range());
    let original_pixels = i.pixels().to_owned();

    i.save("./test").unwrap();
    let i = ImageOf::<ColorU16>::load("./test.png").unwrap();
    let open_pixels = i.pixels().to_owned();

    assert_eq!(original_pixels, open_pixels);

    dbg!(i);
}

fn img_test()
{
    let i = Image::from_fn_coef((1024, 1024),
        |v|
        if (v - 0.5.splat2()).length() <= 0.5
        {
            ColorU8::WHITE
        }
        else
        {
            ColorU8::BLACK
        }
    ).save("./image_test.png").unwrap();
}

use std::ops::*;

pub struct DamageOf<T>
{
    physic: T,
    magic: T,
    melee: T,
}
hexga::math::impl_fixed_array!(DamageOf,3);

pub type Damage = DamageOf<float>;

fn main()
{
    dbg!(&Damage::ZERO);
}
