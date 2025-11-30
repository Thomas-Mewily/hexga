#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;
use hexga_engine::mem::Zeroable;

#[repr(C)]
#[derive(Zeroable, Debug)]
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
