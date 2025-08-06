#![allow(unused_imports)]

use hexga_core::prelude::DefaultIsTripleUnderscore;
use hexga_engine::prelude::*;

#[derive(Default)]
struct MyApp
{
    windows : Vec<Window>,
}

impl App for MyApp
{
    fn update(&mut self)
    {
        /*
        if KeyCode::Space.is_press()
        {

        }
        */

        self.windows.push(Window::new(___()).unwrap());

        for k in Input.keys_just_pressed()
        {
            dbg!(k);
        }
        // for k in KeyCode.just_pressed
        //println!("update");
    }

    fn handle_event(&mut self, event : AppEvent<()>) {
        dbg!(event);
        println!("nb window : {}", self.windows.len());
    }
}

fn main()
{
    println!("hello world");
    MyApp::___().run();
}

/*
cargo run --release --bin test_graphics
*/