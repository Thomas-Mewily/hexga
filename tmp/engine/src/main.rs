#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

struct MyApp {}

impl Application for MyApp
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx)
    {
        dbg!(ev);
    }

    fn draw(&mut self, ctx: &mut AppCtx)
    {
        Pen.
        //Pen.
        println!("hello world");
    }
}

fn main()
{
    hexga_engine::run_with_param(|| MyApp {}, AppParam::new().with_title("hello world"));
}
