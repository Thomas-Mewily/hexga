#![allow(unused)]
use hexga_event_loop::prelude::*;

pub struct MyApp;

impl App for MyApp
{
    fn event(&mut self, ev: AppEvent, event_loop: &mut EventLoop<()>) -> Option<AppEvent> {
        println!("{:?}", ev);
        None
    }
}

fn main()
{
    MyApp::run_with_param(|proxy| MyApp, Default::default());
}
