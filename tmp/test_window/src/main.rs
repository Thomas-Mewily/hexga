#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use hexga_engine_window::app::{AppWindowLoop, AppWindowRun};
pub use hexga_engine_window::prelude::*;
pub use hexga_math::prelude::*;
pub use hexga_core::prelude::*;



#[derive(Default)]
pub struct MyApp
{
    time : Time,
}

impl AppWindowLoop for MyApp
{
    fn handle_event(&mut self, event : Event, ctx: &mut AppWindowContext) -> bool
    {
        if event.is_copy()
        {
            println!("COPY");
            dbg!(event);
        }
        if event.is_paste()
        {
            println!("PASTE");
        }
        //dbg!(event);
        true
    }

    fn update(&mut self, ctx: &mut AppWindowContext) {

    }

    fn draw(&mut self, ctx: &mut AppWindowContext) {

    }
}

fn main()
{
    for _ in 0..10 { println!(); }

    /*
    MultiMediaParam::new()
        .window(WindowParam::new().title("hello"))
        .run(|| TestCtx::___());
    */

    let mut app = MyApp::___();
    app.run().unwrap();
}