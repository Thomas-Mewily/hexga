#![allow(unused_imports)]
use hexga_core::prelude::*;
use hexga_engine::prelude::*;


#[derive(Default)]
pub struct MyApp
{

}
impl MyApp
{
    pub fn new() -> Self { ___() }
}

impl App for MyApp
{

}

fn main() 
{
    println!("Hello");
    MyApp::new().run();
    println!("Good bye");
}
