#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use hexga::prelude::*;
use std::ops::*;

pub mod context;
use context::*;

pub mod texture;
use texture::*;

pub mod app;
use app::*;


#[derive(Default)]
pub struct MyApp
{

}

impl App for MyApp
{
    type UserEvent = ();
}

fn main() 
{
    println!("Hello, world!");
    MyApp::___().run().unwrap();
    println!("Goodbye, world!");
}

// Based on learn-wgpu