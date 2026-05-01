#![allow(dead_code)]
#![allow(unused)]
use hexga_engine::prelude::*;

struct MyApp {}

impl Application for MyApp
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx)
    {
        match &ev
        {
            AppEvent::Input(input_event) => {},
            AppEvent::Window(window_event) => return,
        }
        dbg!(ev);
    }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx) {
        
    }

    fn draw(&mut self, ctx: &mut AppCtx)
    {
        //Pen.
        //Pen.
        //println!("hello world");
    }
}

fn main()
{
    hexga_engine::run_with_param(|| MyApp {}, AppParam::new().with_title("hello world"));
}
