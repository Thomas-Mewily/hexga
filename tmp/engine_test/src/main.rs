use hexga_engine::prelude::*;

pub struct MyApp;

impl<'a> App<(), AppCtx<'a>> for MyApp
{
    fn event(&mut self, ev: AppEvent, _ctx: &mut AppCtx) -> Option<AppEvent>
    {
        println!("{:?}", ev);
        None
    }

    fn draw(&mut self, _ctx: &mut AppCtx<'a>)
    {
        Pen.set_background_color(rgb(0.5, 0.5, 0.5));
        Pen.geometry(
            &[
                Vertex::new().with_position(vec3(0.0, 0.7, 0.0)).with_color(rgb(1.0, 0.0, 0.0)),
                Vertex::new().with_position(vec3(-0.7, -0.7, 0.0)).with_color(rgb(0.0, 1.0, 0.0)),
                Vertex::new().with_position(vec3(0.7, -0.7, 0.0)).with_color(rgb(0.0, 0.0, 1.0)),
            ],
            &[0, 1, 2],
        );
    }
}

fn main() { MyApp::run_with_param(|| MyApp, AppParam::default()).expect("failed to run triangle example"); }
