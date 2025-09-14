#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Deref;
use hexga_engine::prelude::*;


pub struct MyApp
{
    dpad: BindingDpad,
    camera: Camera3D,
    time: Time,
}

impl Default for MyApp
{
    fn default() -> Self 
    {
        let binding = BindingDpad::load_from_disk_or_create("./binding.ron", || ___()).unwrap();
        //let binding = ___();
        Self 
        {
            dpad: binding, 
            time: ___(),
            camera: Camera3D
            { 
                position: (0.0, 1.0, 2.0).into(), 
                target: (0.0, 0.0, 0.0).into(), 
                up: Vec3::Y, 
                perspective: CameraPerspective
                {
                    aspect: 16. / 9.,
                    fovy: 45.degree(),
                    znear: 0.1,
                    zfar: 100.,
                }, 
                viewport: None 
            },
        }
    }
}

impl App for MyApp
{
    type UserEvent = ();

    fn update(&mut self, dt: DeltaTime) 
    {
        let speed = 10.0 * dt.s();
        let (camera, binding) = (&mut self.camera, &self.dpad);

        let forward = camera.target - camera.position;
        let forward_norm = forward.normalized();
        let forward_mag = forward.length();

        if binding.up.is_requested() && forward_mag > speed {
            camera.position += forward_norm * speed;
        }
        if binding.down.is_requested() 
        {
            camera.position -= forward_norm * speed;
        }

        let right = forward_norm.cross(camera.up);
        let forward = camera.target - camera.position;
        let forward_mag = forward.length();

        if binding.right.is_requested()
        {
            camera.position = camera.target - (forward + right * speed).normalized() * forward_mag;
        }
        if binding.left.is_requested()
        {
            camera.position = camera.target - (forward - right * speed).normalized() * forward_mag;
        }

        self.time += dt;
    }

    fn draw(&mut self)
    {
        let a : Vec2 = Ctx.window_size().to_vector();
        self.camera.perspective.aspect = a.x / a.y;
        Pen.set_matrix(self.camera.matrix());

        //Pen.rot_z(self.time.s().degree() * 50.);
        //Cam.rot_z(self.time.s().degree() * 50.);

        /* 
        Pen.triangle(TriangleVertex::new
            (
                Vertex::new().with_position(vec3(-1.,-0.5,0.)).with_color(GpuColor::RED),
                Vertex::new().with_position(vec3(1.,-0.5,0.)).with_color(GpuColor::GREEN),
                Vertex::new().with_position(vec3(0.,1.,0.)).with_color(GpuColor::BLUE),
            )
        );
        */

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
