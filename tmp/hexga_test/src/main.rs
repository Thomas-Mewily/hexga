#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub use quad_hexga_engine::prelude::*;


#[derive(Default)]
pub struct TestCtx
{
    time : Time,
}
impl LoopWindow for TestCtx
{
    fn window_update(&mut self) {
        self.time += (1./60.).s();
    }

    fn window_draw(&mut self) 
    {
        
        Pen.begin_draw();
        Pen.begin_pass();

        Pen.color(Color::RED).set_pos(vec2(-1., 1.)).down()
           .color(Color::BLUE).pos2(vec2(1., 1.)).down()
           .color(Color::GREEN).pos2(vec2(0.0, -1.)).down()
           .make_triangle();

        Pen.color(Color::PINK);
        for c in (Angle::ZERO..Angle::FULL).sample(40)
        {
            Pen.set_pos(c.to_vec2(0.5)).down();
        }
        Pen.make_convex_poly();

        /* 
        for angle in Angle::sample(3)
        {
            Pen.set_pos(angle.to_vec2(0.5)).down();
        }
        */
        
        /* 
        for c in (0.0..1.0).sample(9. + (self.time.s() * 1.).cos() * 7.)
        {
            Pen.set_pos(Angle::from_turn(c).to_vec2(0.5)).down();
        }

        for angle in Angle::sample(3)
        {
            Pen.set_pos(angle.to_vec2(0.5)).down();
        }
        Pen.make_convex_poly();
        */

        Pen.end_pass();
        Pen.end_draw();
    }









    
        //for c in (0.0..1.0).sample(128. + (self.time.s() * 1.).cos() * 7.)

        /* 
                Pen.color(Color::PINK);
        for c in (0.0..1.0).sample(9. + (self.time.s() * 1.).cos() * 7.)
        {
            Pen.set_pos(Angle::from_turn(c).to_vec2(0.5 + (c.turn() * 6.0).cos() * 0.2)).down();
        }
        */

        /* 
        Pen.color(Color::YELLOW);
        for angle in (..=Angle::FLAT).step(67.degree()) //.sample(8)
        {
            Pen.set_pos(angle.to_vec2(0.25)).down();
        }
        Pen.make_convex_poly();
        */

        //Pen.color(Color::YELLOW.lerp(Color::WHITE, 0.5));


        /* 
        Pen.color(Color::YELLOW);
        for angle in (..).sample(8)
        {
            Pen.set_pos(angle.to_vec2(0.25)).down();
        }
        Pen.make_convex_poly();
        */
        /* 
        // for i in Angle::sample(8)
        for angle in Angle.sample(8)
        {
            Pen.set_pos(angle.to_vec2(0.25)).down();
        }
        

        */
        


        /* 
        let Rect2 { pos, size } = Rect2::new_centered(zero(), 1.8.splat2());
        let pos = GpuVec2::from(pos).with_z(0.);
        let size = GpuVec2::from(size);
        let color = Color::WHITE;

        let vertex =
        [
            GpuVertex::new().with_pos(pos).with_color(color),
            GpuVertex::new().with_pos(pos.moved_x(size.x)).with_color(color),
            GpuVertex::new().with_pos(pos.moved_x(size.x).moved_y(size.y)).with_color(color),
            GpuVertex::new().with_pos(pos.moved_y(size.y)).with_color(color),
        ];

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        Pen.static_geometry(vertex, indices);
        Pen.draw_triangle_test();
        */



    fn window_handle_event(&mut self, event : Event) -> bool {
        println!("{:?}", event);
        true
    }
}

fn main() 
{
    for _ in 0..10 { println!(); }
    MultiMediaConfig::new()
        .with_window_config(WindowConfig::new().title("hello"))
        .run(|| TestCtx::___());
}

/* 
fn main() 
{
    for _ in 0..10 { println!(); }
}*/