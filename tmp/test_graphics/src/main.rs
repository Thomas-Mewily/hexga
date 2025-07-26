#![allow(unused_imports)]

use hexga_engine::prelude::*;

struct MyApp;

impl App for MyApp
{
    fn update(&mut self)
    {
        println!("update");
    }
}

fn main()
{
    println!("hello world");
    MyApp.run();
}