use hexga_engine::prelude::*;

#[derive(Default)]
pub struct MyApp
{

}


impl Application for MyApp
{

}

fn main() 
{
    println!("Hello");
    MyApp::default().run().unwrap();
    println!("world!");
}
