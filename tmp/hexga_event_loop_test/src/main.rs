#![allow(unused)]
use hexga_event_loop::prelude::*;

pub struct MyApp;

impl<'a> App<AppEvent, EventLoop<'a, AppEvent>> for MyApp
{
    fn event(&mut self, ev: AppEvent, ctx: &mut EventLoop<'a, AppEvent>) -> Option<AppEvent>
    {
        println!("{:?}", ev);
        None
    }
}

fn main() { MyApp::run_raw_with_param(|proxy| MyApp, Default::default()); }
