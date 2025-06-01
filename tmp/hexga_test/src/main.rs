#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub use hexga_engine::prelude::*;


#[derive(Default)]
pub struct TestCtx
{
    time : Time,
}
impl MainLoop for TestCtx
{
    fn handle_event(&mut self, event : Event) -> bool {
        dbg!(event);
        false
    }

    fn update(&mut self) {

    }

    fn draw(&mut self) {

    }
}

fn main()
{
    for _ in 0..10 { println!(); }

    /*
    MultiMediaParam::new()
        .window(WindowParam::new().title("hello"))
        */
        //.run(|| TestCtx::___());
}

/*
fn main()
{
    for _ in 0..10 { println!(); }
}

        Pen.begin_draw();
        Pen.begin_pass();

        Pen.color(Color::RED).set_pos(vec2(-1., 1.)).down()
           .color(Color::BLUE).pos2(vec2(1., 1.)).down()
           .color(Color::GREEN).pos2(vec2(0.0, -1.)).down()
           .make_triangle();

        Pen.color(Color::PINK);

        //  for c in (0.0..1.0).sample(9. + (self.time.s() * 1.).cos() * 7.)
        for c in (Angle::ZERO..Angle::FULL).sample(40)
        {
            Pen.set_pos(c.to_vec2(0.5)).down();
        }
        Pen.make_convex_poly();


*/