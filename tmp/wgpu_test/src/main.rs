#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Deref;

use hexga_engine::prelude::*;

pub struct DpadBinding
{
    up    : Binding,
    down  : Binding,
    left  : Binding,
    right : Binding
}
impl Default for DpadBinding
{
    fn default() -> Self 
    {
        Self 
        { 
            up: Binding::from(KeyCode::Up).or(KeyCode::W), 
            down: Binding::from(KeyCode::Down).or(KeyCode::S), 
            left: Binding::from(KeyCode::Left).or(KeyCode::A), 
            right: Binding::from(KeyCode::Right).or(KeyCode::D) 
        }
    }
}

#[derive(Default)]
pub struct MyApp
{
    dpad: DpadBinding,
    camera: Camera3D,
    time: Time,
}


impl App for MyApp
{
    type UserEvent = ();

    fn update(&mut self, dt: DeltaTime) 
    {
        let speed = 1.0;
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
        }else
        {
            camera.position = camera.target - (forward - right * speed).normalized() * forward_mag;
        }

        //dbg!(Input.keyboard.keys().down().to_vec());
        //let pressed = Input.keyboard.keys().pressed().to_vec();
        //let released = Input.keyboard.keys().released().to_vec();

        //if(pressed.is_not_empty()) { dbg!(pressed); }
        //if(released.is_not_empty()) { dbg!(released); }
        self.time += dt;
    }

    fn draw(&mut self)
    {
        //info!("nb fps: {}", Perf.fps());

        Cam.set_matrix(self.camera.matrix());

        //Cam.rot_z(self.time.s().degree() * 50.);

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
