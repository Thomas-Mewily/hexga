#![allow(unused_imports, dead_code)]
use std::ops::*;
use hexga::prelude::*;

pub mod input;
use input::*;


#[derive(Debug, Default)]
pub struct Context
{
    input : Input,
}

pub trait App
{
    fn update(&mut self);
    fn draw(&mut self);
}