#![allow(unused)]
use hexga_event_loop::prelude::*;

pub struct MyApp;

impl App for MyApp
{
    type Ctx<'a> = EventLoop<'a>;

    fn event(&mut self, ev: AppEvent, ctx: &mut EventLoop) -> Option<AppEvent> {
        println!("{:?}", ev);
        None
    }
}

fn main()
{
    MyApp::run_raw_with_param(|proxy| MyApp, Default::default());
}
