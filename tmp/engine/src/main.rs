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

            //Window.set_x(960/2);
            dbg!(Clipboard.get());
            //Window.set_size(vec2i(64, 128));
            //Window.set_pos(vec2i(256, 100));
        }
    }
}

fn main()
{
    println!("Hello, world!");

    let _ = MyApp.run();

    println!("Goodbye");
}