#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
/*
use hexga_engine::prelude::*;

//
type Texture2DTmp = Asset<Texture>;

struct MyApp
{
    dpad: BindingDpad,
    nb_split: float,
    camera: Camera3D,
    time: Time,
    texture : Texture2DTmp,
}

impl Application for MyApp
{
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

        let nb_split_inc = 0.25;
        if KeyCode::N.is_pressed() { self.nb_split += nb_split_inc; }
        if KeyCode::B.is_pressed() { self.nb_split -= nb_split_inc; }

        self.time += dt;
    }

    fn draw(&mut self) {

        // if self.texture.is_none()
        // {
        //     let b2= include_bytes!("test.png");
        //     let img = Image::load_from_bytes(b2, "png").unwrap();
        //     self.texture = Some(Texture::from(img));
        // }
        /*
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
        )*/

        for r in Pen.viewport().split_x(self.nb_split)
        {
            Pen.set_viewport(r);
            self.camera.perspective.aspect = r.size.x / r.size.y;
            Pen.set_matrix(self.camera.matrix());

            Pen.texture_replace(Some(&self.texture));

            Pen.geometry
            (
                [
                    Vertex::new().with_position([-0.4, -0.4, 0.0].into()).with_color(GpuColor::RED).with_uv([0.0, 0.0].into()),
                    Vertex::new().with_position([0.4, -0.4, 0.0].into()).with_color(GpuColor::GREEN).with_uv([1.0, 0.0].into()),
                    Vertex::new().with_position([0.4, 0.4, 0.0].into()).with_color(GpuColor::BLUE).with_uv([1.0, 1.0].into()),
                    Vertex::new().with_position([-0.4, 0.4, 0.0].into()).with_color(GpuColor::YELLOW).with_uv([0.0, 1.0].into())
                ],
                [
                    0, 1, 2, 0, 2, 3
                ]
            );
        }
    }
}

fn main()
{
    println!("Hello, world!");

    let camera = Camera3D
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
        };
    let _ = MyApp { nb_split: 3., camera, time: zero(), dpad: ___(), texture: Texture2DTmp::load("./tmp/engine/src/test") }.run();

    println!("Goodbye");
}
*/

fn main() {}