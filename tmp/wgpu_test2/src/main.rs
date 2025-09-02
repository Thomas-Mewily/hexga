#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use hexga_engine::prelude::*;
use hexga::prelude::*;

#[derive(Default)]
pub struct MyApp
{

}

impl App for MyApp
{
    type UserEvent = ();

    fn draw(&mut self) {
        
    }
}

fn main() 
{
    //println!("Hello, world!");
    MyApp::___().run().unwrap();
    //println!("Goodbye, world!");
}
