#![allow(unused)]
use hexga_engine::prelude::*;

pub struct MyApp;

impl App for MyApp
{

}

fn main()
{
    MyApp::run(|_| MyApp);
}
