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


    fn update(&mut self) 
    {
        
    }

    fn draw(&mut self) 
    {
        
        //Angle::sample(20).map(|a| a.to_vec2_normalized()).
        //let vertex = 
    }
}

fn main() 
{
    //println!("Hello, world!");
    MyApp::___().run().unwrap();
    //println!("Goodbye, world!");
}
