#![allow(unused_imports)]
use hexga_engine::{prelude::*, AppRunParam};
use hexga_core::prelude::*;

struct App;



fn debug_millis() -> u32
{
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u32 % 1000
}

impl AppLoop for App
{
    fn update(&mut self, ctx: &mut AppCtx)
    {
        //println!("updated: {}", debug_millis());
    }

    fn draw(&mut self, ctx: &mut AppCtx)
    {
        println!("draw: {}", debug_millis());
        ctx.draw();
    }
}

fn main()
{
    App.run_with_param(
        AppRunParam::game().with_default_window(
            Some
            (
                WindowParam::___().with_title("Hello world").with_buttons(WindowButtons::all())
            )
        )).unwrap();
}
