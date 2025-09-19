use hexga_engine::prelude::*;

#[derive(Default)]
pub struct MyApp
{

}


impl App for MyApp
{
    type CustomEvent=();
}

fn main() 
{
    println!("Hello");
    MyApp::default().run().unwrap();
    println!("world!");
}
