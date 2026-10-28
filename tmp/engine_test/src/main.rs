#![allow(unused)]
use hexga_engine::prelude::*;

pub struct MyApp;

impl<'a> App<(), AppCtx<'a>>  for MyApp
{
    fn message(&mut self, ev: AppMessage<()>, ctx: &mut AppCtx<'a>) -> Option<AppMessage<()>> {
        println!("{:?}", ev);
        None
    }
    
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx) -> Option<AppEvent> {
        println!("{:?}", ev);
        None
    }
}

fn main()
{
    MyApp::run_with_param(|| MyApp, AppParam::default());
}
