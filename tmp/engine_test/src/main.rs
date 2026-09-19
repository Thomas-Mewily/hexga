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
    println!("Hello, world!");
}
