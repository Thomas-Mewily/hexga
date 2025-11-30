#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

#[repr(C)]
#[derive(BitsZero, Debug)]
struct Bar
{
    x: i32,
    y: i32,
}


fn main()
{
    let b = Bar::zeroed();
    dbg!(b);
}
