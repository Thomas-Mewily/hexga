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
        //if Keyboard.keys().is_down(code)
        //dbg!(KeyCode::Space.is_down());
        //dbg!(Keyboard.keys_repeated().is_down(KeyCode::Space));
    }
}

fn main()
{
    println!("Hello, world!");

    let _ = MyApp.run();

    println!("Goodbye");
}