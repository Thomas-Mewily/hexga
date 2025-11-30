#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

#[repr(C)]
#[derive(Foo)]
struct Bar
{
    x: i32,
    y: i32,
}


fn main()
{
    Bar::foo();
}
