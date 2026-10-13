#![allow(unused)]
use hexga_engine::{image::ImageBaseOf, prelude::*};

fn cast<From,To>(from: From) -> To where From: CastInto<To>
{
    from.cast_into()
}

fn cast_range<From,To>(from: From) -> To where From: CastRangeInto<To>
{
    from.cast_range_into()
}


fn main()
{
    dbg!(i8::cast_range_from(-1f32));
    dbg!(i8::cast_range_from(-0.5f32));
    dbg!(i8::cast_range_from(0f32));
    dbg!(i8::cast_range_from(0.5f32));
    dbg!(i8::cast_range_from(1f32));
    //vec2(1., 2.).
    println!("Hello, world!");
}
