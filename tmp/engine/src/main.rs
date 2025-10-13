#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use hexga_engine::prelude::*;

struct MyApp;

impl Application for MyApp
{
    fn update(&mut self, dt: DeltaTime)
    {
        if KeyCode::Space.is_pressed()
        {

            //Window.set_x(960/2);
            dbg!(Clipboard.get());
            //Window.set_size(vec2i(64, 128));
            //Window.set_pos(vec2i(256, 100));
        }
    }

    fn draw(&mut self) {
        Pen.geometry
        (
            [
                Vertex::new().with_position([-0.0868241, 0.49240386, 0.0].into()).with_color(GpuColor::RED),
                Vertex::new().with_position([-0.49513406, 0.06958647, 0.0].into()).with_color(GpuColor::GREEN),
                Vertex::new().with_position([-0.21918549, -0.44939706, 0.0].into()).with_color(GpuColor::BLUE),
                Vertex::new().with_position([0.35966998, -0.3473291, 0.0].into()).with_color(GpuColor::BLUE),
                Vertex::new().with_position([0.44147372, 0.2347359, 0.0].into()).with_color(GpuColor::GREEN),
            ],
            [
                0, 1, 4, 1, 2, 4, 2, 3, 4
            ]
        )
    }
}

fn main()
{
    println!("Hello, world!");

    let _ = MyApp.run();

    println!("Goodbye");
}