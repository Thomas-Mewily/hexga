#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub use hexga_engine_window::prelude::*;
pub use hexga_math::prelude::*;
pub use hexga_core::prelude::*;


#[derive(Default)]
pub struct MyApp
{
    time : Time,
}

impl WindowLoop<()> for MyApp
{
    fn handle_event(&mut self, event : Event, ctx: &mut WindowCtx) -> bool
    {
        if event.is_copy()
        {
            println!("COPY");
        }
        if event.is_paste()
        {
            println!("PASTE");
        }
        if event.is_alt_f4()
        {
            println!("EXIT");
            ctx.exit();
        }
        //dbg!(event);
        true
    }

    fn update(&mut self, ctx: &mut WindowCtx) {

    }

    fn draw(&mut self, ctx: &mut WindowCtx) {

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