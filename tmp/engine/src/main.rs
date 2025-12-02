#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

/*
#[repr(C)]
#[derive(BitZero, Debug)]
struct Bar
{
    x: i32,
    y: i32,
}
*/

struct MyApp
{

}

impl Application for MyApp
{

}

fn main()
{
    MyApp::run(|| MyApp{});
    //let b = Bar::zeroed();
    //dbg!(b);
}
