#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Deref;

use hexga_engine::prelude::*;

#[derive(Default)]
pub struct MyApp
{
    //camera: Camera3D,
    time: Time,
}


impl App for MyApp
{
    type UserEvent = ();

    fn update(&mut self, dt: DeltaTime) 
    {
        self.time += dt;
    }

    fn draw(&mut self)
    {
        info!("nb fps: {}", Perf.fps());

        Cam.rot_z(self.time.s().degree() * 50.);

        Pen.triangle(TriangleVertex::new
            (
                Vertex::new().with_position(vec3(-1.,-0.5,0.)).with_color(GpuColor::RED),
                Vertex::new().with_position(vec3(1.,-0.5,0.)).with_color(GpuColor::GREEN),
                Vertex::new().with_position(vec3(0.,1.,0.)).with_color(GpuColor::BLUE),
            )
        );
    }
}

fn main() 
{
    let x : i32 = 4;
    let y : f32 = x.cast_into();

    let x = [1,2i32];
    let y : [f32;2] = x.cast_into();

    let x = vector2(1,2i32);

    let i = 0;

    // let y : Vec2 = x.cast_into();

    //println!("Hello, world!");
    MyApp::___().run().unwrap();
    //println!("Goodbye, world!");
}
