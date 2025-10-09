#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use hexga_engine::prelude::*;

struct MyApp;

impl Application for MyApp
{
    fn update(&mut self)
    {
        if KeyCode::Space.is_pressed()
        {
            dbg!(Clipboard.get());
        }
    }
}

fn main()
{
    println!("Hello, world!");

    let _ = MyApp.run();

    println!("Goodbye");
}